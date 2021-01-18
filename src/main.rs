extern crate web_view;

use web_view::*;
use std::str::FromStr;
use anyhow::Result;

mod rust_gen;
mod utils;


fn main() {
    let res = web_view::builder()
        .title("Генерация псевдослучайных числовых последовательностей")
        .content(Content::Html(include_str!("../gui/index.html")))
        .size(1200, 900)
        .resizable(true)
        .debug(false)
        .user_data(0)
        .invoke_handler(invoke_handler)
        .run()
        .unwrap();
    println!("res: {:?}", res)
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
        size: usize::from_str(vec[2])?
    })
}

fn invoke_handler(wv: &mut WebView<usize>, arg: &str) -> WVResult {
    println!("Handled {:?}", arg);

    let GenCmd { cmd, kind, size } = parse_cmd(arg).expect("Cmd should be defined");

    if cmd == "gen" {
        if kind == "genRust" {
            let data = rust_gen::generate(size);
            let fill_chart_js = format!("fillCharts({:?})", &data);
            println!("fill_chart_js: {:?}", fill_chart_js);
            wv.eval(&fill_chart_js)?;

            let json_stats = serde_json::to_string(&utils::stats(&data)).unwrap();
            let stats_js = format!("fillStats({})", json_stats);
            println!("stats_js: {:?}", stats_js);
            wv.eval(&stats_js)?;
        } else if kind == "genLemer" {
            todo!("not implemented")
        } else {
            println!("Unknown kind {:?}", kind)
        }
    }

    Ok(())
}