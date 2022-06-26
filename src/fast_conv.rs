use rustfft::{FftPlanner, num_complex::Complex};

pub fn fast_complex_multiply(n1: Complex<f64>, n2: Complex<f64>) -> Complex<f64> { // TODO: NOT WORKING
    // Karatsuba multiplication of complex numbers (3 multiplications required instead of 4)
    let s1 = n1.re * n2.re;
    let s2 = n1.im * n2.im;
    let s3 = (n1.re + n1.im) * (n2.re + n2.im);
    return Complex::new(s1 - s2, s3 - s1 - s2);
}

pub fn slow_complex_multiply(n1: Complex<f64>, n2: Complex<f64>) -> Complex<f64> {
    return Complex{
        re: n1.re * n2.re - n1.im * n2.im,
        im: n1.re * n2.im + n1.im * n2.re
    }
}

// TODO: different core and dry input sizes
// TODO: consider using the exact fft algorithm
pub fn fast_convolution(dry: &mut Vec<Complex<f64>>, core: &mut Vec<Complex<f64>>) -> Vec<Complex<f64>> {
    // fftplanner init
    let fftsize: usize = dry.iter().count() + core.iter().count() - 1;
    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(fftsize);

    // cloning vectors to transform
    let mut _dry = dry.clone();
    let mut _core = core.clone();
    _core.append(&mut vec![Complex{re: 0.0, im: 0.0}; dry.iter().count() - 1]);
    _dry.append(&mut vec![Complex{re: 0.0, im: 0.0}; core.iter().count() - 1]);


    fft.process(&mut _dry);
    fft.process(&mut _core);
    let mut res = Vec::new();
    
    for i in 0 .. _dry.iter().count() { 
        res.push(fast_complex_multiply(_dry[i], _core[i]))
    }

    let bfft = planner.plan_fft_inverse(fftsize);
    bfft.process(&mut res);

    // normalization of output
    for i in 0 .. res.iter().count() {
        res[i] = Complex{
            re: res[i].re / res.iter().count() as f64,
            im: res[i].im / res.iter().count() as f64
        }
    }
    return res;
}