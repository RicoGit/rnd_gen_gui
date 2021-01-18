//! Модуль содержит вспомогательные функции для расчета стохастических величин

pub use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Stats {
    /// Размер выборки
    size: usize,
    /// Мат ожидание
    expectation: f32,
    /// Дисперсия
    dispersion: f32,
    /// Среднеквардатичное отклонение
    deviation: f32
}

/// Расчитывает стохастические величины: мат. ожидание, дисперсию и тп
pub fn stats(vec: &[u32]) -> Stats {
    let expectation = expectation(&vec);
    let dispersion = dispersion(&vec, expectation);
    Stats {
        size: vec.len(),
        expectation,
        dispersion,
        deviation: deviation(dispersion),
    }
}

/// Мат ожидание случайной величины
pub fn expectation(vec: &[u32]) -> f32 {
    vec.iter().sum::<u32>() as f32 / vec.len() as f32
}

/// Дисперсия случайной величины
fn dispersion(vec: &[u32], expectation: f32) -> f32 {
    vec.iter().map(|x| {
        (*x as f32 - expectation).powi(2) / vec.len() as f32
    }).sum()
}

/// Среднеквардатичное отклонение случайной величины
fn deviation(dispersion: f32) -> f32 {
    dispersion.sqrt()
}