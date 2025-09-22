#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct RepositoryAssociationS3RepositoryDetailCodeArtifact {
    #[builder(into)]
    #[serde(rename = "buildArtifactsObjectKey")]
    pub r#build_artifacts_object_key: Option<String>,
    #[builder(into)]
    #[serde(rename = "sourceCodeArtifactsObjectKey")]
    pub r#source_code_artifacts_object_key: Option<String>,
}
