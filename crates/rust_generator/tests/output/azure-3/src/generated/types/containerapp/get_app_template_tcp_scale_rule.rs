#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetAppTemplateTcpScaleRule {
    #[builder(into)]
    #[serde(rename = "authentications")]
    pub r#authentications: Box<Vec<super::super::types::containerapp::GetAppTemplateTcpScaleRuleAuthentication>>,
    #[builder(into)]
    #[serde(rename = "concurrentRequests")]
    pub r#concurrent_requests: Box<String>,
    /// The name of the Container App.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
}
