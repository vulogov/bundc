extern crate log;
use shadow_rs::shadow;
shadow!(build);

pub mod setloglevel;

use clap::{Args, Parser, Subcommand};
use lazy_static::lazy_static;
use std::env;
use std::sync::Mutex;
use time_graph;

pub mod bundc_display_banner;
pub mod bundc_version;

lazy_static! {
    pub static ref CLI: Mutex<Cli> = {
        let e: Mutex<Cli> = Mutex::new(Cli::parse());
        e
    };
}

fn do_panic() {
    log::debug!("Setting a global panic handler");
    better_panic::Settings::auto()
        .most_recent_first(false)
        .lineno_suffix(true)
        .verbosity(better_panic::Verbosity::Full)
        .install();
}

pub fn main() {
    let cli = Cli::parse();
    setloglevel::setloglevel(&cli);
    do_panic();
    let init_cli = CLI.lock().unwrap();
    log::debug!(
        "BUND bytecode compiler version:{}, tag:{}, branch:{}, commit date: {}, commit author:{}({}), commit_id:{}. Build at {}",
        build::VERSION,
        build::TAG,
        build::BRANCH,
        build::COMMIT_DATE,
        build::COMMIT_AUTHOR,
        build::COMMIT_EMAIL,
        build::COMMIT_HASH,
        build::BUILD_TIME
    );
    log::debug!("BUNDCORE version: {}", bundcore::version());
    log::debug!("Initialize global CLI");
    drop(init_cli);
    log::debug!("BUNDC context initialized ...");

    if cli.profile {
        log::debug!("Enable BUNDC profiler");
        time_graph::enable_data_collection(true);
    }

    match &cli.command {
        Commands::Version(_) => {
            bundc_version::run(&cli);
        }
    }

    if cli.profile {
        log::debug!("Generating BUNDC profiler report");
        let graph = time_graph::get_full_graph();
        println!("{}", graph.as_table());
    }
}

#[derive(Subcommand, Clone, Debug)]
enum Commands {
    Version(Version),
}

#[derive(Parser, Clone, Debug)]
#[clap(name = "bundc")]
#[clap(author = "Vladimir Ulogov <vladimir@ulogov.us>")]
#[clap(version = env!("CARGO_PKG_VERSION"))]
#[clap(
    about = "BUNDC - BUND language compiler",
    long_about = "Compiling BUND code int a bytecode"
)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[clap(long, action = clap::ArgAction::SetTrue, help="Execute internal profiler")]
    pub profile: bool,

    #[clap(help = "Full path to the output file", long)]
    pub output: Option<String>,

    #[clap(subcommand, help = "BUNDC subcommands")]
    command: Commands,
}

#[derive(Args, Clone, Debug)]
#[clap(about = "Get the version of the BUNDC")]
pub struct Version {}
