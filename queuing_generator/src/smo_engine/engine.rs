//! Реализация движока системы массивого обслуживания

use crate::smo_engine::model::{Options, State, Stats, Task};
use anyhow::Result;

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

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
    pub fn start(engine: Arc<Mutex<Self>>, time_scale_millis: u64) -> Result<()> {
        println!("Start engine");
        // запускаем эмуляцию в отдельном потоке
        thread::spawn(move || {
            engine
                .as_ref()
                .lock()
                .map(|mut e| e.state.started = true)
                .expect("Не смог захватить мьютекс");

            loop {
                // ждем паузу
                if time_scale_millis > 0 {
                    thread::sleep(Duration::from_millis(time_scale_millis));
                }

                // захватываем мьютекс, выполняем раунд эмуляции и отпускаем лок в конце цикла
                let mut engine = engine.as_ref().lock().expect("Не смог захватить мьютекс");

                if !engine.state.started || engine.time_is_over() {
                    println!("Останавливаем эмуляцию в фоновом процессе");
                    break;
                }

                let now = engine.state.now + 1;
                engine.make_round(now)
            }
        });

        Ok(())
    }

    /// Останавливает эмуляцию в фоне
    pub fn stop(engine: Arc<Mutex<Self>>) {
        println!("Start engine");
        engine
            .lock()
            .map(|mut e| e.state.started = false)
            .expect("Не смог захватить мьютекс");
    }

    /// Рассчитывает модель для заданного момента времени
    pub fn make_round(&mut self, now: usize) {
        // println!("Раунд: {:?}, state: {:?}", now, self.state);

        // время пройдено с последнего раунда
        let time_elapsed = now - self.state.now;

        // обновляем часы
        self.state.now = now;

        // пробуем сосздать задачу, если создана кладем в очередь согласно приоритету
        self.put_task(Task::try_new(now, time_elapsed, self.options));

        let rest_work = self.state.rest_time_working;

        if rest_work == 0 {
            // запущенных задач нет, запускаем новую если есть
            self.state.task = None;
            self.try_start_task();
        } else {
            // задача пока работает, обновляем остаток времени
            self.state.rest_time_working = rest_work - time_elapsed as u32;
            self.state.load += 1;
        }
    }

    /// Складываем задачу в очередь
    pub fn put_task(&mut self, task: Option<Task>) {
        task.map(|task| {
            println!("push task to queue {:?}", task);

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

    /// Обновляем внутренне состояние системы
    fn update_state(&mut self, task: Task) {
        self.state.load += 1;

        self.state.rest_time_working = task.require_time as u32;

        if self.state.min_task_time_require > task.require_time {
            self.state.min_task_time_require = task.require_time
        }

        let wait_time = self.state.now - task.incoming_time;
        self.state.task_done_total += 1;
        self.state.task_wait_time_total += wait_time;

        if task.low_priority {
            self.state.low_prior_task_done_total += 1;
            self.state.low_prior_task_wait_time_total += wait_time;

            if self.state.low_prior_task_max_wait_time_total < wait_time {
                self.state.low_prior_task_max_wait_time_total = wait_time;
            };
        } else {
            if self.state.normal_prior_task_max_wait_time_total < wait_time {
                self.state.normal_prior_task_max_wait_time_total = wait_time;
            };
        }

        self.state.task_wait_in_q_total +=
            self.state.queue.len() + self.state.low_prior_queue.len();
        self.state.task.replace(task);
    }

    /// Вернет true если время эмуляции вышло
    fn time_is_over(&self) -> bool {
        self.state.now > self.options.max_number_of_rounds
    }

    /// Считает статистику для текущего состояния системы, эта статистика отправляеться в полльзовательский интерфейс
    pub fn get_stats(&self) -> Stats {
        self.state.get_stats()
    }
}
