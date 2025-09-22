#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct DataSetRowLevelPermissionDataSet {
    /// ARN of the dataset that contains permissions for RLS.
    #[builder(into)]
    #[serde(rename = "arn")]
    pub r#arn: String,
    /// User or group rules associated with the dataset that contains permissions for RLS.
    #[builder(into)]
    #[serde(rename = "formatVersion")]
    pub r#format_version: Option<String>,
    /// Namespace associated with the dataset that contains permissions for RLS.
    #[builder(into)]
    #[serde(rename = "namespace")]
    pub r#namespace: Option<String>,
    /// Type of permissions to use when interpreting the permissions for RLS. Valid values are `GRANT_ACCESS` and `DENY_ACCESS`.
    #[builder(into)]
    #[serde(rename = "permissionPolicy")]
    pub r#permission_policy: String,
    /// Status of the row-level security permission dataset. If enabled, the status is `ENABLED`. If disabled, the status is `DISABLED`.
    #[builder(into)]
    #[serde(rename = "status")]
    pub r#status: Option<String>,
}
