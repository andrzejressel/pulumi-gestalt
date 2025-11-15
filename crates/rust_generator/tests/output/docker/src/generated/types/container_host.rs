#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ContainerHost {
    /// Hostname to add
    #[builder(into)]
    #[serde(rename = "host")]
    pub r#host: String,
    /// IP address this hostname should resolve to.
    #[builder(into)]
    #[serde(rename = "ip")]
    pub r#ip: String,
}
