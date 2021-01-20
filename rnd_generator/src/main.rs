extern crate web_view;

use anyhow::Result;
use std::str::FromStr;
use utils;
use web_view::*;

mod lemer_gen;
mod rust_gen;

fn main() {
    let _res = web_view::builder()
        .title("Генерация псевдослучайных числовых последовательностей")
        .content(Content::Html(include_str!("../gui/index.html")))
        .size(1200, 900)
        .resizable(true)
        .debug(false)
        .user_data(0)
        .invoke_handler(invoke_handler)
        .run()
        .unwrap();
}

struct GenCmd {
    cmd: String,
    kind: String,
    size: usize,
}

/// Parses string cmd and returns struct
fn parse_cmd(arg: &str) -> Result<GenCmd> {
    let vec = arg.split('|').collect::<Vec<_>>();
    assert!(vec.len() > 2, "cmd should have at least 3 fields");

    Ok(GenCmd {
        cmd: vec[0].to_string(),
        kind: vec[1].to_string(),
        size: usize::from_str(vec[2])?,
    })
}

fn invoke_handler(wv: &mut WebView<usize>, arg: &str) -> WVResult {
    println!("Handled {:?}", arg);

    let GenCmd { cmd, kind, size } = parse_cmd(arg).expect("Cmd should be defined");

    if cmd == "gen" {
        if kind == "genRust" {
            // генерируем случайную величину
            let data = rust_gen::generate(size);

            let json_stats = serde_json::to_string(&utils::stats(&data)).unwrap();
            let stats_js = format!("fillStats({})", json_stats);
            println!("stats_js: {:?}", stats_js);
            // вызываем функцию в Js для отрисовки UI
            wv.eval(&stats_js)?;
        } else if kind == "genLemer" {
            // генерируем случайную величину
            let data = lemer_gen::generate_arr_int(size, 100);

            let json_stats = serde_json::to_string(&utils::stats(&data)).unwrap();
            let stats_js = format!("fillStats({})", json_stats);
            println!("stats_js: {:?}", stats_js);
            // вызываем функцию в Js для отрисовки UI
            wv.eval(&stats_js)?;
        } else if kind == "pi" {
            // генерируем случайную величину
            let x_coords = lemer_gen::generate_arr(size);
            let y_coords = lemer_gen::generate_arr(size);

            let prob = utils::get_probability(&x_coords, &y_coords);
            let pi_js = format!("fillPi({})", 4. * prob);

            println!("pi_js: {:?}", pi_js);
            wv.eval(&pi_js)?;
        } else {
            println!("Unknown kind {:?}", kind)
        }
    }

    Ok(())
}
