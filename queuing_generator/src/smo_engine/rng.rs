//! Модуль с функциями генерации случайных чисел

use rand::Rng;

/// Генерирует случайно число с нормальным распределением
pub fn next(expectation: usize, dispersion: usize) -> f32 {
    let mut rng = rand::thread_rng();
    let mut sum = 0.;

    for _ in 0..6 {
        sum += rng.gen::<f32>();
    }

    dispersion as f32 * (sum - 3 as f32) + expectation as f32
}

/// Генерирует целое случайно число с нормальным распределением
pub fn next_int(expectation: usize, dispersion: usize) -> usize {
    let res = next(expectation, dispersion) as usize;
    println!("m: {}, σ:{}, res: {}", expectation, dispersion, res);
    res
}

/// Генерирует булевое значение с нормальный распределением
pub fn next_bool(probability: f64) -> bool {
    rand::thread_rng().gen_bool(probability)
}
