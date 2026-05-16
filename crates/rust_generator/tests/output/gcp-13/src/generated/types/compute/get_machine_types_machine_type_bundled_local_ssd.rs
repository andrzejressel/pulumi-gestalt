#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetMachineTypesMachineTypeBundledLocalSsd {
    /// The default disk interface if the interface is not specified.
    #[builder(into)]
    #[serde(rename = "defaultInterface")]
    pub r#default_interface: String,
    /// The number of partitions.
    #[builder(into)]
    #[serde(rename = "partitionCount")]
    pub r#partition_count: i32,
}
