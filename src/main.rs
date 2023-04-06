#[allow(non_snake_case,unused_mut,unused_assignments)]
use image::RgbImage;
use mandelbrot_animator::{no_of_bounces, pixelcolour};
use num::complex::Complex;
//Images will be stored in "Rendered_images_here" folder.
fn main() {

    //1:1 Aspect ratio.
    let dimen = 1000;
    //which point you want to be zoomed in(The zoom in boundary point)--the given point is one of the interesting points.
    let (fixpnt_x, fixpnt_y) = (-0.743643887037158704752191506114774 , 0.131825904205311970493132056385139);
    let fosx = -1.8;
    let fosy = -1.15;
    let mut range = 2.3_f64;
    let mut pivot_point = (fosx , fosy);

    let mut frame = 1_u8;

    //loop for creating an animation by changing parameters little by little.
    loop {

        let mut imgbuffer = RgbImage::new(dimen, dimen);

        let gradient = range / (dimen as f64);

        for x in 0..dimen {

            for y in 0..dimen {

                //Nothing much to change here.
                let cx = pivot_point.0 + (gradient * (x as f64));
                let cy = pivot_point.1 + (gradient * (y as f64));

                let c = Complex::new(cx, cy);
                let i = no_of_bounces(c, 255);
                let (cr, cg, cb) = pixelcolour(i);
                let colourofpixel = image::Rgb([cr , cg , cb]);
                imgbuffer.put_pixel(x, y, colourofpixel);

            }
            
        }

        let path = format!("Rendered_images_here/Fractal resolution-{} frame-{}.png", dimen , frame);
        imgbuffer.save(path).unwrap();
        /*
        This controls the speed/smoothness of the animation,
        should be inbetween exclusively 0.0 and 1.0,
        nearer values to 1 make a smoother animation.
        */
        let k = 0.97;
        range = range * k;
        pivot_point.0 = fixpnt_x - (k * (fixpnt_x - pivot_point.0));
        pivot_point.1 = fixpnt_y - (k * (fixpnt_y - pivot_point.1));
        
        frame += 1;
        
        //break;//If you only want one frame.

    }
    
}
