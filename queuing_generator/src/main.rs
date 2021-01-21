extern crate web_view;

use anyhow::Result;
pub use serde::{Deserialize, Serialize};

use crate::smo_engine::engine::Engine;
use crate::smo_engine::model::Options;
use std::sync::{Arc, Mutex};
use web_view::*;

mod smo_engine;

fn main() {
    let _ = web_view::builder()
        .title("Модель системы массового обслуживания")
        .content(Content::Html(include_str!("../gui/index.html")))
        .size(1200, 900)
        .resizable(true)
        .debug(false)
        .user_data(Option::None)
        .invoke_handler(invoke_handler)
        .run()
        .unwrap();
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum Action {
    Start { options: Options },
    Stop,
    Stats,
}

/// Parses string cmd and returns struct
fn parse_cmd(arg: &str) -> Result<Action> {
    let cmd = serde_json::from_str(arg)?;
    Ok(cmd)
}

fn invoke_handler(wv: &mut WebView<Option<Arc<Mutex<Engine>>>>, arg: &str) -> WVResult {
    let action = parse_cmd(arg).expect("Cmd should be defined");

    match action {
        Action::Start { options } => {
            let engine = Arc::new(Mutex::new(Engine::new(options)));

            // запускаем эмуляцию в отдельном треде
            Engine::start(engine.clone(), options.time_scale_millis)
                .expect("Не смог начать симуляцию");

            // перетираем прошлый движок во внутреннем состоянии программы
            wv.user_data_mut().replace(engine);

            let start_js = format!("started(true)");
            println!("start_js: {:?}", start_js);

            // вызываем функцию в Js для отрисовки UI
            wv.eval(&start_js)?;
        }
        Action::Stop => {
            // останавливаем эмуляцию
            let data = wv.user_data().clone();
            data.map(|arc| Engine::stop(arc));

            let stop_js = format!("started(false)");
            println!("stop_js: {:?}", stop_js);
            // вызываем функцию в Js для отрисовки UI
            wv.eval(&stop_js)?;
        }
        Action::Stats => {
            // получение статистики

            if let Some(data) = wv.user_data() {
                let stats = data
                    .lock()
                    .map(|engine| engine.get_stats())
                    .expect("Не могу получить статистику");

                let stats_js = format!("fillStats({})", serde_json::to_string(&stats).unwrap());
                // println!("stats_js: {:?}", stats_js);
                // вызываем функцию в Js для отрисовки UI
                wv.eval(&stats_js)?;
            };
        }
    }

    Ok(())
}
