#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetLoadBalancerListener {
    #[builder(into)]
    #[serde(rename = "instancePort")]
    pub r#instance_port: i32,
    #[builder(into)]
    #[serde(rename = "instanceProtocol")]
    pub r#instance_protocol: String,
    #[builder(into)]
    #[serde(rename = "lbPort")]
    pub r#lb_port: i32,
    #[builder(into)]
    #[serde(rename = "lbProtocol")]
    pub r#lb_protocol: String,
    #[builder(into)]
    #[serde(rename = "sslCertificateId")]
    pub r#ssl_certificate_id: String,
}
