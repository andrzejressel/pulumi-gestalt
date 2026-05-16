#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ActivityLogAlertAction {
    /// The ID of the Action Group can be sourced from the `azure.monitoring.ActionGroup` resource.
    #[builder(into)]
    #[serde(rename = "actionGroupId")]
    pub r#action_group_id: String,
    /// The map of custom string properties to include with the post operation. These data are appended to the webhook payload.
    #[builder(into)]
    #[serde(rename = "webhookProperties")]
    pub r#webhook_properties: Option<std::collections::HashMap<String, String>>,
}
