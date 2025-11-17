#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DistributionOriginVpcOriginConfig {
    #[builder(into)]
    #[serde(rename = "originKeepaliveTimeout")]
    pub r#origin_keepalive_timeout: Option<i32>,
    #[builder(into)]
    #[serde(rename = "originReadTimeout")]
    pub r#origin_read_timeout: Option<i32>,
    /// The VPC origin ID.
    #[builder(into)]
    #[serde(rename = "vpcOriginId")]
    pub r#vpc_origin_id: String,
}
