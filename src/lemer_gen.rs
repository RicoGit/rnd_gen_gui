//! Генерация псевдослучайной последовательности методом Лемера

use std::time::SystemTime;

const A: usize = 25173;
const B: usize = 13849;
const C: usize = 65536;

/// Генератор псевдослучайных чисел
struct Rng {
    prev: usize
}

impl Rng {

    /// Создаем генератор псевдослучайных чисел для указанного начального значения
    pub fn new(seed: usize) -> Self {
        Rng {
            prev: seed
        }
    }

    /// Вернет случайное значение и обновляет внутреннее состояние генератора
    fn next(&mut self, max: usize) -> usize {
        // вычисляем следующее значение
        let next = (A * self.prev + B) % C;
        // обновляем внутренне состояние генератора
        self.prev = next;
        // уменьшаем число до заданного
        next % max
    }
}

/// Генерирует массив случайных чисел в диапазоне [1,99] указанного размера (number) методом Лемера
pub fn generate(size: usize) -> Vec<u32> {

    let seed = SystemTime::now().elapsed().unwrap().as_nanos() as usize;

    let mut rng = Rng::new(seed);

    let mut numbers = Vec::<u32>::with_capacity(size);

    for _ in 0..size {
        numbers.push(rng.next(100) as u32);
    }
    numbers
}