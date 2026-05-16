#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct CacheDefaultAccessPolicy {
    /// One or more `access_rule` blocks (up to three) as defined above.
    #[builder(into)]
    #[serde(rename = "accessRules")]
    pub r#access_rules: Vec<super::super::types::hpc::CacheDefaultAccessPolicyAccessRule>,
}
