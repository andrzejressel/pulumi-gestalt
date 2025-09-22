#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct FirehoseDeliveryStreamSnowflakeConfigurationSnowflakeVpcConfiguration {
    /// The VPCE ID for Firehose to privately connect with Snowflake.
    #[builder(into)]
    #[serde(rename = "privateLinkVpceId")]
    pub r#private_link_vpce_id: String,
}
