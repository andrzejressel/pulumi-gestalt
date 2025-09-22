#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetDataSetRowLevelPermissionDataSet {
    #[builder(into)]
    #[serde(rename = "arn")]
    pub r#arn: String,
    #[builder(into)]
    #[serde(rename = "formatVersion")]
    pub r#format_version: String,
    #[builder(into)]
    #[serde(rename = "namespace")]
    pub r#namespace: String,
    #[builder(into)]
    #[serde(rename = "permissionPolicy")]
    pub r#permission_policy: String,
    #[builder(into)]
    #[serde(rename = "status")]
    pub r#status: String,
}
