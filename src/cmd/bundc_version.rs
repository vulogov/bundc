extern crate log;
use crate::cmd::Cli;
use crate::cmd::bundc_display_banner;

#[time_graph::instrument]
pub fn run(_: &Cli) {
    log::debug!("VERSION::run() reached");
    println!("{}", bundc_display_banner::bundc_banner());
}
