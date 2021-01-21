extern crate web_view;

use anyhow::Result;
pub use serde::{Deserialize, Serialize};

use web_view::*;
use crate::smo_engine::model::Options;
use crate::smo_engine::engine::Engine;
use std::sync::{Arc, Mutex};

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
pub enum Action { Start, Stop }

#[derive(Serialize, Deserialize, Debug)]
pub struct Cmd {
    cmd: Action,
    options: Options,
}

/// Parses string cmd and returns struct
fn parse_cmd(arg: &str) -> Result<Cmd> {
    let cmd = serde_json::from_str(arg)?;
    Ok(cmd)
}

fn invoke_handler(wv: &mut WebView<Option<Arc<Mutex<Engine>>>>, arg: &str) -> WVResult {
    println!("Handled {:?}", arg);

    let Cmd { cmd, options } = parse_cmd(arg).expect("Cmd should be defined");

    match cmd {
        Action::Start => {
            let engine = Arc::new(Mutex::new(Engine::new(options)));

            // запускаем эмуляцию в отдельном треде
            Engine::start(engine.clone());

            let mut data = wv.user_data_mut();
            // перетираем прошлый движок во внутреннем состоянии программы
            data = &mut Some(engine);


            let start_js = format!("started(true)");
            println!("stats_js: {:?}", start_js);
            // вызываем функцию в Js для отрисовки UI
            wv.eval(&start_js)?;
        }
        Action::Stop => {
            let mut data = wv.user_data_mut();
            data = &mut None;


            let stop_js = format!("started(false)");
            println!("stop_js: {:?}", stop_js);
            // вызываем функцию в Js для отрисовки UI
            wv.eval(&stop_js)?;
        }
    }

    Ok(())
}
