#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetUserPoolAdminCreateUserConfig {
    /// - Whether only admins can create users.
    #[builder(into)]
    #[serde(rename = "allowAdminCreateUserOnly")]
    pub r#allow_admin_create_user_only: bool,
    #[builder(into)]
    #[serde(rename = "inviteMessageTemplates")]
    pub r#invite_message_templates: Vec<super::super::types::cognito::GetUserPoolAdminCreateUserConfigInviteMessageTemplate>,
    /// - Number of days an unconfirmed user account remains valid.
    /// * invite_message_template - Templates for invitation messages.
    #[builder(into)]
    #[serde(rename = "unusedAccountValidityDays")]
    pub r#unused_account_validity_days: i32,
}
