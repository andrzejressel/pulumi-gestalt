#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, Debug, PartialEq, Clone)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub enum BuilderVersion {
    /// The first generation builder for Docker Daemon
    BuilderV1,
    /// The builder based on moby/buildkit project
    BuilderBuildKit,
}
