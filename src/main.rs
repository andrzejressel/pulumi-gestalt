use anyhow::Error;
use clap::{arg, Args, Parser, Subcommand};
use log4rs::append::file::FileAppender;
use log4rs::Config;
use log4rs::config::{Appender, Root};
use log4rs::encode::json::JsonEncoder;
use log::LevelFilter;

use pulumi_rust::pulumi::{Pulumi, WasmFile};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct App {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Run {
        #[arg(short, long)]
        wasm: Option<String>,

        #[arg(short, long)]
        cwasm: Option<String>,
    },
    Plugins {
        #[arg(short, long)]
        wasm: Option<String>,

        #[arg(short, long)]
        cwasm: Option<String>,
    },
    Compile {
        #[arg(short, long)]
        wasm: String,
        #[arg(short, long)]
        output: String
    },
}

#[derive(Debug, Args)]
struct GlobalOpts {
    #[arg(short, long)]
    wasm: Option<String>,

    #[arg(short, long)]
    cwasm: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args = App::parse();

    let logfile = FileAppender::builder()
        // .encoder(Box::new(log4rs::encode::pattern::PatternEncoder::new("{h({d(%Y-%m-%d %H:%M:%S)} - [{l}] [{M}] [{f}:{L}] {m}{n})}")))
        .encoder(Box::new(JsonEncoder::new()))
        .build("output.log")?;

    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(
            Root::builder()
                .appender("logfile")
                .build(LevelFilter::Info),
        )?;

    let _handle = log4rs::init_config(config)?;

    match &args.command {
        Command::Run { wasm, cwasm } => {
            let wasm = match (wasm, cwasm) {
                (Some(_), Some(_)) => {
                    eprintln!("Cannot specify both wasm and cwasm");
                    std::process::exit(1);
                }
                (Some(wasm), None) => WasmFile::WASM(wasm.clone()),
                (None, Some(cwasm)) => WasmFile::CWASM(cwasm.clone()),
                (None, None) => {
                    eprintln!("Must specify either wasm or cwasm");
                    std::process::exit(1);
                }
            };

            let _pulumi_engine_url = std::env::var("PULUMI_ENGINE")?;
            let pulumi_monitor_url = std::env::var("PULUMI_MONITOR")?;

            let pulumi = Pulumi::create(&wasm, &Some(pulumi_monitor_url)).await?;
            pulumi.start().await?;
        }
        Command::Compile { wasm, output } => {
            let compiled = Pulumi::compile(&wasm)?;
            std::fs::write(output, compiled)?;
        }
        Command::Plugins { .. } => todo!()
    }

    Ok(())
}
