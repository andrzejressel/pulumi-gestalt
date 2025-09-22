#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ServiceConvergeConfig {
    /// The interval to check if the desired state is reached `(ms|s)`. Defaults to `7s`.
    #[builder(into)]
    #[serde(rename = "delay")]
    pub r#delay: Option<String>,
    /// The timeout of the service to reach the desired state `(s|m)`. Defaults to `3m`
    #[builder(into)]
    #[serde(rename = "timeout")]
    pub r#timeout: Option<String>,
}
