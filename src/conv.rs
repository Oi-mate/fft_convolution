use rustfft::num_complex::Complex;

pub fn convolution(v1: &Vec<Complex<f64>>, v2: &Vec<Complex<f64>>) -> Vec<Complex<f64>> { // TODO: mutable, parallel, different sizes?
    let mut res = Vec::new();
    //
    for i in 0 .. v1.iter().count() + v2.iter().count() - 1 {
        let mut acc = 0.0;
            for j in 0 .. v1.iter().count() {
                let cond = i as i32 - j as i32;
                if cond < v1.iter().count() as i32 && cond >= 0 {
                    acc += v1[cond as usize].re * v2[j].re;
                }
            }
        res.push(Complex::new(acc, 0.0));
    }
    return res;
}