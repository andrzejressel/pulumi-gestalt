#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ClusterOpenMonitoringPrometheus {
    /// Configuration block for JMX Exporter. See below.
    #[builder(into)]
    #[serde(rename = "jmxExporter")]
    pub r#jmx_exporter: Option<Box<super::super::types::msk::ClusterOpenMonitoringPrometheusJmxExporter>>,
    /// Configuration block for Node Exporter. See below.
    #[builder(into)]
    #[serde(rename = "nodeExporter")]
    pub r#node_exporter: Option<Box<super::super::types::msk::ClusterOpenMonitoringPrometheusNodeExporter>>,
}
