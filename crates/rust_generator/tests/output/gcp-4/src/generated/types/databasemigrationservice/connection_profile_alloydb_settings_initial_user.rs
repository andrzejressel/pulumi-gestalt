#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ConnectionProfileAlloydbSettingsInitialUser {
    /// The initial password for the user.
    /// **Note**: This property is sensitive and will not be displayed in the plan.
    #[builder(into)]
    #[serde(rename = "password")]
    pub r#password: String,
    /// (Output)
    /// Output only. Indicates if the initialUser.password field has been set.
    #[builder(into)]
    #[serde(rename = "passwordSet")]
    pub r#password_set: Option<bool>,
    /// The database username.
    #[builder(into)]
    #[serde(rename = "user")]
    pub r#user: String,
}
