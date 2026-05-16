#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ClusterNodeConfigSoleTenantConfigNodeAffinity {
    /// The default or custom node affinity label key name.
    #[builder(into)]
    #[serde(rename = "key")]
    pub r#key: String,
    /// Specifies affinity or anti-affinity. Accepted values are `"IN"` or `"NOT_IN"`
    #[builder(into)]
    #[serde(rename = "operator")]
    pub r#operator: String,
    /// List of node affinity label values as strings.
    #[builder(into)]
    #[serde(rename = "values")]
    pub r#values: Vec<String>,
}
