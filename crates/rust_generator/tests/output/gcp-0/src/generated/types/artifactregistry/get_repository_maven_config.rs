#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetRepositoryMavenConfig {
    /// The repository with this flag will allow publishing the same
    /// snapshot versions.
    #[builder(into)]
    #[serde(rename = "allowSnapshotOverwrites")]
    pub r#allow_snapshot_overwrites: bool,
    /// Version policy defines the versions that the registry will accept. Default value: "VERSION_POLICY_UNSPECIFIED" Possible values: ["VERSION_POLICY_UNSPECIFIED", "RELEASE", "SNAPSHOT"]
    #[builder(into)]
    #[serde(rename = "versionPolicy")]
    pub r#version_policy: String,
}
