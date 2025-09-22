#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct LifecyclePolicyPolicyDetailExclusionRulesAmis {
    /// Configures whether public AMIs are excluded from the lifecycle action.
    #[builder(into)]
    #[serde(rename = "isPublic")]
    pub r#is_public: Option<bool>,
    /// Specifies configuration details for Image Builder to exclude the most recent resources from lifecycle actions. Detailed below.
    #[builder(into)]
    #[serde(rename = "lastLaunched")]
    pub r#last_launched: Option<Box<super::super::types::imagebuilder::LifecyclePolicyPolicyDetailExclusionRulesAmisLastLaunched>>,
    /// Configures AWS Regions that are excluded from the lifecycle action.
    #[builder(into)]
    #[serde(rename = "regions")]
    pub r#regions: Option<Vec<String>>,
    /// Specifies AWS accounts whose resources are excluded from the lifecycle action.
    #[builder(into)]
    #[serde(rename = "sharedAccounts")]
    pub r#shared_accounts: Option<Vec<String>>,
    /// Lists tags that should be excluded from lifecycle actions for the AMIs that have them.
    #[builder(into)]
    #[serde(rename = "tagMap")]
    pub r#tag_map: Option<std::collections::HashMap<String, String>>,
}
