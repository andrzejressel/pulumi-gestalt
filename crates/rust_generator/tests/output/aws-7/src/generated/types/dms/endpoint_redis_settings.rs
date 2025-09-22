#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct EndpointRedisSettings {
    /// The password provided with the auth-role and auth-token options of the AuthType setting for a Redis target endpoint.
    #[builder(into)]
    #[serde(rename = "authPassword")]
    pub r#auth_password: Option<String>,
    /// The type of authentication to perform when connecting to a Redis target. Options include `none`, `auth-token`, and `auth-role`. The `auth-token` option requires an `auth_password` value to be provided. The `auth-role` option requires `auth_user_name` and `auth_password` values to be provided.
    #[builder(into)]
    #[serde(rename = "authType")]
    pub r#auth_type: String,
    /// The username provided with the `auth-role` option of the AuthType setting for a Redis target endpoint.
    #[builder(into)]
    #[serde(rename = "authUserName")]
    pub r#auth_user_name: Option<String>,
    /// Transmission Control Protocol (TCP) port for the endpoint.
    #[builder(into)]
    #[serde(rename = "port")]
    pub r#port: i32,
    /// Fully qualified domain name of the endpoint.
    #[builder(into)]
    #[serde(rename = "serverName")]
    pub r#server_name: String,
    /// The Amazon Resource Name (ARN) for the certificate authority (CA) that DMS uses to connect to your Redis target endpoint.
    #[builder(into)]
    #[serde(rename = "sslCaCertificateArn")]
    pub r#ssl_ca_certificate_arn: Option<String>,
    /// The plaintext option doesn't provide Transport Layer Security (TLS) encryption for traffic between endpoint and database. Options include `plaintext`, `ssl-encryption`. The default is `ssl-encryption`.
    #[builder(into)]
    #[serde(rename = "sslSecurityProtocol")]
    pub r#ssl_security_protocol: Option<String>,
}
