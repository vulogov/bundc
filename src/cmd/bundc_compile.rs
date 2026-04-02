extern crate log;
use crate::cmd::{Cli, Compile, common};
use bund_language_parser::compile;

#[time_graph::instrument]
pub fn run(_: &Cli, cargs: &Compile) {
    log::debug!("COMPILE::run() reached");
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
