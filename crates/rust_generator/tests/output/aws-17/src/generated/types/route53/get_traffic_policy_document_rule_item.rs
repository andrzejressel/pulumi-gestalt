#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetTrafficPolicyDocumentRuleItem {
    #[builder(into)]
    #[serde(rename = "endpointReference")]
    pub r#endpoint_reference: Option<String>,
    #[builder(into)]
    #[serde(rename = "healthCheck")]
    pub r#health_check: Option<String>,
}
