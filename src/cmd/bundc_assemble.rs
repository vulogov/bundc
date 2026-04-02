extern crate log;
use crate::cmd::{Assemble, Cli};

#[time_graph::instrument]
pub fn run(_: &Cli, _aargs: &Assemble) {
    log::debug!("ASSEMBLE::run() reached");
}
