#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct OsPolicyAssignmentInstanceFilterInventory {
    /// The OS short name
    #[builder(into)]
    #[serde(rename = "osShortName")]
    pub r#os_short_name: String,
    /// The OS version Prefix matches are supported if
    /// asterisk(*) is provided as the last character. For example, to match all
    /// versions with a major version of `7`, specify the following value for this
    /// field `7.*` An empty string matches all OS versions.
    #[builder(into)]
    #[serde(rename = "osVersion")]
    pub r#os_version: Option<String>,
}
