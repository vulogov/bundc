extern crate log;
use shadow_rs::shadow;
shadow!(build);

pub mod setloglevel;

use clap::{Args, Parser, Subcommand};
use lazy_static::lazy_static;
use std::env;
use std::sync::Mutex;
use time_graph;

pub mod bundc_assemble;
pub mod bundc_compile;
pub mod bundc_disassemble;
pub mod bundc_display_banner;
pub mod bundc_version;
pub mod common;

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
    log::debug!("BUND version: {}", bund_language_parser::version());
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
        Commands::Compile(cargs) => {
            bundc_compile::run(&cli, &cargs);
        }
        Commands::Assemble(aargs) => {
            bundc_assemble::run(&cli, &aargs);
        }
        Commands::Disassemble(dargs) => {
            bundc_disassemble::run(&cli, &dargs);
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
    Compile(Compile),
    Assemble(Assemble),
    Disassemble(Disassemble),
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

    #[clap(short, long, action = clap::ArgAction::SetTrue, help="Execute internal profiler")]
    pub profile: bool,

    #[clap(subcommand, help = "BUNDC subcommands")]
    command: Commands,
}

#[derive(Args, Clone, Debug)]
#[clap(about = "Compile BUND to bytecode")]
pub struct Compile {
    #[clap(long, action = clap::ArgAction::SetTrue, help="Dump internal represrentation of bytecode to STDOUT")]
    pub dump: bool,

    #[clap(long, action = clap::ArgAction::SetTrue, help="Remove last EXIT")]
    pub skip_last_exit: bool,

    #[clap(help = "Full path to the compiled binary", short, long)]
    pub out: Option<String>,

    #[clap(help = "Full path to the source code", short, long, required = true)]
    pub src: Option<String>,
}

#[derive(Args, Clone, Debug)]
#[clap(about = "Assemble bytecode into container")]
pub struct Assemble {
    #[clap(help = "Full path to the binary container", short, long)]
    pub out: Option<String>,

    #[clap(
        help = "Full path to the compiled bytecode",
        short,
        long,
        required = true
    )]
    pub src: Option<String>,
}

#[derive(Args, Clone, Debug)]
#[clap(about = "Disassemble bytecode")]
pub struct Disassemble {
    #[clap(
        help = "Full path to the compiled bytecode",
        short,
        long,
        required = true
    )]
    pub src: Option<String>,
}

#[derive(Args, Clone, Debug)]
#[clap(about = "Get the version of the BUNDC")]
pub struct Version {}
