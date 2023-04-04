use image::RgbImage;
use mandelbrot::{no_of_bounces, pixelcolour};
use num::complex::Complex;

fn main() {
    /*
    Setting the dimensions of the out put photograph.
    Set the width, it automatically takes in the 
    height value by considering a 16:9 aspect ratio.
    */
    let width = 1000;
    let height = (width as f32 * (9 as f32 / 16 as f32))as u32;
    
    let mut imgbuffer = RgbImage::new(width, height);
      
    loop {

        for x in 0..width {
            for y in 0..height{
                //For adjusting the center and scale.
                let osrx = -1.2;
                let scale = 2.0;
                let osry = osrx * (height as f64 / width as f64);
                let cx = osrx - 0.5 + ((osrx.abs() * scale) * (x as f64 / width as f64));
                let cy = osry + ((osry.abs() * scale) * (y as f64 / height as f64));


                let c = Complex::new(cx, cy);
                let i = no_of_bounces(c, 255);

                let (cr, cg, cb) = pixelcolour(i);
                let colourofpixel = image::Rgb([cr , cg , cb]);
                imgbuffer.put_pixel(x, y, colourofpixel);

            }
            
        }

        let _path = format!("Image.png");
        imgbuffer.save(_path).unwrap();
        
        break;
            
    }
    
}
