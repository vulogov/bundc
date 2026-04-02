extern crate log;
use crate::cmd::{Cli, Compile, common};
use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;
use comfy_table::*;
use easy_error::{Error, bail};
use rust_dynamic::value::Value;

fn bund_vec_to_list(state: &mut Vec<Value>) -> Result<Value, Error> {
    let mut res = Value::list();
    for e in state.iter() {
        res = res.push(e.clone());
    }
    Ok(res)
}

fn bundc_compile<N: AsRef<str> + ToString>(source: N, cargs: &Compile) -> Result<Value, Error> {
    match bund_language_parser::bund_parse(&source.to_string()) {
        Ok(mut tokens) => {
            if cargs.skip_last_exit {
                log::debug!("Removing last EXIT");
                common::remove_if_matches(&mut tokens, |x: &rust_dynamic::value::Value| x.dt == 93);
            }
            bund_vec_to_list(&mut tokens)
        }
        Err(err) => {
            bail!("Error compiling BUND code: {}", err)
        }
    }
}

fn bundc_compile_to_binary<N: AsRef<str> + ToString>(
    source: N,
    cargs: &Compile,
) -> Result<Vec<u8>, Error> {
    match bundc_compile(source, cargs) {
        Ok(the_list) => the_list.to_binary(),
        Err(err) => {
            bail!("{}", err)
        }
    }
}

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
    let output = match bundc_compile_to_binary(script, cargs) {
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
    let mut output = match bund_language_parser::bund_parse(&script) {
        Ok(output) => output,
        Err(err) => {
            log::error!("BUND error: {}", err);
            return;
        }
    };
    if cargs.skip_last_exit {
        log::debug!("Removing last EXIT");
        common::remove_if_matches(&mut output, |x: &rust_dynamic::value::Value| x.dt == 93);
    }
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
