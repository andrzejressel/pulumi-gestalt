#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct LinkLinkConfiguration {
    /// Configuration for filtering which log groups are to send log events from the source account to the monitoring account. See `log_group_configuration` Block for details.
    #[builder(into)]
    #[serde(rename = "logGroupConfiguration")]
    pub r#log_group_configuration: Option<Box<super::super::types::oam::LinkLinkConfigurationLogGroupConfiguration>>,
    /// Configuration for filtering which metric namespaces are to be shared from the source account to the monitoring account. See `metric_configuration` Block for details.
    #[builder(into)]
    #[serde(rename = "metricConfiguration")]
    pub r#metric_configuration: Option<Box<super::super::types::oam::LinkLinkConfigurationMetricConfiguration>>,
}
