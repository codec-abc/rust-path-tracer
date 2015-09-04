mod vector;
mod ray;
mod refl;
mod sphere;

use rand;
use rand::Rng;
use std::f64::consts::PI;
//use std::cell::RefCell;
use std::sync::Mutex;
use std::sync::Arc;
use scoped_threadpool::Pool;
use time;

struct Tracer
{
    spheres : Vec<sphere::Sphere>,
    w : u32,
    h : u32,
    samps : u32,
    //rng : Mutex<RefCell<rand::StdRng>>,
}

impl Tracer
{
    fn generate_random_float(&self) -> f64
    {
        /*
        let ref_cell_rng = self.rng.lock().unwrap();
        let nb = ref_cell_rng.borrow_mut().next_f64();
        return nb;
        */
        rand::thread_rng().gen::<f64>()
    }

    #[allow(dead_code)]
    fn radiance(&self, r_ : &ray::Ray) -> vector::Vector
    {
        let mut t : f64;
        let mut r = ray::Ray::new(r_.o, r_.d);
        let mut cl = vector::Vector::new_zero();
        let mut cf = vector::Vector::new(1.0, 1.0, 1.0);
        let mut depth = 0;
        let mut obj : &sphere::Sphere;
        loop
        {
            let intersect = self.intersect(&r);
            match intersect
            {
                None =>
                {
                    return vector::Vector::new_zero();
                }
                Some(_) =>
                {
                    t = intersect.unwrap().0;
                    obj = intersect.unwrap().1;
                }
            }

            let x = r.o + r.d*t;
            let n= (x-obj.p).norm();
            let nl = if n.dot(&r.d) < 0.0 { n } else { n*(-1.0) };
            let mut f = obj.c;

            let p : f64 = if f.x>f.y && f.x>f.z
            {
                f.x
            }
            else
            {
                if f.y > f.z
                {
                    f.y
                }
                else
                {
                    f.z
                }
            };
            cl = cl + cf.mult(&obj.e);
            depth = depth + 1;

            if depth > 5
            {
                if self.generate_random_float() < p
                {
                    f = f * (1.0/p);
                }
                else
                {
                    return cl;
                }
            }
            cf = cf.mult(&f);
            if obj.refl == refl::Refl::DIFF
            {
                let r1 : f64 = 2.0 * PI * self.generate_random_float();
                let r2 : f64 = self.generate_random_float();
                let r2s : f64 = r2.sqrt();

                let w = nl;
                let temp = if (w.x).abs() > 0.1 { vector::Vector::new(0.0, 1.0, 0.0) } else { vector::Vector::new(1.0, 0.0, 0.0) };
                let u = (temp % w).norm();
                let v = w % u;

                let d = (u*r1.cos()*r2s + v*r1.sin()*r2s + w*(1.0-r2).sqrt()).norm();
                r = ray::Ray::new(x,d);
                continue;

            }
            else if obj.refl == refl::Refl::SPEC
            {
                r = ray::Ray::new(x,r.d-n*2.0*n.dot(&r.d));
                continue;
            }

            let refl_ray = ray::Ray::new(x, r.d-n*2.0*n.dot(&r.d));
            let into = n.dot(&nl) > 0.0;
            let nc = 1.0;
            let nt = 1.5;
            let nnt = if into { nc/nt } else { nt/nc };
            let ddn = r.d.dot(&nl);
            let cos2t = 1.0 - nnt * nnt * (1.0-ddn*ddn);

            if cos2t < 0.0
            {
                r = refl_ray;
                continue;
            }

            let sign = if into { 1.0 } else { -1.0 };
            let tdir = (r.d * nnt - n * ( sign * (ddn * nnt + (cos2t.sqrt())))).norm();
            let a=nt-nc;
            let b=nt+nc;
            let r0= a*a / (b*b);
            let temp = if into { -ddn } else { tdir.dot(&n) };
            let c = 1.0 - temp;
            let re = r0+(1.0 - r0)*c*c*c*c*c;
            let tr = 1.0-re;
            let p = 0.25 + 0.5*re;
            let rp = re/p;
            let tp = tr/(1.0-p);
            if self.generate_random_float() < p
            {
                cf = cf*rp;
                r = refl_ray;
            }
            else
            {
                cf = cf*tp;
                r = ray::Ray::new(x,tdir);
            }
            continue;
        }
    }

    #[allow(unused_variables)]
    #[allow(dead_code)]
    pub fn compute(&self) -> Vec<vector::Vector>
    {
        println!("Running");
        let mut pool = Pool::new(16);
        let begin_time = time::now();

        let cam = ray::Ray::new(vector::Vector::new(50.0, 52.0, 295.6), vector::Vector::new(0.0,-0.042612,-1.0).norm());
        let cx = vector::Vector::new((self.w as f64) * 0.5135 / (self.h as f64), 0.0 , 0.0);
        let cy = (cx % cam.d).norm() * 0.5135;

        let mut c_ = vec![];
        for _ in 0 .. self.w
        {
            for _ in 0 .. self.h
            {
                c_.push(vector::Vector::new_zero());
            }
        }
        let c = Arc::new(Mutex::new(c_));
        let cam_arc = Arc::new(cam);
        let completion = Arc::new(Mutex::new(0.0));
        let old_completion = Arc::new(Mutex::new(0.0));
        pool.scoped(|scope|
        {
            for y in 0 .. self.h
            {
                for x in 0 .. self.w
                {
                    unsafe
                    {
                        let c_ref = c.clone();
                        let cam_copy = cam_arc.clone();
                        let completion_ref = completion.clone();
                        let old_completion_ref = old_completion.clone();
                        scope.execute(move ||
                        {
                            let i = (self.h - y -1) * self.w + x;
                            for sy in 0 .. 2
                            {
                                for sx in 0 .. 2
                                {

                                    let mut r = vector::Vector::new_zero();
                                    for _ in 0 .. self.samps
                                    {
                                        let r1 = 2.0 * self.generate_random_float();
                                        let dx = if r1 < 1.0 { r1.sqrt() - 1.0} else { 1.0 - (2.0 - r1).sqrt() };
                                        let r2 = 2.0 * self.generate_random_float();
                                        let dy = if r2 < 1.0 { r2.sqrt() - 1.0} else { 1.0 - (2.0 - r2).sqrt() };

                                        let r1 = 0.0;
                                        let dx = 0.0;
                                        let r2 = 0.0;
                                        let dy = 0.0;

                                        let sxf64 : f64 = sx as f64;
                                        let syf64 : f64 = sy as f64;

                                        let dxf64 : f64 = dx as f64;
                                        let dyf64 : f64 = dy as f64;

                                        let xf64 : f64 = x as f64;
                                        let yf64 : f64 = y as f64;

                                        let wf64 : f64 = self.w as f64;
                                        let hf64 : f64 = self.h as f64;

                                        let fx = ((sxf64 + 0.5 + dxf64)/2.0 + xf64)/wf64 - 0.5;
                                        let fy = ((syf64 + 0.5 + dyf64)/2.0 + yf64)/hf64 - 0.5;
                                        let d = cx * fx + cy * fy + cam_copy.d;
                                        r = r + self.radiance(&ray::Ray::new(cam_copy.o + d * 140.0, d.norm()))*(1.0/self.samps as f64);
                                    }
                                    let mut c2 = c_ref.lock().unwrap();
                                    let new_value = c2[i as usize] + vector::Vector::new(clamp(r.x),clamp(r.y),clamp(r.z))* 0.25;
                                    c2[i as usize] = new_value;
                                }
                            }
                            let ref mut completion = *(completion_ref.lock().unwrap());
                            let ref mut old_completion = *(old_completion_ref.lock().unwrap());
                            let completion_add = 1.0/((self.w * self.h) as f64);
                            if *completion - *old_completion > 0.1
                            {
                                *old_completion = *completion;
                                println!("Rendering {} spp, {1:.3} %", self.samps * 4, 100.0 *(*completion));
                            }
                            *completion = *completion + completion_add;
                        });
                    }
                }
            }
        });

        let w = c.lock().unwrap().clone();
        let end_time = time::now();
        let diff_time = end_time - begin_time;
        println!("Time taken to render picture {:02}h:{:02}m:{:02}s:{:04}ms" , diff_time.num_hours(), diff_time.num_minutes(), diff_time.num_seconds(), diff_time.num_milliseconds());
        return w;
    }

    #[allow(dead_code)]
    fn intersect<'a>(&'a self, r : &ray::Ray) -> Option< (f64, &'a sphere::Sphere) >
    {
        let n = self.spheres.len();
        let inf : f64 = 1e20;
        let mut t = inf;
        let mut sphere : Option<&sphere::Sphere> = None;
        for i in 0 .. n
        {
            let intersect_distance = self.spheres[i].intersect(&r);
            if intersect_distance != 0.0 && intersect_distance < t
            {
                t = intersect_distance;
                sphere = Some(&self.spheres[i]);
            }
        }
        if t < inf
        {
            Some((t, sphere.unwrap()))
        }
        else
        {
            None
        }
    }
}

#[allow(dead_code)]
fn clamp(x : f64) -> f64
{
    let result = if x < 0.0
    {
        0.0
    }
    else
    {
        if x > 1.0
        {
            1.0
        }
        else
        {
            x
        }
    };
    return result;
}

#[allow(dead_code)]
fn build_sphere() -> Vec<sphere::Sphere>
{
    let spheres = vec![
        sphere::Sphere::new(1e5,    vector::Vector::new(1e5+1.0,40.8,81.6),     vector::Vector::new(0.0, 0.0, 0.0),  vector::Vector::new(0.75,0.25,0.25),    refl::Refl::DIFF),//Left
        sphere::Sphere::new(1e5,    vector::Vector::new(-1e5+99.0,40.8,81.6),   vector::Vector::new(0.0, 0.0, 0.0),  vector::Vector::new(0.25,0.25,0.75),    refl::Refl::DIFF),//Rght
        sphere::Sphere::new(1e5,    vector::Vector::new(50.0,40.8, 1e5),        vector::Vector::new(0.0, 0.0, 0.0),  vector::Vector::new(0.75,0.75,0.75),    refl::Refl::DIFF),//Back
        sphere::Sphere::new(1e5,    vector::Vector::new(50.0,40.8,-1e5+170.0),  vector::Vector::new(0.0, 0.0, 0.0),  vector::Vector::new(0.0, 0.0, 0.0),     refl::Refl::DIFF),//Frnt
        sphere::Sphere::new(1e5,    vector::Vector::new(50.0, 1e5, 81.6),       vector::Vector::new(0.0, 0.0, 0.0),  vector::Vector::new(0.75,0.75,0.75),    refl::Refl::DIFF),//Botm
        sphere::Sphere::new(1e5,    vector::Vector::new(50.0,-1e5+81.6,81.6),   vector::Vector::new(0.0, 0.0, 0.0),  vector::Vector::new(0.75,0.75,0.75),    refl::Refl::DIFF),//Top
        sphere::Sphere::new(16.5,   vector::Vector::new(27.0,16.5,47.0),        vector::Vector::new(0.0, 0.0, 0.0),  vector::Vector::new(0.999,0.999,0.999), refl::Refl::SPEC),//Mirr
        sphere::Sphere::new(16.5,   vector::Vector::new(73.0,16.5,78.0),        vector::Vector::new(0.0, 0.0, 0.0),  vector::Vector::new(0.999,0.999,0.999), refl::Refl::REFR),//Glas
        sphere::Sphere::new(600.0,  vector::Vector::new(50.0,681.6-0.27,81.6),  vector::Vector::new(12.0,12.0,12.0), vector::Vector::new(0.0, 0.0, 0.0),     refl::Refl::DIFF) //Light
     ];
    return spheres;
}

pub fn generate_image(w : u32, h : u32, samps : u32) -> Vec<vector::Vector>
{
    let tr = Tracer
    {
        spheres : build_sphere(),
        w : w,
        h : h,
        samps : samps,
        //rng : Mutex::new(RefCell::new(rand::StdRng::new().unwrap())),
    };
    tr.compute()
}
