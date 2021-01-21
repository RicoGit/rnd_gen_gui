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

#[derive(Serialize, Deserialize, Clone, Debug)]
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
        if dbg!(rand_appearance) <= elapsed {
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
    pub rest_time_working: u32,

    /// Очередь для задач с нормальным приоритетом (FIFO)
    pub queue: VecDeque<Task>,

    /// Очередь задач с нижким приоритетом (LIFO)
    pub low_prior_queue: Vec<Task>,

    /// Запущенная задача
    pub task: Option<Task>,

    // Аккумуляторы

    /// Всего задач выполнено
    pub task_done_total: usize,

    /// Всего низкоприоритетных задач выполнено
    pub low_prior_task_done_total: usize,

    /// Общее время ожидания завершенных задач
    pub task_wait_time_total: usize,
    /// Общее время ожидания завершенных низкоприоритетных задач
    pub low_prior_task_wait_time_total: usize,

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
            task_wait_time_total: 0,
            low_prior_task_wait_time_total: 0,
            task: None,
        }
    }

    /// Считает статистику для текущего состояния системы, эта статистика отправляеться в полльзовательский интерфейс
    pub fn get_stats(&self) -> Stats {
        let task_time_spent = self
            .task
            .as_ref()
            .map(|t| {
                self.now - t.incoming_time - t.require_time + self.rest_time_working as usize
            })
            .unwrap_or(0);

        // суммарное время ожидания всех нормальных задач в очереди
        let total_wait_time_in_q: usize = self.queue.iter().map(|t| self.now - t.incoming_time).sum();
        // суммарное время ожидания всех низуоприоритетных задач в очереди
        let total_wait_time_in_low_prior_q: usize = self.low_prior_queue.iter().map(|t| self.now - t.incoming_time).sum();

        // суммарное время ожидания всех задач завершенных и в очереди
        let total_wait_time = self.task_wait_time_total + total_wait_time_in_q + total_wait_time_in_low_prior_q;
        // суммарное время ожидания всех низуоприоритетных задач завершенных и в очереди
        let total_wait_time_low_prior = self.low_prior_task_wait_time_total + total_wait_time_in_low_prior_q;

        // количество всех задач законченый и в очередях
        let total_task = self.task_done_total + self.queue.len() + self.low_prior_queue.len();
        // количество всех низкоприоритетных задач законченных и в очередях
        let total_low_task = self.low_prior_task_done_total + self.low_prior_queue.len();
        // количество всех обычных задач законченных и в очередях
        let total_norm_task = total_task - total_low_task;
        Stats {
            now: self.now,
            task: self.task.clone(),
            task_time_spent,
            rest_time_working: self.rest_time_working,
            task_done_total: self.task_done_total,
            low_prior_task_done_total: self.low_prior_task_done_total,
            task_in_q_total: self.queue.len() + self.low_prior_queue.len(),
            low_prior_task_in_q_total: self.low_prior_queue.len(),

            avg_task_wait_time: total_wait_time as f32 / total_task as f32,
            low_prior_avg_task_wait_time: total_wait_time_low_prior as f32 / total_low_task as f32,
            normal_prior_avg_task_wait_time: (total_wait_time - total_wait_time_low_prior) as f32 / total_norm_task as f32,

            avg_time_between_tasks: self.now as f32 / total_task as f32,
            avg_time_between_low_prior_tasks: self.now as f32 / total_low_task as f32,
            avg_time_between_normal_prior_tasks: self.now as f32 / total_norm_task as f32
        }
    }
}

/// Статистика системы
///
/// В среднем на задачу: ``` now / task_done_total ```
///
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Stats {
    /// Текущий момент времени (модельного времени)
    now: usize,

    /// Информация о текущей задаче
    task: Option<Task>,

    /// Сколько прождал в очереди
    task_time_spent: usize,

    /// Осталось обработывать текущую задачу
    rest_time_working: u32,

    /// Всего задач выполнено
    pub task_done_total: usize,
    /// Всего задач низкого приоритета выполнено
    pub low_prior_task_done_total: usize,

    /// Всего задач в очереди
    pub task_in_q_total: usize,
    /// Всего низкоприоритетных задач в очереди
    pub low_prior_task_in_q_total: usize,

    /// Среднее время ожидания
    pub avg_task_wait_time: f32,
    /// Среднее время ожидания низкоприоритетных задач
    pub low_prior_avg_task_wait_time: f32,
    /// Среднее время ожидания обычных задач
    pub normal_prior_avg_task_wait_time: f32,

    /// Среднее время между появления всех задач
    pub avg_time_between_tasks: f32,
    /// Среднее время между появления низкоприоритетных задач
    pub avg_time_between_low_prior_tasks: f32,
    /// Среднее время между появления обычных задач
    pub avg_time_between_normal_prior_tasks: f32,
}
