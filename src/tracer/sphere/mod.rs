use ::tracer::vector;
use ::tracer::refl;
use ::tracer::ray;

#[derive(Copy, Clone)]
pub struct Sphere
{
  pub rad : f64,
  pub p : vector::Vector,
  pub e : vector::Vector,
  pub c : vector::Vector,
  pub refl : refl::Refl
}

impl Sphere
{
    #[allow(dead_code)]
    pub fn new
    (
    radius : f64,
    position : vector::Vector,
    emission : vector::Vector,
    color : vector::Vector,
    reflection : refl::Refl
    ) -> Sphere
    {
        Sphere
        {
            rad : radius,
            p : position,
            e : emission,
            c : color,
            refl : reflection
        }
    }

    #[allow(dead_code)]
    pub fn intersect(&self, r : &ray::Ray) -> f64
    {
        let op = self.p - r.o;
        let eps = 1e-4;
        let b = op.dot(&r.d);
        let mut det = b*b - op.dot(&op) + self.rad*self.rad;
        if det < 0.0
        {
            return 0.0;
        }
        else
        {
            det = det.sqrt();
            let mut t = b - det;
            if t > eps
            {
                return t;
            }
            else
            {
                t = b + det;
                if t > eps
                {
                    return t;
                }
                else
                {
                    return 0.0;
                }
            }
        }
    }
}
