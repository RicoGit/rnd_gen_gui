//! Встроенная в Rust генерация случайных чисел

use rand::Rng;

/// Генерирует массив случайных чисел в лиапазоне [1,99] указанного размера (number)
pub fn generate(size: usize) -> Vec<u8> {

    let mut rng = rand::thread_rng();
    let mut numbers = Vec::<u8>::with_capacity(size);

    for _ in 0..size {
        numbers.push(rng.gen_range(1..100));
    }
    numbers
}