use ::tracer::vector;

pub struct Ray
{
    pub o : vector::Vector,
    pub d : vector::Vector
}

impl Ray
{
    #[allow(dead_code)]
    pub fn new (origin : vector::Vector, direction : vector::Vector) -> Ray
    {
        Ray { o : origin, d : direction}
    }
}
