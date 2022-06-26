mod tests {
    #[cfg(test)]
    use rustfft::num_complex::Complex;
    #[cfg(test)]
    use crate::fast_conv::{fast_convolution, fast_complex_multiply, slow_complex_multiply};
    #[cfg(test)]
    use crate::conv::convolution;
    #[cfg(test)]
    use float_cmp::approx_eq;

    #[test]
    fn fast_complex_multiplication_works() {
        let z = Complex::new(2.0,3.0);
        let y = Complex::new(4.0,7.0);
        assert_eq!(fast_complex_multiply(z, y), slow_complex_multiply(z, y))
    }
    
    #[test]
    fn convs_are_equal() {
        let mut x = vec![Complex{
            re: 1.0,
            im: 0.0
        },Complex{
            re: 2.0,
            im: 0.0
        },Complex{
            re: 3.0,
            im: 0.0
        },Complex{
            re: 4.0,
            im: 0.0
        },Complex{
            re: 5.0,
            im: 0.0
        }];
        let mut y = vec![Complex{
            re: 6.0,
            im: 0.0
        },Complex{
            re: 2.0,
            im: 0.0
        },Complex{
            re: 1.0,
            im: 0.0
        },Complex{
            re: 3.0,
            im: 0.0
        },Complex{
            re: 0.0,
            im: 0.0
        }];
        let mut c = y.clone();
        let mut d = x.clone();
        let z = fast_convolution(&mut x, &mut y);
        let q = convolution(&mut c, &mut d);
        assert_eq!(z.iter().count(), q.iter().count());
        for i in 0 .. z.iter().count() {
            assert!(approx_eq!(f64, z[i].re, q[i].re, epsilon = 0.00000001));
            assert!(approx_eq!(f64, z[i].im, q[i].im, epsilon = 0.00000001));
        }
    }

    
}