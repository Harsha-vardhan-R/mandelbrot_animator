#![allow(non_snake_case)]


pub mod pixel_parameters {
    
    use num::{complex::Complex};
    /*
    This function gives out the count value, which is the number of bounces(iterations)
    it takes to get out of the threshold value(point of no return)(here it is 2)
    so if a input number has more bounces it means it is nearer to converging or it converges.
    Distintion takes the maximum number of bounces to look before it escapes.
    */
    pub fn no_of_bounces(input_complex: num::complex::Complex64, distinction: u32) -> u32 {

        let mut count = 0;
        let input_complex = Complex::new(input_complex.re, input_complex.im);
        let mut z = Complex::new(0.0, 0.0);

        while count <= distinction && z.norm() < 2.0  {
            //Can be changed for different fractals,but also need to change the center(fosx , fosy) at the same time.
            z = (z * z) + input_complex;
            count += 1;
        }

        count

    }
   
    ///Todo: try normalising the number of bounces, cause maybe the number of bounces is only giving a subset of the values it is meant to give out , so nearby pixels are taking the same colours.
    //Take in a single value return three values for r,g,b components to maintain a proper colour palatte.
    pub fn pixelcolour(para: u32 , distinction: u32) -> (u8 , u8 , u8) {
        //let us control the hue saturation value and then turn it into an rgb value.
        //hue: 0 -> 359 ..... value: 0.0 -> 1.0 ..... saturation: 0.0 -> 1.0.
        let b = (para) as f64 / distinction as f64;
        let hue = (b * 359.0).floor();
        //let saturation = 1.0;//b;////modified
        let saturation = 1.0;
        //let value = 1.0;// - b;/////modified
        let value = if para >= (distinction - 1) {//this is to get that black void where it never escapes . looks cool.
            0.0
        } else {
            1.0
        };

        let ( r , g , b ) = hsv_to_rgb(hue, saturation, value);

        ( r, g, b )
    }

    //Algorithm by me, but the idea is pretty standard.
    fn hsv_to_rgb(hue: f64, saturation: f64, value: f64) -> (u8, u8 , u8) {

        let rgb_range = value * saturation;
        let rgb_max = value;
        let rgb_min = rgb_max - rgb_range;
        //dividing the hsv cylinder into 6 parts , in each part we know the behaviour of R G B values will be constant.
        let partition: u8 = (hue / 60.0).floor() as u8;
        let remainder = (hue / 60.0) - partition as f64;
        //rgb range gets fixed once the saturation and value are fixed , so we multiply the remainder to get the value in that context
        //and add rgbmin as an offset.
        let upgradient = (rgb_range * remainder) + rgb_min;
        let downgradient = (rgb_range * ( 1.0 - remainder )) + rgb_min;

        let ( cr , cg , cb ) = match partition {

            0 => ( rgb_max , upgradient , rgb_min ),
            1 => ( downgradient , rgb_max , rgb_min ),
            2 => ( rgb_min , rgb_max , upgradient ),
            3 => ( rgb_min , downgradient , rgb_max ),
            4 => ( upgradient , rgb_min , rgb_max ),
            _ => ( rgb_max , rgb_min , downgradient ),

        };
        //return values(presently between [0.0 and 1.0]) multipling with 255 so that we magnify it for the respective rgb values.
        ( (cr * 255.0).floor() as u8 , (cg * 255.0).floor() as u8 , (cb * 255.0).floor() as u8)


    }
    

}



pub mod image_handlers {
    //still need to write.
    pub fn into_video(path : &str) {

    }

}

pub mod arith_max_min {
    pub fn give_max_and_min(in_vector : &Vec<Vec<u32>>) -> (u32 , u32) {
        let mut here: Vec<u32> = vec![];

        for x in 0..in_vector.len() {
            //assumin it atleast has one value inside the vectors
            for y in 0..in_vector[0].len() {
                here.push(in_vector[x][y]);
            }
        }


        (*here.iter().max().unwrap() , *here.iter().min().unwrap())
    }
}



#[cfg(test)]
mod tests{
    use image;

    use crate::pixel_parameters::pixelcolour;

    #[test]   
    
    fn some_rand_test() {//perfectly working
        let (w, h) = ( 256 , 256 );
        let mut image_buffer = image::RgbImage::new(w , h);

        
        for x in 0..255 {
            for y in 0..255 {
                //let value = 1.0;
                let (cr, cg, cb) = pixelcolour(x , 255);
                let colourofpixel = image::Rgb([cr , cg , cb]);
                image_buffer.put_pixel(x.into() , y, colourofpixel);
                
            }
        }

        image_buffer.save("dnno/image1.png").unwrap();

    }

}