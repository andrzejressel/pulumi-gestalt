#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct LoadBalancerListener {
    /// The port on the instance to route to
    #[builder(into)]
    #[serde(rename = "instancePort")]
    pub r#instance_port: i32,
    /// The protocol to use to the instance. Valid
    /// values are `HTTP`, `HTTPS`, `TCP`, or `SSL`
    #[builder(into)]
    #[serde(rename = "instanceProtocol")]
    pub r#instance_protocol: String,
    /// The port to listen on for the load balancer
    #[builder(into)]
    #[serde(rename = "lbPort")]
    pub r#lb_port: i32,
    /// The protocol to listen on. Valid values are `HTTP`,
    /// `HTTPS`, `TCP`, or `SSL`
    #[builder(into)]
    #[serde(rename = "lbProtocol")]
    pub r#lb_protocol: String,
    /// The ARN of an SSL certificate you have
    /// uploaded to AWS IAM. **Note ECDSA-specific restrictions below.  Only valid when `lb_protocol` is either HTTPS or SSL**
    #[builder(into)]
    #[serde(rename = "sslCertificateId")]
    pub r#ssl_certificate_id: Option<String>,
}
