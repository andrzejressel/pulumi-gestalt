#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetBackendServiceConsistentHashHttpCooky {
    /// The name of the Backend Service.
    /// 
    /// - - -
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// Path to set for the cookie.
    #[builder(into)]
    #[serde(rename = "path")]
    pub r#path: String,
    /// Lifetime of the cookie.
    #[builder(into)]
    #[serde(rename = "ttls")]
    pub r#ttls: Vec<super::super::types::compute::GetBackendServiceConsistentHashHttpCookyTtl>,
}
