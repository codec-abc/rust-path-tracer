mod tracer;
mod bmp;

extern crate rand;

//rewritten from http://www.kevinbeason.com/smallpt/

#[allow(dead_code)]
fn main()
{
    let width = 300;
    let height = 200;
    let sample = 4;
    let image = tracer::compute(width, height, sample);
    let mut output_image = bmp::Image::new_image(width, height, bmp::PixelFormat::RGBA);

    for i in 0 .. width
    {
        for j in 0 .. height
        {
            let index = i + j * width;
            let vector = image[index as usize];
            //println!("pixel ({},{}) is ({},{},{})", i, j, vector.x, vector.y, vector.z);
            output_image.set_pixel(i, j, bmp::RGBAPixel
                {
                    r : (vector.x * 255.0) as u8,
                    g : (vector.y * 255.0) as u8,
                    b : (vector.z * 255.0) as u8,
                    a : 255});
        }
    }

    output_image.write_to_file("output_image.bmp")
}
