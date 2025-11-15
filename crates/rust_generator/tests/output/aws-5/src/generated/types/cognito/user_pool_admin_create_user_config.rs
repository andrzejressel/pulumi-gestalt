#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct UserPoolAdminCreateUserConfig {
    /// Set to True if only the administrator is allowed to create user profiles. Set to False if users can sign themselves up via an app.
    #[builder(into)]
    #[serde(rename = "allowAdminCreateUserOnly")]
    pub r#allow_admin_create_user_only: Option<bool>,
    /// Invite message template structure. Detailed below.
    #[builder(into)]
    #[serde(rename = "inviteMessageTemplate")]
    pub r#invite_message_template: Option<Box<super::super::types::cognito::UserPoolAdminCreateUserConfigInviteMessageTemplate>>,
}
