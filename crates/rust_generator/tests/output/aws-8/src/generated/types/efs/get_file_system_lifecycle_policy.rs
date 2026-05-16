#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetFileSystemLifecyclePolicy {
    #[builder(into)]
    #[serde(rename = "transitionToArchive")]
    pub r#transition_to_archive: String,
    #[builder(into)]
    #[serde(rename = "transitionToIa")]
    pub r#transition_to_ia: String,
    #[builder(into)]
    #[serde(rename = "transitionToPrimaryStorageClass")]
    pub r#transition_to_primary_storage_class: String,
}
