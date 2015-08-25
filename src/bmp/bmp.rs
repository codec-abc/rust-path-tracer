pub mod bmp
{
    use std;
    use std::fs::File;
    use std::io::prelude::*;
    use std::mem::transmute;

    pub struct Image
    {
        width : u32,
        height : u32,
        pixel_data : Vec<u8>,
        pixel_format : PixelFormat
    }

    #[derive(PartialEq)]
    pub enum PixelFormat
    {
        RGB,
        RGBA
    }

    pub struct RGBAPixel
    {
        pub r : u8,
        pub g : u8,
        pub b : u8,
        pub a : u8
    }

    fn get_padding(nb_padding_byte : u32) -> Vec<u8>
    {
        let mut padding : Vec<u8> = vec![];
        if nb_padding_byte == 1
        {
            padding = vec![0x00];
        }
        if nb_padding_byte == 2
        {
            padding = vec![0x00, 0x00];
        }
        if nb_padding_byte == 3
        {
            padding = vec![0x00, 0x00, 0x00];
        }
        return padding;
    }

    impl Image
    {
        pub fn new_image(width : u32, height : u32, format : PixelFormat) -> Image
        {
            let buffer_size : usize =
            match format
            {
                PixelFormat::RGBA =>
                {
                    (width * height * 4) as usize
                }
                PixelFormat::RGB =>
                {
                    (width * height * 3) as usize
                }
            };
            let vec = std::iter::repeat(0).take(buffer_size).collect::<Vec<u8>>();
            let img = Image
            {
                width : width,
                height : height,
                pixel_format : format,
                pixel_data :vec
            };
            return img;
        }

        pub fn get_pixel(& self, x : u32, y : u32) -> RGBAPixel
        {
            let bytes_per_pixel = match self.pixel_format
            {
                PixelFormat::RGBA => 4,
                PixelFormat::RGB => 3
            };

            let index = ((x + y * self.width) * bytes_per_pixel) as usize;
            let r = self.pixel_data[index];
            let g = self.pixel_data[index + 1];
            let b = self.pixel_data[index + 2];
            let a = match self.pixel_format
            {
                PixelFormat::RGB => 255,
                PixelFormat::RGBA =>
                {
                    self.pixel_data[index + 3]
                }
            };
            let pixel = RGBAPixel
            {
                r : r,
                g : g,
                b : b,
                a : a
            };
            return pixel;
        }

        pub fn set_pixel(&mut self, x : u32, y : u32, pixel : RGBAPixel)
        {
            let bytes_per_pixel = match self.pixel_format
            {
                PixelFormat::RGBA => 4,
                PixelFormat::RGB => 3
            };
            let index = ((x + y * self.width) * bytes_per_pixel) as usize;
            //println!("index is {}", index);
            self.pixel_data[index] = pixel.r;
            self.pixel_data[index + 1] = pixel.g;
            self.pixel_data[index + 2] = pixel.b;

            if bytes_per_pixel == 4
            {
                self.pixel_data[index + 3] = pixel.a;
            }
        }

        pub fn write_to_file(&self, path : &str)
        {
            let f = File::create(path);
            match f
            {
                Err(_) =>
                {
                    println!("Error when triying to create file");
                }
                Ok (mut file) =>
                {
                    let mut bytes_data : Vec<u8> = vec![];
                    let width = self.width;
                    let height = self.height;
                    let nb_bytes_per_pixel = match self.pixel_format
                    {
                        PixelFormat::RGBA => 4,
                        PixelFormat::RGB => 3
                    };

                    let nb_bytes : u32 = nb_bytes_per_pixel * width;

                    let nb_padding_byte : u32 = 4 - (nb_bytes % 4);
                    let padding = get_padding(nb_padding_byte);

                    let total_offset : u32 = 122;
                    let size_bitmap : u32 = width * height * nb_bytes_per_pixel + nb_padding_byte * height;
                    let size : u32 = size_bitmap + total_offset;

                    let mut bytes: [u8; 4];
                    let mut bytes_16: [u8; 2];
                    bytes = unsafe { transmute(size.to_le()) };

                    /* header */
                    bytes_data.extend(b"BM");
                    bytes_data.extend(&bytes);
                    bytes_data.extend(&vec![0xFF,0xFF]);
                    bytes_data.extend(&vec![0xFF,0xFF]);
                    bytes_data.extend(&vec![0x7A,0x00,0x00,0x00]);
                    /****/

                    /* dib header */
                    bytes_data.extend(&vec![0x6C, 0x00, 0x00, 0x00]);
                    bytes = unsafe { transmute(width.to_le()) };
                    bytes_data.extend(&bytes);
                    bytes = unsafe { transmute(height.to_le()) };
                    bytes_data.extend(&bytes);
                    bytes_data.extend(&vec![0x01, 0x00]);
                    let bits_per_pixel_on_16_bytes : u16 = (nb_bytes_per_pixel * 8) as u16;
                    bytes_16 = unsafe { transmute(bits_per_pixel_on_16_bytes.to_le()) };
                    bytes_data.extend(&bytes_16);
                    bytes_data.extend(&vec![0x03, 0x00, 0x00, 0x00]);
                    bytes = unsafe { transmute((size_bitmap).to_le()) };
                    bytes_data.extend(&bytes);
                    bytes_data.extend(&vec![0x13, 0x0B, 0x00, 0x00]);
                    bytes_data.extend(&vec![0x13, 0x0B, 0x00, 0x00]);

                    bytes_data.extend(&vec![0x00, 0x00, 0x00, 0x00]);
                    bytes_data.extend(&vec![0x00, 0x00, 0x00, 0x00]);

                    bytes_data.extend(&vec![0x00, 0x00, 0xFF, 0x00]);
                    bytes_data.extend(&vec![0x00, 0xFF, 0x00, 0x00]);
                    bytes_data.extend(&vec![0xFF, 0x00, 0x00, 0x00]);
                    bytes_data.extend(&vec![0x00, 0x00, 0x00, 0xFF]);
                    bytes_data.extend(&vec![0x20, 0x6E, 0x69, 0x57]);

                    bytes_data.extend(&vec![
                        0x00, 0x00, 0x00, 0x00,
                        0x00, 0x00, 0x00, 0x00,
                        0x00, 0x00, 0x00, 0x00,
                        0x00, 0x00, 0x00, 0x00,
                        0x00, 0x00, 0x00, 0x00,
                        0x00, 0x00, 0x00, 0x00,
                        0x00, 0x00, 0x00, 0x00,
                        0x00, 0x00, 0x00, 0x00,
                        0x00, 0x00, 0x00, 0x00]);

                    bytes_data.extend(&vec![0x00, 0x00, 0x00, 0x00]);
                    bytes_data.extend(&vec![0x00, 0x00, 0x00, 0x00]);
                    bytes_data.extend(&vec![0x00, 0x00, 0x00, 0x00]);

                    /****/

                    /* pixel data */

                    for j in 0 .. self.height
                    {
                        for i in 0 .. self.width
                        {
                            let u = i;
                            let v = self.height -1 - j;
                            let pixel = self.get_pixel(u,v);
                            let mut pixel_data : Vec<u8> = vec![];
                            pixel_data.push(pixel.b);
                            pixel_data.push(pixel.g);
                            pixel_data.push(pixel.r);
                            if self.pixel_format == PixelFormat::RGBA
                            {
                                pixel_data.push(pixel.a);
                            }
                            bytes_data.extend(pixel_data);
                        }
                        bytes_data.extend(&padding);
                    }
                    /****/

                    let _ = file.write_all(&bytes_data);
                }
            }
        }
    }
}
