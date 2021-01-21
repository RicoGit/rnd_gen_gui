use crate::smo_engine::rng;
use crate::smo_engine::rng::next_int;
pub use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

/// Настройки для нормального распределения
#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub struct Distribution {
    /// Матожидание для интервала появления
    pub expectation_time: usize,
    /// Дисперсия для интервала появления
    pub dispersion_time: usize,
}

/// Заданные пользователем настройки системы
#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub struct Options {
    /// Распределение для интервала появления
    pub appearance_time: Distribution,
    /// Распределения для длительности задачи
    pub task_weight_time: Distribution,
    /// Вероятность получении задачей низкого приоритета [0, 1]
    pub low_priority_probability: f64,
    /// Скольким миллисекундам равен один раунд
    pub time_scale_millis: u64,
    /// Количество циклов эмуляции
    pub max_number_of_rounds: usize,
}

#[derive(Debug)]
pub struct Task {
    /// Время прибытия (создания)
    pub incoming_time: usize,
    /// Требуемое время обслуживания
    pub require_time: usize,
    /// Приоритет
    pub low_priority: bool,
}

impl Task {
    /// Вероятностное создание новой задачи
    /// Генерация новой задача изходя из пройденного время, вероятности генерации и нормального закона распределения
    pub fn try_new(now: usize, elapsed: usize, options: Options) -> Option<Self> {
        // получаем случайно время появления новой задачи
        let rand_appearance = next_int(
            options.appearance_time.expectation_time,
            options.appearance_time.dispersion_time,
        );

        // если это случайное время < прошедшего с последней генерации задачи - создаем новую, иначе - ничего
        if rand_appearance < elapsed {
            Some(Task::new(now, options))
        } else {
            None
        }
    }

    /// Сощдание новой задачи
    pub fn new(time: usize, options: Options) -> Self {
        Task {
            incoming_time: time,
            require_time: rng::next_int(
                options.task_weight_time.expectation_time,
                options.task_weight_time.dispersion_time,
            ),
            low_priority: rng::next_bool(options.low_priority_probability),
        }
    }
}

/// Внутренне состояние системы
#[derive(Debug)]
pub struct State {
    /// Если true эмуляция запущена
    pub started: bool,

    /// Текущий момент времени (модельного времени)
    pub now: usize,

    /// Сколько времени осталось выполнять текущую задачу (если 0 - нет запущенных задач)
    pub rest_time_working: usize,

    /// Очередь для задач с нормальным приоритетом (FIFO)
    pub queue: VecDeque<Task>,

    /// Очередь задач с нижким приоритетом (LIFO)
    pub low_prior_queue: Vec<Task>,

    /// Всего задач выполнено
    pub task_done_total: usize,

    /// Всего низкоприоритетных задач выполнено
    pub low_prior_task_done_total: usize,

    /// Сумма интервалов появления всех задач (для расчета среднего интервала между задачами)
    pub time_between_tasks_total: usize,
}

impl State {
    /// Новое пустоя состояние
    pub fn new() -> Self {
        State {
            started: false,
            now: 0,
            rest_time_working: 0,
            queue: Default::default(),
            low_prior_queue: Default::default(),
            task_done_total: 0,
            low_prior_task_done_total: 0,
            time_between_tasks_total: 0,
        }
    }

    /// Считает статистику для текущего состояния системы, эта статистика отправляеться в полльзовательский интерфейс
    pub fn get_stats(self) -> Stats {
        todo!()
    }
}

/// Статистика системы
///
/// В среднем на задачу: ``` now / task_done_total ```
///
pub struct Stats {
    // /// Текущий момент времени (модельного времени)
// now: usize,

// /// Всего задач выполнено
// task_done_total: usize,
//
// /// Сумма интервалов появления всех задач (для расчета среднего интервала между задачами)
// time_between_tasks_total: usize
}
