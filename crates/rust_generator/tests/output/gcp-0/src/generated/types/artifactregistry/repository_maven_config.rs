#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RepositoryMavenConfig {
    /// The repository with this flag will allow publishing the same
    /// snapshot versions.
    #[builder(into)]
    #[serde(rename = "allowSnapshotOverwrites")]
    pub r#allow_snapshot_overwrites: Option<bool>,
    /// Version policy defines the versions that the registry will accept.
    /// Default value is `VERSION_POLICY_UNSPECIFIED`.
    /// Possible values are: `VERSION_POLICY_UNSPECIFIED`, `RELEASE`, `SNAPSHOT`.
    #[builder(into)]
    #[serde(rename = "versionPolicy")]
    pub r#version_policy: Option<String>,
}
