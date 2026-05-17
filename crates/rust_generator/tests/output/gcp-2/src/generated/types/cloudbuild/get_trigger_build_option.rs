#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetTriggerBuildOption {
    /// Requested disk size for the VM that runs the build. Note that this is NOT "disk free";
    /// some of the space will be used by the operating system and build utilities.
    /// Also note that this is the minimum disk size that will be allocated for the build --
    /// the build may run with a larger disk than requested. At present, the maximum disk size
    /// is 1000GB; builds that request more than the maximum are rejected with an error.
    #[builder(into)]
    #[serde(rename = "diskSizeGb")]
    pub r#disk_size_gb: i32,
    /// Option to specify whether or not to apply bash style string operations to the substitutions.
    /// 
    /// NOTE this is always enabled for triggered builds and cannot be overridden in the build configuration file.
    #[builder(into)]
    #[serde(rename = "dynamicSubstitutions")]
    pub r#dynamic_substitutions: bool,
    /// A list of global environment variable definitions that will exist for all build steps
    /// in this build. If a variable is defined in both globally and in a build step,
    /// the variable will use the build step value.
    /// 
    /// The elements are of the form "KEY=VALUE" for the environment variable "KEY" being given the value "VALUE".
    #[builder(into)]
    #[serde(rename = "envs")]
    pub r#envs: Vec<String>,
    /// Option to define build log streaming behavior to Google Cloud Storage. Possible values: ["STREAM_DEFAULT", "STREAM_ON", "STREAM_OFF"]
    #[builder(into)]
    #[serde(rename = "logStreamingOption")]
    pub r#log_streaming_option: String,
    /// Option to specify the logging mode, which determines if and where build logs are stored. Possible values: ["LOGGING_UNSPECIFIED", "LEGACY", "GCS_ONLY", "STACKDRIVER_ONLY", "CLOUD_LOGGING_ONLY", "NONE"]
    #[builder(into)]
    #[serde(rename = "logging")]
    pub r#logging: String,
    /// Compute Engine machine type on which to run the build.
    #[builder(into)]
    #[serde(rename = "machineType")]
    pub r#machine_type: String,
    /// Requested verifiability options. Possible values: ["NOT_VERIFIED", "VERIFIED"]
    #[builder(into)]
    #[serde(rename = "requestedVerifyOption")]
    pub r#requested_verify_option: String,
    /// A list of global environment variables, which are encrypted using a Cloud Key Management
    /// Service crypto key. These values must be specified in the build's Secret. These variables
    /// will be available to all build steps in this build.
    #[builder(into)]
    #[serde(rename = "secretEnvs")]
    pub r#secret_envs: Vec<String>,
    /// Requested hash for SourceProvenance. Possible values: ["NONE", "SHA256", "MD5"]
    #[builder(into)]
    #[serde(rename = "sourceProvenanceHashes")]
    pub r#source_provenance_hashes: Vec<String>,
    /// Option to specify behavior when there is an error in the substitution checks.
    /// 
    /// NOTE this is always set to ALLOW_LOOSE for triggered builds and cannot be overridden
    /// in the build configuration file. Possible values: ["MUST_MATCH", "ALLOW_LOOSE"]
    #[builder(into)]
    #[serde(rename = "substitutionOption")]
    pub r#substitution_option: String,
    /// Global list of volumes to mount for ALL build steps
    /// 
    /// Each volume is created as an empty volume prior to starting the build process.
    /// Upon completion of the build, volumes and their contents are discarded. Global
    /// volume names and paths cannot conflict with the volumes defined a build step.
    /// 
    /// Using a global volume in a build with only one step is not valid as it is indicative
    /// of a build request with an incorrect configuration.
    #[builder(into)]
    #[serde(rename = "volumes")]
    pub r#volumes: Vec<super::super::types::cloudbuild::GetTriggerBuildOptionVolume>,
    /// Option to specify a WorkerPool for the build. Format projects/{project}/workerPools/{workerPool}
    /// 
    /// This field is experimental.
    #[builder(into)]
    #[serde(rename = "workerPool")]
    pub r#worker_pool: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetTriggerBuildOption {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "disk_size_gb",
                    &self.r#disk_size_gb,
                ),
                to_pulumi_object_field(
                    "dynamic_substitutions",
                    &self.r#dynamic_substitutions,
                ),
                to_pulumi_object_field(
                    "envs",
                    &self.r#envs,
                ),
                to_pulumi_object_field(
                    "log_streaming_option",
                    &self.r#log_streaming_option,
                ),
                to_pulumi_object_field(
                    "logging",
                    &self.r#logging,
                ),
                to_pulumi_object_field(
                    "machine_type",
                    &self.r#machine_type,
                ),
                to_pulumi_object_field(
                    "requested_verify_option",
                    &self.r#requested_verify_option,
                ),
                to_pulumi_object_field(
                    "secret_envs",
                    &self.r#secret_envs,
                ),
                to_pulumi_object_field(
                    "source_provenance_hashes",
                    &self.r#source_provenance_hashes,
                ),
                to_pulumi_object_field(
                    "substitution_option",
                    &self.r#substitution_option,
                ),
                to_pulumi_object_field(
                    "volumes",
                    &self.r#volumes,
                ),
                to_pulumi_object_field(
                    "worker_pool",
                    &self.r#worker_pool,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetTriggerBuildOption {
    fn from_pulumi_value(
        value: &pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    ) -> pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::rootcause::Result<Self> {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValueContent;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::rootcause::bail;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue;

        match value.content {
            PulumiValueContent::Object(ref _obj) => {
                use std::collections::BTreeMap;
                let fields_map: BTreeMap<String, PulumiValue> =
                    _obj.iter().cloned().collect();

                Ok(Self {
                    r#disk_size_gb: {
                        let field_value = match fields_map.get("disk_size_gb") {
                            Some(value) => value,
                            None => bail!("Missing field 'disk_size_gb' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#dynamic_substitutions: {
                        let field_value = match fields_map.get("dynamic_substitutions") {
                            Some(value) => value,
                            None => bail!("Missing field 'dynamic_substitutions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#envs: {
                        let field_value = match fields_map.get("envs") {
                            Some(value) => value,
                            None => bail!("Missing field 'envs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#log_streaming_option: {
                        let field_value = match fields_map.get("log_streaming_option") {
                            Some(value) => value,
                            None => bail!("Missing field 'log_streaming_option' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#logging: {
                        let field_value = match fields_map.get("logging") {
                            Some(value) => value,
                            None => bail!("Missing field 'logging' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#machine_type: {
                        let field_value = match fields_map.get("machine_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'machine_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#requested_verify_option: {
                        let field_value = match fields_map.get("requested_verify_option") {
                            Some(value) => value,
                            None => bail!("Missing field 'requested_verify_option' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#secret_envs: {
                        let field_value = match fields_map.get("secret_envs") {
                            Some(value) => value,
                            None => bail!("Missing field 'secret_envs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#source_provenance_hashes: {
                        let field_value = match fields_map.get("source_provenance_hashes") {
                            Some(value) => value,
                            None => bail!("Missing field 'source_provenance_hashes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#substitution_option: {
                        let field_value = match fields_map.get("substitution_option") {
                            Some(value) => value,
                            None => bail!("Missing field 'substitution_option' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#volumes: {
                        let field_value = match fields_map.get("volumes") {
                            Some(value) => value,
                            None => bail!("Missing field 'volumes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#worker_pool: {
                        let field_value = match fields_map.get("worker_pool") {
                            Some(value) => value,
                            None => bail!("Missing field 'worker_pool' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
