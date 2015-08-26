mod vector;
mod ray;
mod refl;
mod sphere;

#[allow(dead_code)]
/*
fn radiance(r : &ray::Ray, depth : u32, xi : &mut u32) -> vector::Vector
{
    /Todo
}
*/

//http://www.kevinbeason.com/smallpt/

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

fn intersect(spheres : &Vec<sphere::Sphere>, r : &ray::Ray, t : &mut f64, id : &mut i32) -> bool
{
    let n = spheres.len();
    let inf : f64 = 1e20;
    *t = 1e20;
    for i in n .. 0
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
