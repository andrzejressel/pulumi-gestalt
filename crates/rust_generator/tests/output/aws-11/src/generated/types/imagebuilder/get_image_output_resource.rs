#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetImageOutputResource {
    /// Set of objects with each Amazon Machine Image (AMI) created.
    #[builder(into)]
    #[serde(rename = "amis")]
    pub r#amis: Vec<super::super::types::imagebuilder::GetImageOutputResourceAmi>,
    /// Set of objects with each container image created and stored in the output repository.
    #[builder(into)]
    #[serde(rename = "containers")]
    pub r#containers: Vec<super::super::types::imagebuilder::GetImageOutputResourceContainer>,
}
