#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct LaunchTemplateMetadataOptions {
    /// Whether the metadata service is available. Can be `"enabled"` or `"disabled"`. (Default: `"enabled"`).
    #[builder(into)]
    #[serde(rename = "httpEndpoint")]
    pub r#http_endpoint: Option<String>,
    /// Enables or disables the IPv6 endpoint for the instance metadata service. Can be `"enabled"` or `"disabled"`.
    #[builder(into)]
    #[serde(rename = "httpProtocolIpv6")]
    pub r#http_protocol_ipv_6: Option<String>,
    /// The desired HTTP PUT response hop limit for instance metadata requests. The larger the number, the further instance metadata requests can travel. Can be an integer from `1` to `64`. (Default: `1`).
    #[builder(into)]
    #[serde(rename = "httpPutResponseHopLimit")]
    pub r#http_put_response_hop_limit: Option<i32>,
    /// Whether or not the metadata service requires session tokens, also referred to as _Instance Metadata Service Version 2 (IMDSv2)_. Can be `"optional"` or `"required"`. (Default: `"optional"`).
    #[builder(into)]
    #[serde(rename = "httpTokens")]
    pub r#http_tokens: Option<String>,
    /// Enables or disables access to instance tags from the instance metadata service. Can be `"enabled"` or `"disabled"`.
    /// 
    /// For more information, see the documentation on the [Instance Metadata Service](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ec2-instance-metadata.html).
    #[builder(into)]
    #[serde(rename = "instanceMetadataTags")]
    pub r#instance_metadata_tags: Option<String>,
}
