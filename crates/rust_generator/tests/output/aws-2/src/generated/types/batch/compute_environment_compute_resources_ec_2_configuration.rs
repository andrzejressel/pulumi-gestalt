#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ComputeEnvironmentComputeResourcesEc2Configuration {
    /// The AMI ID used for instances launched in the compute environment that match the image type. This setting overrides the `image_id` argument in the `compute_resources` block.
    #[builder(into)]
    #[serde(rename = "imageIdOverride")]
    pub r#image_id_override: Option<String>,
    /// The image type to match with the instance type to select an AMI. If the `image_id_override` parameter isn't specified, then a recent [Amazon ECS-optimized Amazon Linux 2 AMI](https://docs.aws.amazon.com/AmazonECS/latest/developerguide/ecs-optimized_AMI.html#al2ami) (`ECS_AL2`) is used.
    #[builder(into)]
    #[serde(rename = "imageType")]
    pub r#image_type: Option<String>,
}
