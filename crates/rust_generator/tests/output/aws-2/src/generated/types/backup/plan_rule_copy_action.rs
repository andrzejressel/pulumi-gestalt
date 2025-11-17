#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PlanRuleCopyAction {
    /// An Amazon Resource Name (ARN) that uniquely identifies the destination backup vault for the copied backup.
    #[builder(into)]
    #[serde(rename = "destinationVaultArn")]
    pub r#destination_vault_arn: String,
    /// The lifecycle defines when a protected resource is copied over to a backup vault and when it expires.  Fields documented above.
    #[builder(into)]
    #[serde(rename = "lifecycle")]
    pub r#lifecycle: Option<Box<super::super::types::backup::PlanRuleCopyActionLifecycle>>,
}
