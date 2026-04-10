use crate::golang::{
    FileWithContent, G2RCall, G2RCallImpl, GeneratePackageRequest, GeneratePackageResult,
    GenerateProgramRequest, GenerateProgramResult, GenerateProjectRequest, GenerateProjectResult,
};
use generator::generate_main;
use prost::Message;
use rootcause::Result;
use rootcause::option_ext::OptionExt;
use rootcause::prelude::ResultExt;
use std::fs::create_dir_all;
use std::path::Path;
use std::{env, fs};

mod generator;
mod golang;
mod package_model;
mod pcl_model;

fn generate_project(req: GenerateProjectRequest) -> Result<()> {
    let program = pulumi_gestalt_proto::language_server::pulumipcl::PclProtobufProgram::decode(
        &*req.protobuf,
    )
    .context("Cannot decode protobuf")?;
    let current_dir = env::current_dir().context("Cannot get current directory")?
        .join("../crates/rust");
    let current_dir = current_dir
        .to_str()
        .context("Current directory is not valid UTF-8")?;
    let model_program = pcl_model::map_program(program);
    let main_rs = generate_main(&model_program).context("Failed to generate main.rs")?;
    let cargo_rs = include_str!("./Cargo.toml.template");
    let cargo_rs = cargo_rs.replace(
        "{{CURRENT_DIR}}",
        current_dir,
    );
    let files = vec![
        FileWithContent {
            path: "src/main.rs".to_string(),
            content: main_rs.into_bytes(),
        },
        FileWithContent {
            path: "Cargo.toml".to_string(),
            content: cargo_rs.as_bytes().to_vec(),
        },
    ];

    let dir = Path::new(&req.directory);
    for file in &files {
        let path = dir.join(file.path.clone());
        if let Some(parent) = path.parent() {
            create_dir_all(parent).context("Failed to create parent directory")?;
        }
        fs::write(path, &file.content).context("Failed to write file")?
    }

    Ok(())
}

impl G2RCall for G2RCallImpl {
    fn generate_package(req: GeneratePackageRequest) -> GeneratePackageResult {
        let package = match pulumi_gestalt_proto::language_server::pulumipackage::Package::decode(
            &*req.protobuf,
        ) {
            Ok(package) => package,
            Err(error) => {
                return GeneratePackageResult {
                    error: format!("invalid package bytes: {error:?}"),
                };
            }
        };
        let _model_package = package_model::map_package(package);

        let dir = Path::new(&req.directory);
        if !dir.exists()
            && let Err(error) = create_dir_all(dir)
        {
            return GeneratePackageResult {
                error: format!("failed to create output directory: {error:?}"),
            };
        }

        if let Err(error) = pulumi_gestalt_generator::generate_rust(&_model_package, dir) {
            return GeneratePackageResult {
                error: format!("failed to generate package: {error:?}"),
            };
        }

        if let Err(error) = fs::write(
            dir.join("Cargo.toml"),
            include_str!("./Cargo.toml.template"),
        ) {
            return GeneratePackageResult {
                error: format!("failed to write Cargo.toml: {error:?}"),
            };
        }

        GeneratePackageResult {
            error: String::new(),
        }
    }

    fn generate_program(req: GenerateProgramRequest) -> GenerateProgramResult {
        let program =
            match pulumi_gestalt_proto::language_server::pulumipcl::PclProtobufProgram::decode(
                &*req.protobuf,
            ) {
                Ok(program) => program,
                Err(error) => {
                    return GenerateProgramResult {
                        files_content: vec![],
                        error: format!("invalid program bytes: {error:?}"),
                    };
                }
            };
        let model_program = pcl_model::map_program(program);

        let main_rs = match generate_main(&model_program) {
            Ok(main_rs) => main_rs,
            Err(error) => {
                return GenerateProgramResult {
                    files_content: vec![],
                    error: format!("failed to generate main.rs: {error:?}"),
                };
            }
        };
        let file = vec![FileWithContent {
            path: "main.rs".to_string(),
            content: main_rs.into_bytes(),
        }];
        GenerateProgramResult {
            files_content: file,
            error: String::new(),
        }
    }

    fn generate_project(req: GenerateProjectRequest) -> GenerateProjectResult {
        match generate_project(req) {
            Ok(()) => GenerateProjectResult {
                error: String::new(),
            },
            Err(error) => GenerateProjectResult {
                error: error.to_string(),
            },
        }
    }
}

pub fn generate_project_from_protobuf(protobuf: Vec<u8>, directory: String) -> Result<()> {
    generate_project(GenerateProjectRequest {
        protobuf,
        directory,
    })
}
