extern crate log;
use crate::cmd::{Cli, Compile};

#[time_graph::instrument]
pub fn run(_: &Cli, _cargs: &Compile) {
    log::debug!("COMPILE::run() reached");
}
