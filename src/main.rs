#[allow(non_snake_case,unused_mut,unused_assignments)]
use image::RgbImage;
use num::complex::Complex;
use mandelbrot_animator::pixel_parameters::*;
use std::time;


fn main() -> ! {

    //Images will be stored in "Rendered_images_here" folder.
    let path = "Rendered_images_here";
    //1:1 Aspect ratio.
    let dimen = 1000;
    //which point you want to be zoomed in(The zoom in boundary point)--the given point is one of the interesting points.
    let (fixpnt_x, fixpnt_y) = (-0.743643887037158704752191506114774 , 0.131825904205311970493132056385139);
    //Initial fixed off-sets and range, Not to be disturbed if you want a full image at the start of the animation.
    let fosx = -1.8;
    let fosy = -1.15;
    let mut range = 2.3_f64;
    let mut pivot_point = (fosx , fosy);
    let distinction = 255_u16;
    
    let mut frame = 1_u16;

    //loop for creating an animation, Need to stop it manually by pessing ctrl + c or cmd + c in the terminal.
    loop {

        let start_time = time::Instant::now();
        let mut imgbuffer = RgbImage::new(dimen, dimen);
        let gradient = range / (dimen as f64);

        for x in 0..dimen {

            for y in 0..dimen {

                //Nothing much to change here.
                let cx = pivot_point.0 + (gradient * (x as f64));
                let cy = pivot_point.1 + (gradient * (y as f64));
                let c = Complex::new(cx, cy);
                let i = no_of_bounces(c, distinction);
                let (cr, cg, cb) = pixelcolour(i);
                let colourofpixel = image::Rgb([cr , cg , cb]);
                imgbuffer.put_pixel(x, y, colourofpixel);

            }
            
        }

        let path = format!("{}/Fractal resolution-{} frame-{}.png",path, dimen , frame);
        imgbuffer.save(path).unwrap();
        /*
        This controls the speed/smoothness of the animation,should be inbetween exclusively 0.0 and 1.0,
        nearer values to 1 make a smoother animation.
        */
        let k = 0.99;
        range = range * k;
        pivot_point.0 = fixpnt_x - (k * (fixpnt_x - pivot_point.0));
        pivot_point.1 = fixpnt_y - (k * (fixpnt_y - pivot_point.1));
        
        frame += 1;
        
        //break;//If you only want one frame.
        let end_time = time::Instant::now();
        let duration = end_time - start_time;
        //This debug is to know the amount of time taken to render the present frame;
        dbg!(duration);

    }
    
}
