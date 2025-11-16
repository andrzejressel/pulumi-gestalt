#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ServiceVolumeConfigurationManagedEbsVolumeTagSpecification {
    /// Determines whether to propagate the tags from the task definition to the Amazon EBS volume.
    #[builder(into)]
    #[serde(rename = "propagateTags")]
    pub r#propagate_tags: Option<String>,
    /// The type of volume resource. Valid values, `volume`.
    #[builder(into)]
    #[serde(rename = "resourceType")]
    pub r#resource_type: String,
    /// The tags applied to this Amazon EBS volume. `AmazonECSCreated` and `AmazonECSManaged` are reserved tags that can't be used.
    #[builder(into)]
    #[serde(rename = "tags")]
    pub r#tags: Option<std::collections::HashMap<String, String>>,
}
