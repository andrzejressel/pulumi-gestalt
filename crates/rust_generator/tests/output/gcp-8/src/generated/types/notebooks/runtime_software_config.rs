#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
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
