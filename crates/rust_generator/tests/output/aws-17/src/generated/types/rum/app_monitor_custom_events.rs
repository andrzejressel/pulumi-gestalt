#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AppMonitorCustomEvents {
    /// Specifies whether this app monitor allows the web client to define and send custom events. The default is for custom events to be `DISABLED`. Valid values are `DISABLED` and `ENABLED`.
    #[builder(into)]
    #[serde(rename = "status")]
    pub r#status: Option<String>,
}
