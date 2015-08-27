mod vector;
mod ray;
mod refl;
mod sphere;

use rand;
use rand::Rng;
use std::f64::consts::PI;

fn erand48(xi : &mut rand::StdRng) -> f64
{
    let random = xi.next_f64();
    //println!("random is {}" , random);
    return random;
}

#[allow(dead_code)]
fn radiance(spheres : &Vec<sphere::Sphere>, r : &ray::Ray, depth : &mut u32, xi : &mut rand::StdRng) -> vector::Vector
{
    let mut t : f64 = 0.0;
    let mut id : i32 = 0;
    if !intersect(spheres, r, &mut t, &mut id)
    {
        return vector::Vector::new_zero();
    }

    let obj = spheres[id as usize];

    let x = r.o+r.d*t;
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
    *depth = *depth + 1;

    if *depth > 5
    {
        if erand48(xi) < p
        {
            f = f * (1.0/p);
        }
        else
        {
            return obj.e;
        }
    }

    if obj.refl == refl::Refl::DIFF
    {
        let r1 : f64 = 2.0 * PI * erand48(xi);
        let r2 : f64 = erand48(xi);
        let r2s : f64 = r2.sqrt();

        let w = nl;
        let temp = if (w.x).abs() > 0.1 { vector::Vector::new(0.0, 1.0, 0.0) } else { vector::Vector::new(1.0, 0.0, 0.0) };
        let u = (temp % w).norm();
        let v = w % u;

        let d = (u*r1.cos()*r2s + v*r1.sin()*r2s + w*(1.0-r2).sqrt()).norm();
        return obj.e + f.mult(&radiance(spheres, &ray::Ray::new(x,d),depth,xi));

    }
    else if obj.refl == refl::Refl::SPEC
    {
        return obj.e + f.mult(&radiance(spheres, &ray::Ray::new(x,r.d-n*2.0*n.dot(&r.d)),depth,xi));
    }

    let refl_ray = ray::Ray::new(x, r.d-n*2.0*n.dot(&r.d));     // Ideal dielectric REFRACTION
    let into = n.dot(&nl) > 0.0;                // Ray from outside going in?
    let nc = 1.0;
    let nt = 1.5;
    let nnt = if into { nc/nt } else { nt/nc };
    let ddn = r.d.dot(&nl);
    let cos2t = 1.0 - nnt*nnt*(1.0-ddn*ddn);

    if cos2t < 0.0    // Total internal reflection
    {
        return obj.e + f.mult(&radiance(spheres, &refl_ray, depth, xi));
    }

    let sign = if into { 1.0 } else { -1.0 };
    let tdir = (r.d*nnt - n*(sign*(ddn*nnt+(cos2t.sqrt())))).norm();
    let a=nt-nc;
    let b=nt+nc;
    let r0=a*a/(b*b);
    let temp = if into { -ddn } else { tdir.dot(&n) };
    let c = 1.0 - temp;
    let re = r0+(1.0 - r0)*c*c*c*c*c;
    let tr = 1.0-re;
    let p = 0.25 + 0.5*re;
    let rp = re/p;
    let tp = tr/(1.0-p);
    let result = if *depth > 2
    {
        if erand48(xi)<p
        {
            radiance(spheres, &refl_ray,depth,xi)*rp
        }
        else
        {
            radiance(spheres, &ray::Ray::new(x,tdir),depth,xi)*tp
        }
    }
    else
    {
        radiance(spheres, &refl_ray,depth,xi)*re + radiance(spheres, &ray::Ray::new(x,tdir),depth,xi)*tr
    };
    return obj.e + f.mult(&result);
}

#[allow(dead_code)]
pub fn compute(w : u32, h : u32, samps : u32) -> Vec<vector::Vector>
{
    let cam = ray::Ray::new(vector::Vector::new(50.0, 52.0, 295.6), vector::Vector::new(0.0,-0.042612,-1.0).norm());
    let cx = vector::Vector::new((w as f64) * 0.5135 / (h as f64), 0.0 , 0.0);
    let cy = (cx%cam.d).norm() * 0.5135;
    let spheres = build_sphere();

    let mut c = vec![];
    for _ in 0 .. w
    {
        for _ in 0 .. h
        {
            c.push(vector::Vector::new_zero());
        }
    }
    for y in 0 .. h
    {
        println!("Rendering ({} spp) {}", samps*4, 100.0 * (y as f64)/(h as f64 - 1.0));
        let mut xi = rand::StdRng::new().unwrap();
        for x in 0 .. w
        {
            let i = (h-y-1)*w+x;
            for sy in 0 .. 2
            {
                for sx in 0 .. 2
                {
                    let mut r = vector::Vector::new_zero();
                    for _ in 0 .. samps //s
                    {
                        //println!("pixel is {} {}" , x, y);
                        let r1 = 2.0 * erand48(&mut xi);
                        let dx = if r1 < 1.0 { r1.sqrt() - 1.0} else { 1.0 - (2.0 - r1).sqrt() };
                        let r2 = 2.0 * erand48(&mut xi);
                        let dy = if r2 < 1.0 { r2.sqrt() - 1.0} else { 1.0 - (2.0 - r2).sqrt() };

                        let sxf64 : f64 = sx as f64;
                        let syf64 : f64 = sy as f64;

                        let dxf64 : f64 = dx as f64;
                        let dyf64 : f64 = dy as f64;

                        let xf64 : f64 = x as f64;
                        let yf64 : f64 = y as f64;

                        let wf64 : f64 = w as f64;
                        let hf64 : f64 = h as f64;

                        let fx = ((sxf64 + 0.5 + dxf64)/2.0 + xf64)/wf64 - 0.5;
                        let fy = ((syf64 + 0.5 + dyf64)/2.0 + yf64)/hf64 - 0.5;
                        let d = cx * fx + cy * fy + cam.d;
                        let mut depth : u32 = 0;
                        r = r + radiance(&spheres, &ray::Ray::new(cam.o+d*140.0,d.norm()),&mut depth,&mut xi)*(1.0/samps as f64);

                    }
                    //println!("r is {} {} {}", r.x, r.y, r.z);
                    c[i as usize] = c[i as usize] + vector::Vector::new(clamp(r.x),clamp(r.y),clamp(r.z))* 0.25;
                }
            }
        }
    }
    return c;
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
fn intersect(spheres : &Vec<sphere::Sphere>, r : &ray::Ray, t : &mut f64, id : &mut i32) -> bool
{
    let n = spheres.len();
    let inf : f64 = 1e20;
    *t = 1e20;
    for i in 0 .. n
    {
        let intersect_distance = spheres[i].intersect(&r);
        if intersect_distance != 0.0 && intersect_distance < *t
        {
            *t = intersect_distance;
            *id = i as i32;
        }
    }
    *t < inf
}

#[allow(dead_code)]
fn build_sphere() -> Vec<sphere::Sphere>
{
    //Scene: radius, position, emission, color, material
    let spheres = vec![
        sphere::Sphere::new(1e5, vector::Vector::new(1e5+1.0,40.8,81.6), vector::Vector::new(0.0, 0.0, 0.0), vector::Vector::new(0.75,0.25,0.25),   refl::Refl::DIFF),//Left
        sphere::Sphere::new(1e5, vector::Vector::new(-1e5+99.0,40.8,81.6),vector::Vector::new(0.0, 0.0, 0.0), vector::Vector::new(0.25,0.25,0.75),   refl::Refl::DIFF),//Rght
        sphere::Sphere::new(1e5, vector::Vector::new(50.0,40.8, 1e5),     vector::Vector::new(0.0, 0.0, 0.0), vector::Vector::new(0.75,0.75,0.75),   refl::Refl::DIFF),//Back
        sphere::Sphere::new(1e5, vector::Vector::new(50.0,40.8,-1e5+170.0), vector::Vector::new(0.0, 0.0, 0.0), vector::Vector::new(0.0, 0.0, 0.0), refl::Refl::DIFF),//Frnt
        sphere::Sphere::new(1e5, vector::Vector::new(50.0, 1e5, 81.6),    vector::Vector::new(0.0, 0.0, 0.0), vector::Vector::new(0.75,0.75,0.75),   refl::Refl::DIFF),//Botm
        sphere::Sphere::new(1e5, vector::Vector::new(50.0,-1e5+81.6,81.6),vector::Vector::new(0.0, 0.0, 0.0), vector::Vector::new(0.75,0.75,0.75),   refl::Refl::DIFF),//Top
        sphere::Sphere::new(16.5,vector::Vector::new(27.0,16.5,47.0),       vector::Vector::new(0.0, 0.0, 0.0), vector::Vector::new(0.999,0.999,0.999),    refl::Refl::SPEC),//Mirr
        sphere::Sphere::new(16.5,vector::Vector::new(73.0,16.5,78.0),       vector::Vector::new(0.0, 0.0, 0.0), vector::Vector::new(0.999,0.999,0.999),    refl::Refl::REFR),//Glas
        sphere::Sphere::new(600.0, vector::Vector::new(50.0,681.6-0.27,81.6),vector::Vector::new(12.0,12.0,12.0),      vector::Vector::new(0.0, 0.0, 0.0), refl::Refl::DIFF) //Lite
     ];
    return spheres;
}
