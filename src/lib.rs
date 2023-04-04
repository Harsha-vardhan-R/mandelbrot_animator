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
        z = (z * z) + input_complex;
        count += 1;
    }

    count

}


/*
The work of this function is to take in a single value
turn the value into r,g,b components to maintain a
proper colour palatte.Editing this function will 
give different colour palette.
*/
pub fn pixelcolour(Para: u16) -> (u8 , u8 , u8) {
    let r = Para as u8;
    let g = Para as u8;
    let b = Para as u8;

    (r , g , b)

}
