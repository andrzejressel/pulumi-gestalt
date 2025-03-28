#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct DistributionOriginCustomOriginConfig {
    /// HTTP port the custom origin listens on.
    #[builder(into)]
    #[serde(rename = "httpPort")]
    pub r#http_port: Box<i32>,
    /// HTTPS port the custom origin listens on.
    #[builder(into)]
    #[serde(rename = "httpsPort")]
    pub r#https_port: Box<i32>,
    #[builder(into, default)]
    #[serde(rename = "originKeepaliveTimeout")]
    pub r#origin_keepalive_timeout: Box<Option<i32>>,
    /// Origin protocol policy to apply to your origin. One of `http-only`, `https-only`, or `match-viewer`.
    #[builder(into)]
    #[serde(rename = "originProtocolPolicy")]
    pub r#origin_protocol_policy: Box<String>,
    #[builder(into, default)]
    #[serde(rename = "originReadTimeout")]
    pub r#origin_read_timeout: Box<Option<i32>>,
    /// List of SSL/TLS protocols that CloudFront can use when connecting to your origin over HTTPS. Valid values: `SSLv3`, `TLSv1`, `TLSv1.1`, `TLSv1.2`. For more information, see [Minimum Origin SSL Protocol](https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/distribution-web-values-specify.html#DownloadDistValuesOriginSSLProtocols) in the Amazon CloudFront Developer Guide.
    #[builder(into)]
    #[serde(rename = "originSslProtocols")]
    pub r#origin_ssl_protocols: Box<Vec<String>>,
}
