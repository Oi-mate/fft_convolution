use std::f64::consts::PI;
use rustfft::num_complex::Complex;

const DEFAULT_SAMPLING_FREQ: i32 = 44000;
const FFT_EFFECTIVE_SIZE: i32 = 128; // 2 ^ 6 * 3 ^ 4
const FFT_UNEFFECTIVE_SIZE: i32 = 5183;
const FFT_SIZE_5184: i32 = 5184; // 2 ^ 6 * 3 ^ 4
const FFT_SIZE_216: i32 = 216; // 2^3 * 3^3
const FFT_SIZE_1296: i32 = 1296; // 2^4 * 3^4
const TEST_SIZE: i32 = 16;
const SAMPLE_LENGTH: f64 = 1.0 / DEFAULT_SAMPLING_FREQ as f64;


fn sin_value(freq: i32, ind: i32, amp: f64, shift: f64) -> f64 {
    (2.0 * PI * (freq as f64) * (ind as f64) * SAMPLE_LENGTH + shift).cos() * amp
}

fn generate_mock_buffer(amp: f64, freq: i32, shift: f64, size: i32) -> Vec<Complex<f64>> {
    let mut buffer = Vec::new();
    for i in 1..size + 1 {
        buffer.push(Complex{
            re: sin_value(freq, i, amp, shift),
            im: 0.0
        });
    }
    return buffer;
}