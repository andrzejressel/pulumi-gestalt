use crate::pulumi::Pulumi;
use anyhow::{Context, Error};
use clap::{Args, Parser, Subcommand, arg};
use log::LevelFilter;
use log4rs::Config;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Root};
use log4rs::encode::json::JsonEncoder;
use pulumi_gestalt_wasm_component_creator::source::GithubWasmComponentSource;
use pulumi_gestalt_wasm_component_creator::source::WasmComponentSource;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

mod model;
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
        #[arg(long)]
        pulumi_gestalt: Option<PathBuf>,
        #[clap(
            long,
            action,
            help = "When set to true, Wasm components with debug symbols will be used. Should be only used for debugging - it will massively increase execution time"
        )]
        debug: bool,
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

#[tokio::main]
async fn main() -> Result<(), Error> {
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
        Command::Run {
            pulumi_gestalt,
            debug,
            program,
        } => {
            use pulumi_gestalt_wasm_component_creator::source::FileSource;
            log::info!("Debug set to {debug}");
            log::info!("Creating final component");
            let pulumi_gestalt_source: Box<dyn WasmComponentSource> = match pulumi_gestalt {
                None => Box::new(GithubWasmComponentSource {}),
                Some(location) => Box::new(FileSource::new(location.clone())),
            };

            let component = pulumi_gestalt_wasm_component_creator::create(
                pulumi_gestalt_source.as_ref(),
                fs::read(program)
                    .context(format!("Cannot read program {}", program.to_str().unwrap()))?,
                *debug,
            )
            .await?;
            log::info!("Created final component");
            let wasm = component;

            let pulumi_engine_url = std::env::var("PULUMI_ENGINE")?;
            let pulumi_monitor_url = std::env::var("PULUMI_MONITOR")?;
            let pulumi_stack = std::env::var("PULUMI_STACK")?;
            let pulumi_project = std::env::var("PULUMI_PROJECT")?;
            let pulumi_preview = match std::env::var("PULUMI_DRY_RUN") {
                Ok(preview) if preview == "true" => true,
                Ok(preview) if preview == "false" => false,
                _ => false,
            };

            let mut pulumi = Pulumi::create(
                wasm,
                pulumi_monitor_url,
                pulumi_engine_url,
                pulumi_stack,
                pulumi_project,
                pulumi_preview
            )
            .await?;
            log::info!("Invoking main");
            pulumi.start().await?;
        }
        Command::Plugins {
            program,
            destination,
        } => {
            let program = fs::read(program)
                .context(format!("Cannot read program {}", program.to_str().unwrap()))?;

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
