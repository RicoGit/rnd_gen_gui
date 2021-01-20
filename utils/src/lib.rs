//! Модуль содержит вспомогательные функции для расчета стохастических величин

pub use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Stats {
    /// Размер выборки
    size: usize,
    /// Мат ожидание
    expectation: f32,
    /// Дисперсия
    dispersion: f32,
    /// Среднеквардатичное отклонение
    deviation: f32,
    /// Дифференциальной функция распределения
    density: Vec<f32>,
    /// Интегральаня функция распределения
    distribution: Vec<f32>,
}

/// Расчитывает стохастические величины: мат. ожидание, дисперсию и тп
pub fn stats(vec: &[u32]) -> Stats {
    let expectation = expectation(&vec);
    let dispersion = dispersion(&vec, expectation);
    let density = distribution_density(vec);
    Stats {
        size: vec.len(),
        expectation,
        dispersion,
        deviation: deviation(dispersion),
        density: Vec::from(density.clone()),
        distribution: Vec::from(distribution(&density)),
    }
}

/// Мат ожидание случайной величины
pub fn expectation(vec: &[u32]) -> f32 {
    vec.iter().sum::<u32>() as f32 / vec.len() as f32
}

/// Дисперсия случайной величины
fn dispersion(vec: &[u32], expectation: f32) -> f32 {
    vec.iter()
        .map(|x| {
            // случайная число - мат ожидание в квадрате деленное на размер выборки
            (*x as f32 - expectation).powi(2) / vec.len() as f32
        })
        .sum()
}

/// Среднеквардатичное отклонение случайной величины
fn deviation(dispersion: f32) -> f32 {
    dispersion.sqrt()
}

/// Дифференциальной функция распределения
fn distribution_density(vec: &[u32]) -> [f32; 100] {
    let mut result = [0.0; 100];
    // при встрече числа обновляем его вероятность на 1/100 и записываем в результирующий массив
    for num in vec {
        result[*num as usize] += 1.0 / vec.len() as f32;
    }

    result
}

/// Интегральная функция распределения
fn distribution(density: &[f32]) -> [f32; 100] {
    let mut result = [0.0; 100];
    let mut sum = 0.0;
    for (idx, prob) in density.iter().enumerate() {
        // сумма вероятности растет для каждого элемета
        sum += prob;
        result[idx] = sum;
    }

    result
}

/// Проверяем принадледит ли точка сектору
pub fn check((x, y): (f32, f32)) -> bool {
    let segment = (x * x + y * y).sqrt();
    segment <= 1.
}

/// Расчитываем какова вероятность попадания случайной величины в сектор
pub fn get_probability(x: &[f32], y: &[f32]) -> f32 {
    // Создаем из 2х массивов массив точек с координатами (x, y)
    let points = x.into_iter().zip(y.into_iter());
    // находим только точки лежащие в секторе
    let count = points.filter(|(x, y)| check((**x, **y))).count();

    // считаем вероятность
    count as f32 / x.len() as f32
}
