use crate::pulumi::Pulumi;
use anyhow::{Context, Error};
use clap::{Args, Parser, Subcommand, arg};
use log::LevelFilter;
use log4rs::Config;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Root};
use log4rs::encode::json::JsonEncoder;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

mod pulumi;
mod version_finder;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct App {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Run {
        program: PathBuf,
    },
    Plugins {
        program: PathBuf,
        destination: PathBuf,
    },
}

#[derive(Debug, Args)]
struct GlobalOpts {
    #[arg(short, long)]
    wasm: Option<String>,

    #[arg(short, long)]
    cwasm: Option<String>,
}

fn main() -> Result<(), Error> {
    let args = App::parse();

    let logfile = FileAppender::builder()
        // .encoder(Box::new(log4rs::encode::pattern::PatternEncoder::new("{h({d(%Y-%m-%d %H:%M:%S)} - [{l}] [{M}] [{f}:{L}] {m}{n})}")))
        .encoder(Box::new(JsonEncoder::new()))
        .build("output.log")?;

    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(Root::builder().appender("logfile").build(LevelFilter::Info))?;

    let _handle = log4rs::init_config(config)?;

    match &args.command {
        Command::Run { program } => {
            let mut pulumi = Pulumi::create(program)?;
            log::info!("Invoking main");
            pulumi.start()?;
        }
        Command::Plugins {
            program,
            destination,
        } => {
            let program = fs::read(program)
                .context(format!("Cannot read program {}", 
                    program.to_str().expect("Program path is not valid UTF-8")))?;

            let plugins = version_finder::extract_custom_section(&program);

            let mut file = File::create(destination).context("Cannot create destination file")?;
            file.write_all(
                serde_json::to_string(&plugins)
                    .context("Cannot serialize plugins")?
                    .as_bytes(),
            )
            .context("Cannot write to destination file")?;
        }
    }

    Ok(())
}
