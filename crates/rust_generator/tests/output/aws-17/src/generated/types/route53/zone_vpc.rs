#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ZoneVpc {
    /// ID of the VPC to associate.
    #[builder(into)]
    #[serde(rename = "vpcId")]
    pub r#vpc_id: String,
    /// Region of the VPC to associate. Defaults to AWS provider region.
    #[builder(into)]
    #[serde(rename = "vpcRegion")]
    pub r#vpc_region: Option<String>,
}
