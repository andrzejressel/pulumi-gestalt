#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ApplicationApplicationConfigurationFlinkApplicationConfigurationParallelismConfiguration {
    /// Describes whether the Kinesis Data Analytics service can increase the parallelism of the application in response to increased throughput.
    #[builder(into)]
    #[serde(rename = "autoScalingEnabled")]
    pub r#auto_scaling_enabled: Option<bool>,
    /// Describes whether the application uses the default parallelism for the Kinesis Data Analytics service. Valid values: `CUSTOM`, `DEFAULT`. Set this attribute to `CUSTOM` in order for any specified `auto_scaling_enabled`, `parallelism`, or `parallelism_per_kpu` attribute values to be effective.
    #[builder(into)]
    #[serde(rename = "configurationType")]
    pub r#configuration_type: String,
    /// Describes the initial number of parallel tasks that a Flink-based Kinesis Data Analytics application can perform.
    #[builder(into)]
    #[serde(rename = "parallelism")]
    pub r#parallelism: Option<i32>,
    /// Describes the number of parallel tasks that a Flink-based Kinesis Data Analytics application can perform per Kinesis Processing Unit (KPU) used by the application.
    #[builder(into)]
    #[serde(rename = "parallelismPerKpu")]
    pub r#parallelism_per_kpu: Option<i32>,
}
