#[allow(non_snake_case)]
use image::RgbImage;
use mandelbrot::{no_of_bounces, pixelcolour};
use num::complex::Complex;
//c = -0.75 + 0i
/* 
for a normal set.
let osx = 2.1_f64;
let osy = osx;
let fosx = -1.6;
let fosy = -1.05;
*/
fn main() {

    // Setting the dimensions of the output photograph.1:1 image, so two values are ot required.
    let dimen = 1081;
    let mut imgbuffer = RgbImage::new(dimen, dimen);
    //which point you want at the center--(The zoom in boundary point)
    let (_fixpnt_x, _fixpnt_y) = (0.0 , 0.0);

    let osx = 2.5_f64;
    let osy = osx;
    //Shifting the center because the image crate has (o,o) at it's top-left corer.
    let fosx = -1.8;
    let fosy = -1.15;

    //loop for creating an animation by changing parameters little by little.
    loop {

        let rngx = 2.3_f64;
        let rngy = rngx;

        for x in 0..dimen {

            for y in 0..dimen {

                let cx = fosx + (rngx * (x as f64 / dimen as f64));
                let cy = fosy + (rngy * (y as f64 / dimen as f64));

                //Nothing to do here.
                let c = Complex::new(cx, cy);
                let i = no_of_bounces(c, 255);
                let (cr, cg, cb) = pixelcolour(i);
                let colourofpixel = image::Rgb([cr , cg , cb]);
                imgbuffer.put_pixel(x, y, colourofpixel);

            }
            
        }

        let _path = format!("Rendered_images_here/Image {} {}.png", dimen , dimen);
        imgbuffer.save(_path).unwrap();
        
        break;
            
    }
    
}
