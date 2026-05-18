#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RuntimeSoftwareConfig {
    /// Specify a custom Cloud Storage path where the GPU driver is stored.
    /// If not specified, we'll automatically choose from official GPU drivers.
    #[builder(into)]
    #[serde(rename = "customGpuDriverPath")]
    pub r#custom_gpu_driver_path: Option<String>,
    /// Verifies core internal services are running. Default: True.
    #[builder(into)]
    #[serde(rename = "enableHealthMonitoring")]
    pub r#enable_health_monitoring: Option<bool>,
    /// Runtime will automatically shutdown after idle_shutdown_time.
    /// Default: True
    #[builder(into)]
    #[serde(rename = "idleShutdown")]
    pub r#idle_shutdown: Option<bool>,
    /// Time in minutes to wait before shuting down runtime.
    /// Default: 180 minutes
    #[builder(into)]
    #[serde(rename = "idleShutdownTimeout")]
    pub r#idle_shutdown_timeout: Option<i32>,
    /// Install Nvidia Driver automatically.
    #[builder(into)]
    #[serde(rename = "installGpuDriver")]
    pub r#install_gpu_driver: Option<bool>,
    /// Use a list of container images to use as Kernels in the notebook instance.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "kernels")]
    pub r#kernels: Option<Vec<super::super::types::notebooks::RuntimeSoftwareConfigKernel>>,
    /// Cron expression in UTC timezone for schedule instance auto upgrade.
    /// Please follow the [cron format](https://en.wikipedia.org/wiki/Cron).
    #[builder(into)]
    #[serde(rename = "notebookUpgradeSchedule")]
    pub r#notebook_upgrade_schedule: Option<String>,
    /// Path to a Bash script that automatically runs after a notebook instance
    /// fully boots up. The path must be a URL or
    /// Cloud Storage path (gs://path-to-file/file-name).
    #[builder(into)]
    #[serde(rename = "postStartupScript")]
    pub r#post_startup_script: Option<String>,
    /// Behavior for the post startup script.
    /// Possible values are: `POST_STARTUP_SCRIPT_BEHAVIOR_UNSPECIFIED`, `RUN_EVERY_START`, `DOWNLOAD_AND_RUN_EVERY_START`.
    #[builder(into)]
    #[serde(rename = "postStartupScriptBehavior")]
    pub r#post_startup_script_behavior: Option<String>,
    /// (Output)
    /// Bool indicating whether an newer image is available in an image family.
    #[builder(into)]
    #[serde(rename = "upgradeable")]
    pub r#upgradeable: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for RuntimeSoftwareConfig {
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
                    "custom_gpu_driver_path",
                    &self.r#custom_gpu_driver_path,
                ),
                to_pulumi_object_field(
                    "enable_health_monitoring",
                    &self.r#enable_health_monitoring,
                ),
                to_pulumi_object_field(
                    "idle_shutdown",
                    &self.r#idle_shutdown,
                ),
                to_pulumi_object_field(
                    "idle_shutdown_timeout",
                    &self.r#idle_shutdown_timeout,
                ),
                to_pulumi_object_field(
                    "install_gpu_driver",
                    &self.r#install_gpu_driver,
                ),
                to_pulumi_object_field(
                    "kernels",
                    &self.r#kernels,
                ),
                to_pulumi_object_field(
                    "notebook_upgrade_schedule",
                    &self.r#notebook_upgrade_schedule,
                ),
                to_pulumi_object_field(
                    "post_startup_script",
                    &self.r#post_startup_script,
                ),
                to_pulumi_object_field(
                    "post_startup_script_behavior",
                    &self.r#post_startup_script_behavior,
                ),
                to_pulumi_object_field(
                    "upgradeable",
                    &self.r#upgradeable,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for RuntimeSoftwareConfig {
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
                    r#custom_gpu_driver_path: {
                        let field_value = match fields_map.get("custom_gpu_driver_path") {
                            Some(value) => value,
                            None => bail!("Missing field 'custom_gpu_driver_path' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#enable_health_monitoring: {
                        let field_value = match fields_map.get("enable_health_monitoring") {
                            Some(value) => value,
                            None => bail!("Missing field 'enable_health_monitoring' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#idle_shutdown: {
                        let field_value = match fields_map.get("idle_shutdown") {
                            Some(value) => value,
                            None => bail!("Missing field 'idle_shutdown' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#idle_shutdown_timeout: {
                        let field_value = match fields_map.get("idle_shutdown_timeout") {
                            Some(value) => value,
                            None => bail!("Missing field 'idle_shutdown_timeout' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#install_gpu_driver: {
                        let field_value = match fields_map.get("install_gpu_driver") {
                            Some(value) => value,
                            None => bail!("Missing field 'install_gpu_driver' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#kernels: {
                        let field_value = match fields_map.get("kernels") {
                            Some(value) => value,
                            None => bail!("Missing field 'kernels' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#notebook_upgrade_schedule: {
                        let field_value = match fields_map.get("notebook_upgrade_schedule") {
                            Some(value) => value,
                            None => bail!("Missing field 'notebook_upgrade_schedule' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#post_startup_script: {
                        let field_value = match fields_map.get("post_startup_script") {
                            Some(value) => value,
                            None => bail!("Missing field 'post_startup_script' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#post_startup_script_behavior: {
                        let field_value = match fields_map.get("post_startup_script_behavior") {
                            Some(value) => value,
                            None => bail!("Missing field 'post_startup_script_behavior' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#upgradeable: {
                        let field_value = match fields_map.get("upgradeable") {
                            Some(value) => value,
                            None => bail!("Missing field 'upgradeable' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
