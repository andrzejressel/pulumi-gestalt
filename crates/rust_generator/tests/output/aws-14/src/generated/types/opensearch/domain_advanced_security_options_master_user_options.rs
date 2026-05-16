#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DomainAdvancedSecurityOptionsMasterUserOptions {
    /// ARN for the main user. Only specify if `internal_user_database_enabled` is not set or set to `false`.
    #[builder(into)]
    #[serde(rename = "masterUserArn")]
    pub r#master_user_arn: Option<String>,
    /// Main user's username, which is stored in the Amazon OpenSearch Service domain's internal database. Only specify if `internal_user_database_enabled` is set to `true`.
    #[builder(into)]
    #[serde(rename = "masterUserName")]
    pub r#master_user_name: Option<String>,
    /// Main user's password, which is stored in the Amazon OpenSearch Service domain's internal database. Only specify if `internal_user_database_enabled` is set to `true`.
    #[builder(into)]
    #[serde(rename = "masterUserPassword")]
    pub r#master_user_password: Option<String>,
}
