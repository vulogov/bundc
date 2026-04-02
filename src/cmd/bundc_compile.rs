extern crate log;
use crate::cmd::{Cli, Compile, common};
use bund_language_parser::compile;
use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;
use comfy_table::*;

fn bundc_compile_to_file(cargs: &Compile) {
    let out = match &cargs.out {
        Some(out) => out.to_string(),
        None => "bund.out".to_string(),
    };
    let src = match &cargs.src {
        Some(src) => src.to_string(),
        None => {
            log::error!("Missed argument --src");
            return;
        }
    };
    let script = match common::read_file(&src) {
        Ok(source) => source,
        Err(err) => {
            log::error!("Error reading file: {}", err);
            return;
        }
    };
    log::debug!("Reading from: {}, {} bytes", &src, &script.len());
    log::debug!("Writing to: {}", &out);
    let output = match compile::compile_to_binary(script) {
        Ok(output) => output,
        Err(err) => {
            log::error!("BUND error: {}", err);
            return;
        }
    };
    match common::write_file(&out, output) {
        Ok(_) => log::debug!("All Okay!"),
        Err(err) => log::error!("Write error: {}", err),
    }
}

fn bundc_dump(cargs: &Compile) {
    let src = match &cargs.src {
        Some(src) => src.to_string(),
        None => {
            log::error!("Missed argument --src");
            return;
        }
    };
    let script = match common::read_file(&src) {
        Ok(source) => source,
        Err(err) => {
            log::error!("Error reading file: {}", err);
            return;
        }
    };
    log::debug!("Reading from: {}, {} bytes", &src, &script.len());
    let output = match bund_language_parser::bund_parse(&script) {
        Ok(output) => output,
        Err(err) => {
            log::error!("BUND error: {}", err);
            return;
        }
    };
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_content_arrangement(ContentArrangement::Dynamic);
    for (n, v) in output.iter().enumerate() {
        table.add_row(vec![
            Cell::new(format!("{}", n)).fg(Color::Green),
            Cell::new(format!("{:?}", v)).fg(Color::White),
        ]);
    }
    println!("{}", table);
}

#[time_graph::instrument]
pub fn run(_: &Cli, cargs: &Compile) {
    log::debug!("COMPILE::run() reached");
    if cargs.dump {
        bundc_dump(cargs);
    } else {
        bundc_compile_to_file(cargs);
    }
}
