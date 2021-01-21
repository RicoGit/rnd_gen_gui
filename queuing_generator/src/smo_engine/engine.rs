//! Реализация движока системы массивого обслуживания

use crate::smo_engine::model::{Options, State, Stats, Task};
use anyhow::Result;
use std::borrow::BorrowMut;
use std::sync::{Arc, Mutex};
use std::thread;

/// Внутренне состояние движка эмеляуии
#[derive(Debug)]
pub struct Engine {
    state: State,
    options: Options,
}

impl Engine {
    pub fn new(options: Options) -> Self {
        let state = State::new();
        Engine { state, options }
    }

    /// Начинает эмуляцию в фоне
    pub fn start(engine: Arc<Mutex<Self>>) {
        // todo добавить проверку времени окончания
        // запускаем эмуляцию в отдельном потоке
        thread::spawn(move || {
            loop {
                // захватываем мьютекс, выполняем раунд эмуляции и отпускаем лок в конце цикла
                engine.as_ref().lock().map(|mut engine| {
                    let now = engine.state.now + 1;
                    engine.make_round(now)
                });
            }
        });
    }

    /// Рассчитывает модель для заданного момента времени
    pub fn make_round(&mut self, now: usize) {
        println!("Раунд: {:?}", now);

        // время пройдено с последнего раунда
        let time_elapsed = now - self.state.now;

        // обновляем часы
        self.state.now = now;

        // пробуем сосздать задачу, если создана кладем в очередь согласно приоритету
        self.put_task(Task::try_new(now, time_elapsed, self.options));

        let rest_work = self.state.rest_time_working;

        if (rest_work - time_elapsed) <= 0 {
            // запущенных задач нет, запускаем новую
            self.try_start_task();
        } else {
            // задча пока работает, обновляем остаток времени
            self.state.rest_time_working = rest_work - time_elapsed;
        }
    }

    /// Складываем задачу в очередь
    pub fn put_task(&mut self, task: Option<Task>) {
        task.map(|task| {
            if task.low_priority {
                self.state.low_prior_queue.push(task) // LIFO
            } else {
                self.state.queue.push_back(task) // FIFO
            }
        });
    }

    /// Запускаем задачу на выполнение если что то есть в очереди и возвращает эту задачу.
    /// Сначала пытаемся достать из очереди с нормальным приоритетом, затем из очереди с низким.
    pub fn try_start_task(&mut self) {
        self.state
            .queue
            .pop_front() // FIFO
            .or(self.state.low_prior_queue.pop()) // LIFO
            .map(|task| self.update_state(task))
            .or_else(|| {
                println!("Обе очерели пусты, нечего запускать");
                None
            });
    }

    /// Oбновляем внутренне состояние системы
    fn update_state(&mut self, task: Task) {
        self.state.rest_time_working = task.require_time;
        self.state.task_done_total += 1;
        self.state.time_between_tasks_total += task.require_time;
        if task.low_priority {
            self.state.low_prior_task_done_total += 1;
        }
    }
}

/// Запуск эмуляци
pub fn start() -> Result<()> {
    todo!()
}

/// Остановка эмуляции
pub fn stop() -> Result<()> {
    todo!()
}
