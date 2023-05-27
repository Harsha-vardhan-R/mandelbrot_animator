#![allow(non_snake_case,unused_mut,unused_assignments,unused_imports)]
use image::{RgbImage, Rgb};
use num::complex::Complex;
use mandelbrot_animator::{pixel_parameters::*, image_handlers::into_video, arith_max_min::give_max_and_min};
use std::{time, cmp::min};

fn main() {

    //#########can be changed, from here.##########
    //Images will be stored in "##" folder,do not forget to create one before rendering.
    let path_ = "new_folder";    
    let dimen = 1000_u32;//1:1 Aspect ratio.
    //which point you want to be zoomed in(The zoom in boundary point)--the given point is one of the interesting points.
    //(-0.743643887037158704752191506114774 , 0.131825904205311970493132056385139)
    let (fixpnt_x, fixpnt_y) = (-0.77568377, 0.13646737);    
    let number_of_frames = 100;
    let mut _max_iterations = 255_u32;//number of color bands in the image
    /*This controls the speed/smoothness of the animation,should be inbetween excluding 0.0 and 1.0,values nearer to 1 fetch a smoother animation.*/
    let k = 0.1;
    //##########Till here###########.

    //Initial fixed off-sets and range, Not to be messed with if you want a near full image at the start.
    let fosx = -1.8;
    let fosy = -1.15;
    let mut range = 2.3_f64;
    let mut pivot_point = (fosx , fosy);
    let mut frame = 1_u32;
    let mut store_bounces : Vec<Vec<u32>> = vec![ vec![0 ; dimen.try_into().unwrap()] ; dimen.try_into().unwrap()];
    let mut max_iterations = _max_iterations.clone();

    //loop for creating an animation, Need to stop it manually by pessing ctrl + c or cmd + c in the terminal.
    loop {

        let start_time = time::Instant::now();
        let mut imgbuffer = RgbImage::new(dimen, dimen);
        let gradient = range / (dimen as f64);

        //This loop is only to store the number of bounces for each pixel, So we will have a better control over the normalisation process.
        for x in 0..dimen {
            for y in 0..dimen {
                let cx = pivot_point.0 + (gradient * (x as f64));
                let cy = pivot_point.1 + (gradient * (y as f64));
                let c = Complex::new(cx, cy);
                store_bounces[x as usize][y as usize] = no_of_bounces(c, max_iterations);               
            }            
        }

        //normalising the number of bounces.
        let min_max = give_max_and_min(&store_bounces);
        for list in store_bounces.iter() {
            for mut number in list.iter() {
                number = &((((number  - min_max.1) * min_max.0) as f64 / (min_max.0 - min_max.1) as f64).round() as u32 + 1_u32);
            }
        }
        

        //In this loop we will take the normalised bounce values and apply them onto the image.
        for x in 0..dimen {
            for y in 0..dimen {
                let (cr, cg, cb) = pixelcolour(store_bounces[x as usize][y as usize] , min_max.0);
                imgbuffer.put_pixel(x, y, image::Rgb([cr , cg , cb]));
            }            
        }        

        let path = format!("{}/Fractal final_resolution-{} frame-{}.png",path_, dimen , frame);
        imgbuffer.save(path).unwrap();
        
        
        //Nothing much to change here.
        /////to improve , also check if there is any way to not get precision errors.
        //I do not have any idea even if i am getting them.   

        print!("{frame}  -> ");
        println!("{:?}" , min_max);
        println!("Zoom level : {}" , range);
        frame += 1;    
        //Improving the number of max bounces with frame , remember this also depends on the zoom value , so simply putting the frame in some formula will not work.
        max_iterations = _max_iterations + min_max.1;//we add the min value to original value every time , from debug we know that the min value increases so this should also increase.
        range = range * k;
        pivot_point.0 = fixpnt_x - (k * (fixpnt_x - pivot_point.0));
        pivot_point.1 = fixpnt_y - (k * (fixpnt_y - pivot_point.1));      
        
        //break;//uncomment, if you only want one frame.
        let end_time = time::Instant::now();
        let duration = end_time - start_time;
        //This debug is to know the amount of time taken to render the present frame.
        println!("{:?}\n",duration);

        if frame > number_of_frames {
            break;
        }

    }

    //Here we need to solve glitches?

    //This is for creating the video.it will save in the same folder as the images.
    /* println!("Images rendered!, now joining them to make a video\n");
    into_video(path_);
    println!("Video created, have a nice day!\n"); */
    
}
