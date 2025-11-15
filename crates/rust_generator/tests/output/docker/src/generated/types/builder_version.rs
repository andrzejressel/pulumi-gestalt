#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, Debug, PartialEq, Clone)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub enum BuilderVersion {
    /// The first generation builder for Docker Daemon
    #[serde(rename = "BuilderV1")]
    BuilderV1,
    /// The builder based on moby/buildkit project
    #[serde(rename = "BuilderBuildKit")]
    BuilderBuildKit,
}
