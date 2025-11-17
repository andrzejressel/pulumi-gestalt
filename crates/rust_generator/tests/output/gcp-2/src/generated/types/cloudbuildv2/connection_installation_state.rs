#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ConnectionInstallationState {
    /// (Output)
    /// Output only. Link to follow for next action. Empty string if the installation is already complete.
    #[builder(into)]
    #[serde(rename = "actionUri")]
    pub r#action_uri: Option<String>,
    /// (Output)
    /// Output only. Message of what the user should do next to continue the installation. Empty string if the installation is already complete.
    #[builder(into)]
    #[serde(rename = "message")]
    pub r#message: Option<String>,
    /// (Output)
    /// Output only. Current step of the installation process.
    #[builder(into)]
    #[serde(rename = "stage")]
    pub r#stage: Option<String>,
}
