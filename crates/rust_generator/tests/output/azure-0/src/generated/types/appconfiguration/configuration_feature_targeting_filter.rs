#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ConfigurationFeatureTargetingFilter {
    /// A number representing the percentage of the entire user base.
    #[builder(into)]
    #[serde(rename = "defaultRolloutPercentage")]
    pub r#default_rollout_percentage: Box<i32>,
    /// One or more `groups` blocks as defined below.
    #[builder(into, default)]
    #[serde(rename = "groups")]
    pub r#groups: Box<Option<Vec<super::super::types::appconfiguration::ConfigurationFeatureTargetingFilterGroup>>>,
    /// A list of users to target for this feature.
    #[builder(into, default)]
    #[serde(rename = "users")]
    pub r#users: Box<Option<Vec<String>>>,
}
