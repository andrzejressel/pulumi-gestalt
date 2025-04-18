#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ContainerPort {
    /// Port exposed out of the container. If not given a free random port `>= 32768` will be used.
    #[builder(into, default)]
    #[serde(rename = "external")]
    pub r#external: Box<Option<i32>>,
    /// Port within the container.
    #[builder(into)]
    #[serde(rename = "internal")]
    pub r#internal: Box<i32>,
    /// IP address/mask that can access this port. Defaults to `0.0.0.0`.
    #[builder(into, default)]
    #[serde(rename = "ip")]
    pub r#ip: Box<Option<String>>,
    /// Protocol that can be used over this port. Defaults to `tcp`.
    #[builder(into, default)]
    #[serde(rename = "protocol")]
    pub r#protocol: Box<Option<String>>,
}
