#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ServiceTemplateSpecContainerPort {
    /// Port number the container listens on. This must be a valid port number (between 1 and 65535). Defaults to "8080".
    #[builder(into)]
    #[serde(rename = "containerPort")]
    pub r#container_port: Option<i32>,
    /// If specified, used to specify which protocol to use. Allowed values are "http1" (HTTP/1) and "h2c" (HTTP/2 end-to-end). Defaults to "http1".
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// Protocol for port. Must be "TCP". Defaults to "TCP".
    #[builder(into)]
    #[serde(rename = "protocol")]
    pub r#protocol: Option<String>,
}
