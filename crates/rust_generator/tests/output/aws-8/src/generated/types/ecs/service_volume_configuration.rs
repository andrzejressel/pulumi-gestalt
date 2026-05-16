#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ServiceVolumeConfiguration {
    /// Configuration for the Amazon EBS volume that Amazon ECS creates and manages on your behalf. See below.
    #[builder(into)]
    #[serde(rename = "managedEbsVolume")]
    pub r#managed_ebs_volume: Box<super::super::types::ecs::ServiceVolumeConfigurationManagedEbsVolume>,
    /// Name of the volume.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
}
