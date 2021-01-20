extern crate web_view;

use anyhow::Result;
pub use serde::{Deserialize, Serialize};

use web_view::*;

fn main() {
    let _res = web_view::builder()
        .title("Модель системы массового обслуживания")
        .content(Content::Html(include_str!("../gui/index.html")))
        .size(1200, 900)
        .resizable(true)
        .debug(false)
        .user_data(0)
        .invoke_handler(invoke_handler)
        .run()
        .unwrap();
}

#[derive(Serialize, Deserialize)]
struct GenCmd {
    cmd: String,
    kind: String,
    size: usize,
}

/// Parses string cmd and returns struct
fn parse_cmd(_arg: &str) -> Result<GenCmd> {
    // todo parse json

    // let vec = arg.split('|').collect::<Vec<_>>();
    // assert!(vec.len() > 2, "cmd should have at least 3 fields");
    //
    // Ok(GenCmd {
    //     cmd: vec[0].to_string(),
    //     kind: vec[1].to_string(),
    //     size: usize::from_str(vec[2])?
    // })

    todo!()
}

fn invoke_handler(wv: &mut WebView<usize>, arg: &str) -> WVResult {
    println!("Handled {:?}", arg);

    let GenCmd { cmd, kind, size: _ } = parse_cmd(arg).expect("Cmd should be defined");

    // todo implement
    if cmd == "gen" {
        if kind == "genRust" {
            let stats_js = format!("fillStats({})", "todo data");
            println!("stats_js: {:?}", stats_js);
            // вызываем функцию в Js для отрисовки UI
            wv.eval(&stats_js)?;
        } else {
            println!("Unknown kind {:?}", kind)
        }
    }

    Ok(())
}
