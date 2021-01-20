//! Встроенная в Rust генерация случайных чисел

use rand::Rng;

/// Генерирует массив случайных чисел в диапазоне [1,99] указанного размера (number)
pub fn generate(size: usize) -> Vec<u32> {
    let mut rng = rand::thread_rng();
    let mut numbers = Vec::<u32>::with_capacity(size);

    for _ in 0..size {
        numbers.push(rng.gen_range(1..100));
    }
    numbers
}
