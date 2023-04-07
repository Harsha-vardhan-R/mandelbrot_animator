#[allow(non_snake_case)]


pub mod pixel_parameters {

    use num::complex::Complex;
    /*
    This function gives out the count value, which is the number of bounces(iterations) 
    it takes to get out of the threshold vlue(point of no return)(here it is 2)
    so if a input number has more bounces it means it is nearer to converging or it converges. 
    Distintion takes the maximum number of bounces to look before it escapes.
    */
    pub fn no_of_bounces(input_complex: num::complex::Complex64,distinction: u16) -> u16 {

        let mut count = 0;
        let mut z = Complex::new(0.0, 0.0);

        while count <= distinction && z.norm() < 2.0  {
            //Can be changed for different fractals,but also need to change the center at the same time.
            z = (z * z) + input_complex;
            count += 1;
        }

        count

    }

    
    //Take in a single value return three values for r,g,bcomponents to maintain a proper colour palatte.
    pub fn pixelcolour(para: u16) -> (u8 , u8 , u8) {

        let para = para as u8;
        let r = para;
        let g = para;
        let b = para;

        (r , g , b)

    }

}
