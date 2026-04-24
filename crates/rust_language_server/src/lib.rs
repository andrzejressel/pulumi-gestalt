use crate::cargo_templater::Dependency;
use crate::golang::{
    FileWithContent, G2RCall, G2RCallImpl, GeneratePackageRequest, GeneratePackageResult,
    GenerateProgramRequest, GenerateProgramResult, GenerateProjectRequest, GenerateProjectResult,
};
use crate::pcl_model::PluginReference;
use generator::generate_main;
use prost::Message;
use rootcause::Result;
use rootcause::compat::IntoRootcause;
use rootcause::option_ext::OptionExt;
use rootcause::prelude::ResultExt;
use std::collections::HashMap;
use std::fs::create_dir_all;
use std::path::{Path, PathBuf};
use std::{env, fs};

mod cargo_templater;
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

    let model_program = pcl_model::map_program(program);
    let result = generate_main(&model_program).context("Failed to generate main.rs")?;

    let local_dependencies = req
        .local_dependencies
        .into_iter()
        .map(|ld| (ld.name, ld.path))
        .collect::<HashMap<_, _>>();

    let mut dependencies = vec![
        Dependency {
            name: "bon".to_string(),
            source: cargo_templater::DependencySource::CratesIo {
                version: "3.9.1".to_string(),
            },
        },
        Dependency {
            name: "anyhow".to_string(),
            source: cargo_templater::DependencySource::CratesIo {
                version: "1.0.102".to_string(),
            },
        },
        Dependency {
            name: "serde_json".to_string(),
            source: cargo_templater::DependencySource::CratesIo {
                version: "1.0.140".to_string(),
            },
        },
        create_pulumi_gestalt_rust_dependency()
            .context("Failed to create pulumi_gestalt_rust dependency")?,
    ];
    dependencies
        .append(create_provider_dependencies(&model_program.plugins, &local_dependencies).as_mut());

    let cargo_rs = cargo_templater::render_cargo_toml(&cargo_templater::CargoToml {
        name: "pulumi-rust".to_string(),
        version: "0.0.0".to_string(),
        dependencies,
    })
    .context("Failed to render Cargo.toml")?;
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

fn create_provider_dependencies(
    plugins: &Vec<PluginReference>,
    local_dependencies: &HashMap<String, String>,
) -> Vec<Dependency> {
    plugins
        .iter()
        .map(|plugin| {
            let plugin_name = &plugin.name;
            if let Some(local_path) = local_dependencies.get(plugin_name) {
                Dependency {
                    name: format!("pulumi_{}", plugin_name),
                    source: cargo_templater::DependencySource::Local {
                        path: local_path.clone(),
                    },
                }
            } else {
                Dependency {
                    name: format!("pulumi_{}", plugin_name),
                    source: cargo_templater::DependencySource::CratesIo {
                        version: plugin.version.clone(),
                    },
                }
            }
        })
        .collect()
}

fn generate_package(req: GeneratePackageRequest) -> Result<()> {
    let package =
        pulumi_gestalt_proto::language_server::pulumipackage::Package::decode(&*req.protobuf)
            .context("Cannot decode package protobuf")?;

    let model_package = package_model::map_package(package);

    let dir = Path::new(&req.directory);
    if !dir.exists() {
        create_dir_all(dir).context("Failed to create output directory")?;
    }

    pulumi_gestalt_generator::generate_rust(&model_package, &dir.join("src"))
        .into_rootcause()
        .context("Failed to generate package")?;

    let cargo_rs = cargo_templater::render_cargo_toml(&cargo_templater::CargoToml {
        name: format!("pulumi_{}", &model_package.name),
        version: model_package.version.clone(),
        dependencies: vec![
            Dependency {
                name: "bon".to_string(),
                source: cargo_templater::DependencySource::CratesIo {
                    version: "3.9.1".to_string(),
                },
            },
            Dependency {
                name: "anyhow".to_string(),
                source: cargo_templater::DependencySource::CratesIo {
                    version: "1.0.102".to_string(),
                },
            },
            Dependency {
                name: "serde_json".to_string(),
                source: cargo_templater::DependencySource::CratesIo {
                    version: "1.0.140".to_string(),
                },
            },
            create_pulumi_gestalt_rust_dependency()
                .context("Failed to create pulumi_gestalt_rust dependency")?,
        ],
    })
    .context("Failed to render Cargo.toml")?;

    fs::write(dir.join("Cargo.toml"), cargo_rs).context("Failed to write Cargo.toml")?;

    Ok(())
}

fn get_project_root_dir() -> PathBuf {
    let cargo_manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    cargo_manifest_dir.join("..").join("..")
}

fn create_pulumi_gestalt_rust_dependency() -> Result<Dependency> {
    Ok(Dependency {
        name: "pulumi_gestalt_rust".to_string(),
        source: cargo_templater::DependencySource::Local {
            path: get_project_root_dir()
                .join("crates")
                .join("rust")
                .to_str()
                .context("Failed to convert pulumi_gestalt_rust path to string")?
                .to_string(),
        },
    })
}

impl G2RCall for G2RCallImpl {
    fn generate_package(req: GeneratePackageRequest) -> GeneratePackageResult {
        match generate_package(req) {
            Ok(()) => GeneratePackageResult {
                error: String::new(),
            },
            Err(error) => GeneratePackageResult {
                error: error.to_string(),
            },
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
