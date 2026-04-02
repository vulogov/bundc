extern crate log;
use crate::cmd::{Cli, Disassemble};

#[time_graph::instrument]
pub fn run(_: &Cli, _dargs: &Disassemble) {
    log::debug!("DISASSEMBLE::run() reached");
}
