#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct TriggerBuildOptions {
    /// Requested disk size for the VM that runs the build. Note that this is NOT "disk free";
    /// some of the space will be used by the operating system and build utilities.
    /// Also note that this is the minimum disk size that will be allocated for the build --
    /// the build may run with a larger disk than requested. At present, the maximum disk size
    /// is 1000GB; builds that request more than the maximum are rejected with an error.
    #[builder(into)]
    #[serde(rename = "diskSizeGb")]
    pub r#disk_size_gb: Option<i32>,
    /// Option to specify whether or not to apply bash style string operations to the substitutions.
    /// NOTE this is always enabled for triggered builds and cannot be overridden in the build configuration file.
    #[builder(into)]
    #[serde(rename = "dynamicSubstitutions")]
    pub r#dynamic_substitutions: Option<bool>,
    /// A list of global environment variable definitions that will exist for all build steps
    /// in this build. If a variable is defined in both globally and in a build step,
    /// the variable will use the build step value.
    /// The elements are of the form "KEY=VALUE" for the environment variable "KEY" being given the value "VALUE".
    #[builder(into)]
    #[serde(rename = "envs")]
    pub r#envs: Option<Vec<String>>,
    /// Option to define build log streaming behavior to Google Cloud Storage.
    /// Possible values are: `STREAM_DEFAULT`, `STREAM_ON`, `STREAM_OFF`.
    #[builder(into)]
    #[serde(rename = "logStreamingOption")]
    pub r#log_streaming_option: Option<String>,
    /// Option to specify the logging mode, which determines if and where build logs are stored.
    /// Possible values are: `LOGGING_UNSPECIFIED`, `LEGACY`, `GCS_ONLY`, `STACKDRIVER_ONLY`, `CLOUD_LOGGING_ONLY`, `NONE`.
    #[builder(into)]
    #[serde(rename = "logging")]
    pub r#logging: Option<String>,
    /// Compute Engine machine type on which to run the build.
    #[builder(into)]
    #[serde(rename = "machineType")]
    pub r#machine_type: Option<String>,
    /// Requested verifiability options.
    /// Possible values are: `NOT_VERIFIED`, `VERIFIED`.
    #[builder(into)]
    #[serde(rename = "requestedVerifyOption")]
    pub r#requested_verify_option: Option<String>,
    /// A list of global environment variables, which are encrypted using a Cloud Key Management
    /// Service crypto key. These values must be specified in the build's Secret. These variables
    /// will be available to all build steps in this build.
    #[builder(into)]
    #[serde(rename = "secretEnvs")]
    pub r#secret_envs: Option<Vec<String>>,
    /// Requested hash for SourceProvenance.
    /// Each value may be one of: `NONE`, `SHA256`, `MD5`.
    #[builder(into)]
    #[serde(rename = "sourceProvenanceHashes")]
    pub r#source_provenance_hashes: Option<Vec<String>>,
    /// Option to specify behavior when there is an error in the substitution checks.
    /// NOTE this is always set to ALLOW_LOOSE for triggered builds and cannot be overridden
    /// in the build configuration file.
    /// Possible values are: `MUST_MATCH`, `ALLOW_LOOSE`.
    #[builder(into)]
    #[serde(rename = "substitutionOption")]
    pub r#substitution_option: Option<String>,
    /// Global list of volumes to mount for ALL build steps
    /// Each volume is created as an empty volume prior to starting the build process.
    /// Upon completion of the build, volumes and their contents are discarded. Global
    /// volume names and paths cannot conflict with the volumes defined a build step.
    /// Using a global volume in a build with only one step is not valid as it is indicative
    /// of a build request with an incorrect configuration.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "volumes")]
    pub r#volumes: Option<Vec<super::super::types::cloudbuild::TriggerBuildOptionsVolume>>,
    /// Option to specify a WorkerPool for the build. Format projects/{project}/workerPools/{workerPool}
    /// This field is experimental.
    #[builder(into)]
    #[serde(rename = "workerPool")]
    pub r#worker_pool: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for TriggerBuildOptions {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > + Send {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "diskSizeGb",
                    &self.r#disk_size_gb,
                ),
                to_pulumi_object_field(
                    "dynamicSubstitutions",
                    &self.r#dynamic_substitutions,
                ),
                to_pulumi_object_field(
                    "envs",
                    &self.r#envs,
                ),
                to_pulumi_object_field(
                    "logStreamingOption",
                    &self.r#log_streaming_option,
                ),
                to_pulumi_object_field(
                    "logging",
                    &self.r#logging,
                ),
                to_pulumi_object_field(
                    "machineType",
                    &self.r#machine_type,
                ),
                to_pulumi_object_field(
                    "requestedVerifyOption",
                    &self.r#requested_verify_option,
                ),
                to_pulumi_object_field(
                    "secretEnvs",
                    &self.r#secret_envs,
                ),
                to_pulumi_object_field(
                    "sourceProvenanceHashes",
                    &self.r#source_provenance_hashes,
                ),
                to_pulumi_object_field(
                    "substitutionOption",
                    &self.r#substitution_option,
                ),
                to_pulumi_object_field(
                    "volumes",
                    &self.r#volumes,
                ),
                to_pulumi_object_field(
                    "workerPool",
                    &self.r#worker_pool,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for TriggerBuildOptions {
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
                        let field_value = match fields_map.get("diskSizeGb") {
                            Some(value) => value,
                            None => bail!("Missing field 'diskSizeGb' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#dynamic_substitutions: {
                        let field_value = match fields_map.get("dynamicSubstitutions") {
                            Some(value) => value,
                            None => bail!("Missing field 'dynamicSubstitutions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("logStreamingOption") {
                            Some(value) => value,
                            None => bail!("Missing field 'logStreamingOption' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("machineType") {
                            Some(value) => value,
                            None => bail!("Missing field 'machineType' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#requested_verify_option: {
                        let field_value = match fields_map.get("requestedVerifyOption") {
                            Some(value) => value,
                            None => bail!("Missing field 'requestedVerifyOption' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#secret_envs: {
                        let field_value = match fields_map.get("secretEnvs") {
                            Some(value) => value,
                            None => bail!("Missing field 'secretEnvs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#source_provenance_hashes: {
                        let field_value = match fields_map.get("sourceProvenanceHashes") {
                            Some(value) => value,
                            None => bail!("Missing field 'sourceProvenanceHashes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#substitution_option: {
                        let field_value = match fields_map.get("substitutionOption") {
                            Some(value) => value,
                            None => bail!("Missing field 'substitutionOption' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("workerPool") {
                            Some(value) => value,
                            None => bail!("Missing field 'workerPool' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
