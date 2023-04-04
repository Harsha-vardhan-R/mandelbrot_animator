use image::RgbImage;
use mandelbrot::no_of_bounces;
use num::complex::Complex;

fn main() {
    //Setting the dimensions of the out put photograph.
    //Just set the width it automatically takes in the 
    //height value by considering a 16:9 aspect ratio.
    let width = 3000;
    let height = width * ( 9 / 16);
    
    let mut imgbuffer = RgbImage::new(width, height);
      
    loop {

        'outer: for x in 0..width {
            for y in 0..height{
                //For adjusting the center and scale.
                let osrx = -1.2;
                let scale = 2.0;
                let osry = osrx * (height as f64 / width as f64);
                let cx = osrx - 0.5 + ((osrx.abs() * scale) * (x as f64 / width as f64));
                let cy = osry + ((osry.abs() * scale) * (y as f64 / height as f64));


                let c = Complex::new(cx, cy);
                let i = no_of_bounces(c, 255) as u8;

                let colourofpixel = image::Rgb([i * 0.3 as u8, i as u8,i * 0.2 as u8]);
                imgbuffer.put_pixel(x, y, colourofpixel);

                }
            
            }
            let _path = format!("havaqq{}.png",count);
            imgbuffer.save(_path).unwrap();
        
        
            break 'outer
    }
    
}
