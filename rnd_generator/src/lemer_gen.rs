//! Генерация псевдослучайной последовательности методом Лемера

use std::time::SystemTime;

const A: usize = 25173;
const B: usize = 13849;
const C: f32 = usize::MAX as f32;

/// Генератор псевдослучайных чисел
struct Rng {
    prev: usize,
}

impl Rng {
    /// Создаем генератор псевдослучайных чисел для указанного начального значения
    pub fn new(seed: usize) -> Self {
        Rng { prev: seed }
    }

    /// Вернет случайное значение и обновит внутреннее состояние генератора
    fn next(&mut self) -> f32 {
        // вычисляем следующее значение
        let next = A * self.prev + B;
        // обновляем внутренне состояние генератора
        self.prev = next;
        // уменьшаем число до (0: 1]
        next as f32 / C
    }

    /// Вернет случайное целочисленное значение
    fn next_int(&mut self, max: usize) -> usize {
        (self.next() * max as f32) as usize
    }
}

/// Генерирует массив случайных чисел в диапазоне max указанного размера (number) методом Лемера
pub fn generate_arr_int(size: usize, max: usize) -> Vec<u32> {
    let seed = SystemTime::now().elapsed().unwrap().as_nanos() as usize;

    let mut rng = Rng::new(seed);

    let mut numbers = Vec::<u32>::with_capacity(size);

    for _ in 0..size {
        numbers.push(rng.next_int(max) as u32);
    }
    numbers
}

/// Генерирует массив случайных чисел в диапазоне (0:1] указанного размера (number) методом Лемера
pub fn generate_arr(size: usize) -> Vec<f32> {
    let seed = SystemTime::now().elapsed().unwrap().as_nanos() as usize;

    let mut rng = Rng::new(seed);

    let mut numbers = Vec::<f32>::with_capacity(size);

    for _ in 0..size {
        numbers.push(rng.next());
    }
    numbers
}
