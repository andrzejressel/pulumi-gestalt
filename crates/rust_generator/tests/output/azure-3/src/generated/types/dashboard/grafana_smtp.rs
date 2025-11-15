#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GrafanaSmtp {
    /// Whether to enable the smtp setting of the Grafana instance. Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Option<bool>,
    /// Address used when sending emails.
    #[builder(into)]
    #[serde(rename = "fromAddress")]
    pub r#from_address: String,
    /// Name used when sending emails. Defaults to `Azure Managed Grafana Notification`.
    #[builder(into)]
    #[serde(rename = "fromName")]
    pub r#from_name: Option<String>,
    /// SMTP server hostname with port, e.g. test.email.net:587
    #[builder(into)]
    #[serde(rename = "host")]
    pub r#host: String,
    /// Password of SMTP authentication.
    #[builder(into)]
    #[serde(rename = "password")]
    pub r#password: String,
    /// Whether to use TLS when connecting to SMTP server. Possible values are `OpportunisticStartTLS`, `NoStartTLS`, `MandatoryStartTLS`.
    #[builder(into)]
    #[serde(rename = "startTlsPolicy")]
    pub r#start_tls_policy: String,
    /// User of SMTP authentication.
    #[builder(into)]
    #[serde(rename = "user")]
    pub r#user: String,
    /// Whether verify SSL for SMTP server. Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "verificationSkipEnabled")]
    pub r#verification_skip_enabled: Option<bool>,
}
