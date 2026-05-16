#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct JobTemplateVolume {
    /// The name of the volume.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// The name of the storage to use for the volume.
    #[builder(into)]
    #[serde(rename = "storageName")]
    pub r#storage_name: Option<String>,
    /// The type of storage to use for the volume. Possible values are `AzureFile`, `EmptyDir` and `Secret`.
    #[builder(into)]
    #[serde(rename = "storageType")]
    pub r#storage_type: Option<String>,
}
