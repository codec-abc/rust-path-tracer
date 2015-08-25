mod bmp;

fn main()
{
    let mut img_alpha = bmp::Image::new_image(4, 2, bmp::PixelFormat::RGBA);
    img_alpha.set_pixel(0, 0, bmp::RGBAPixel {r:253,g:0,b:0,a:255});
    img_alpha.set_pixel(1, 0, bmp::RGBAPixel {r:253,g:254,b:255,a:255});
    img_alpha.set_pixel(2, 0, bmp::RGBAPixel {r:0,g:254,b:255,a:255});
    img_alpha.set_pixel(3, 0, bmp::RGBAPixel {r:253,g:0,b:0,a:255});

    img_alpha.set_pixel(0, 1, bmp::RGBAPixel {r:253,g:0,b:0,a:255});
    img_alpha.set_pixel(1, 1, bmp::RGBAPixel {r:253,g:0,b:0,a:157});
    img_alpha.set_pixel(2, 1, bmp::RGBAPixel {r:253,g:0,b:0,a:22});
    img_alpha.set_pixel(3, 1, bmp::RGBAPixel {r:253,g:0,b:0,a:0});
    img_alpha.write_to_file("test_alpha.bmp");

    let mut img = bmp::Image::new_image(4, 2, bmp::PixelFormat::RGB);
    img.set_pixel(0, 0, bmp::RGBAPixel {r:253,g:0,b:0,a:255});
    img.set_pixel(1, 0, bmp::RGBAPixel {r:253,g:254,b:255,a:255});
    img.set_pixel(2, 0, bmp::RGBAPixel {r:0,g:254,b:255,a:255});
    img.set_pixel(3, 0, bmp::RGBAPixel {r:253,g:0,b:0,a:255});

    img.set_pixel(0, 1, bmp::RGBAPixel {r:253,g:0,b:0,a:255});
    img.set_pixel(1, 1, bmp::RGBAPixel {r:253,g:0,b:0,a:255});
    img.set_pixel(2, 1, bmp::RGBAPixel {r:253,g:0,b:0,a:255});
    img.set_pixel(3, 1, bmp::RGBAPixel {r:253,g:0,b:0,a:255});
    img.write_to_file("test.bmp");

}
