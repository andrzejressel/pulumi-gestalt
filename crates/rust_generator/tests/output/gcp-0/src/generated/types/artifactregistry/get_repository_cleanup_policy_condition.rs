#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetRepositoryCleanupPolicyCondition {
    /// Match versions newer than a duration.
    #[builder(into)]
    #[serde(rename = "newerThan")]
    pub r#newer_than: String,
    /// Match versions older than a duration.
    #[builder(into)]
    #[serde(rename = "olderThan")]
    pub r#older_than: String,
    /// Match versions by package prefix. Applied on any prefix match.
    #[builder(into)]
    #[serde(rename = "packageNamePrefixes")]
    pub r#package_name_prefixes: Vec<String>,
    /// Match versions by tag prefix. Applied on any prefix match.
    #[builder(into)]
    #[serde(rename = "tagPrefixes")]
    pub r#tag_prefixes: Vec<String>,
    /// Match versions by tag status. Default value: "ANY" Possible values: ["TAGGED", "UNTAGGED", "ANY"]
    #[builder(into)]
    #[serde(rename = "tagState")]
    pub r#tag_state: String,
    /// Match versions by version name prefix. Applied on any prefix match.
    #[builder(into)]
    #[serde(rename = "versionNamePrefixes")]
    pub r#version_name_prefixes: Vec<String>,
}
