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

mod domain_ir;
mod domain_to_rust;
mod generator;
mod golang;
mod package_model;
mod pcl_model;
mod pcl_to_domain;
mod rust_ir;
mod rust_to_string;

fn generate_project(req: GenerateProjectRequest) -> Result<()> {
    let program = pulumi_gestalt_proto::language_server::pulumipcl::PclProtobufProgram::decode(
        &*req.protobuf,
    )
    .context("Cannot decode protobuf")?;
    let current_dir = env::current_dir()
        .context("Cannot get current directory")?
        .join("../crates/rust");
    let current_dir = current_dir
        .to_str()
        .context("Current directory is not valid UTF-8")?;
    let model_program = pcl_model::map_program(program);
    let result = generate_main(&model_program).context("Failed to generate main.rs")?;
    let cargo_rs = include_str!("./Cargo.toml.template");
    let cargo_rs = cargo_rs.replace("{{CURRENT_DIR}}", current_dir);
    let mut files = vec![
        FileWithContent {
            path: "src/main.rs".to_string(),
            content: result.main_rs.into_bytes(),
        },
        FileWithContent {
            path: "Cargo.toml".to_string(),
            content: cargo_rs.as_bytes().to_vec(),
        },
    ];

    if req.testing {
        let protobuf_json = serde_json::to_vec_pretty(&model_program)
            .context("Failed to serialize protobuf to JSON")?;
        files.push(FileWithContent {
            path: "protobuf.json".to_string(),
            content: protobuf_json,
        });
        let domain_json = serde_json::to_vec_pretty(&result.domain)
            .context("Failed to serialize domain IR to JSON")?;
        files.push(FileWithContent {
            path: "domain.json".to_string(),
            content: domain_json,
        });
        let rust_ir_json = serde_json::to_vec_pretty(&result.rust_ir)
            .context("Failed to serialize Rust IR to JSON")?;
        files.push(FileWithContent {
            path: "rust_ir.json".to_string(),
            content: rust_ir_json,
        });
    }

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

        if let Err(error) = pulumi_gestalt_generator::generate_rust(&_model_package, &dir.join("src")) {
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

        let result = match generate_main(&model_program) {
            Ok(result) => result,
            Err(error) => {
                return GenerateProgramResult {
                    files_content: vec![],
                    error: format!("failed to generate main.rs: {error:?}"),
                };
            }
        };

        let mut files = vec![FileWithContent {
            path: "main.rs".to_string(),
            content: result.main_rs.into_bytes(),
        }];

        if req.testing {
            match serde_json::to_vec_pretty(&model_program) {
                Ok(json) => files.push(FileWithContent {
                    path: "protobuf.json".to_string(),
                    content: json,
                }),
                Err(error) => {
                    return GenerateProgramResult {
                        files_content: vec![],
                        error: format!("failed to serialize protobuf to JSON: {error:?}"),
                    };
                }
            }
            match serde_json::to_vec_pretty(&result.domain) {
                Ok(json) => files.push(FileWithContent {
                    path: "domain.json".to_string(),
                    content: json,
                }),
                Err(error) => {
                    return GenerateProgramResult {
                        files_content: vec![],
                        error: format!("failed to serialize domain IR to JSON: {error:?}"),
                    };
                }
            }
            match serde_json::to_vec_pretty(&result.rust_ir) {
                Ok(json) => files.push(FileWithContent {
                    path: "rust_ir.json".to_string(),
                    content: json,
                }),
                Err(error) => {
                    return GenerateProgramResult {
                        files_content: vec![],
                        error: format!("failed to serialize Rust IR to JSON: {error:?}"),
                    };
                }
            }
        }

        GenerateProgramResult {
            files_content: files,
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
        testing: false,
    })
}
