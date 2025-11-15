#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AwsNodePoolConfigInstancePlacement {
    /// The tenancy for the instance. Possible values: TENANCY_UNSPECIFIED, DEFAULT, DEDICATED, HOST
    #[builder(into)]
    #[serde(rename = "tenancy")]
    pub r#tenancy: Option<String>,
}
