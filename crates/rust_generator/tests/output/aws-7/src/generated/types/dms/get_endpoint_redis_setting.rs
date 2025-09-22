#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetEndpointRedisSetting {
    #[builder(into)]
    #[serde(rename = "authPassword")]
    pub r#auth_password: String,
    #[builder(into)]
    #[serde(rename = "authType")]
    pub r#auth_type: String,
    #[builder(into)]
    #[serde(rename = "authUserName")]
    pub r#auth_user_name: String,
    #[builder(into)]
    #[serde(rename = "port")]
    pub r#port: i32,
    #[builder(into)]
    #[serde(rename = "serverName")]
    pub r#server_name: String,
    #[builder(into)]
    #[serde(rename = "sslCaCertificateArn")]
    pub r#ssl_ca_certificate_arn: String,
    #[builder(into)]
    #[serde(rename = "sslSecurityProtocol")]
    pub r#ssl_security_protocol: String,
}
