#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetTableAclAccessPolicy {
    #[builder(into)]
    #[serde(rename = "expiry")]
    pub r#expiry: String,
    #[builder(into)]
    #[serde(rename = "permissions")]
    pub r#permissions: String,
    #[builder(into)]
    #[serde(rename = "start")]
    pub r#start: String,
}
