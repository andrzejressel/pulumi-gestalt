#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct SparkClusterComputeIsolation {
    /// This field indicates whether enable compute isolation or not. Possible values are `true` or `false`.
    #[builder(into)]
    #[serde(rename = "computeIsolationEnabled")]
    pub r#compute_isolation_enabled: Option<bool>,
    /// The name of the host SKU.
    #[builder(into)]
    #[serde(rename = "hostSku")]
    pub r#host_sku: Option<String>,
}
