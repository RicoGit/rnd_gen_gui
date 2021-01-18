//! Модуль содержит вспомогательные функции для расчета стохастических величин

pub use serde::{Serialize, Deserialize};
use std::collections::HashMap;

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
        distribution: Vec::from(distribution(&density))
    }
}

/// Мат ожидание случайной величины
pub fn expectation(vec: &[u32]) -> f32 {
    vec.iter().sum::<u32>() as f32 / vec.len() as f32
}

/// Дисперсия случайной величины
fn dispersion(vec: &[u32], expectation: f32) -> f32 {
    vec.iter().map(|x| {
        // случайная число - мат ожидание в квадрате деленное на размер выборки
        (*x as f32 - expectation).powi(2) / vec.len() as f32
    }).sum()
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
        result[*num as usize] += 0.01;
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
        result[idx] = sum;
    }

    result
}