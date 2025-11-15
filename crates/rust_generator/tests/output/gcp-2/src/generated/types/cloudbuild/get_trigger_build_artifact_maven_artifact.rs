#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetTriggerBuildArtifactMavenArtifact {
    /// Maven artifactId value used when uploading the artifact to Artifact Registry.
    #[builder(into)]
    #[serde(rename = "artifactId")]
    pub r#artifact_id: String,
    /// Maven groupId value used when uploading the artifact to Artifact Registry.
    #[builder(into)]
    #[serde(rename = "groupId")]
    pub r#group_id: String,
    /// Path to an artifact in the build's workspace to be uploaded to Artifact Registry. This can be either an absolute path, e.g. /workspace/my-app/target/my-app-1.0.SNAPSHOT.jar or a relative path from /workspace, e.g. my-app/target/my-app-1.0.SNAPSHOT.jar.
    #[builder(into)]
    #[serde(rename = "path")]
    pub r#path: String,
    /// Artifact Registry repository, in the form "https://$REGION-maven.pkg.dev/$PROJECT/$REPOSITORY"
    /// 
    /// Artifact in the workspace specified by path will be uploaded to Artifact Registry with this location as a prefix.
    #[builder(into)]
    #[serde(rename = "repository")]
    pub r#repository: String,
    /// Maven version value used when uploading the artifact to Artifact Registry.
    #[builder(into)]
    #[serde(rename = "version")]
    pub r#version: String,
}
