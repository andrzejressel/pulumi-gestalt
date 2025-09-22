#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct WorkstationConfigReadinessCheck {
    /// Path to which the request should be sent.
    #[builder(into)]
    #[serde(rename = "path")]
    pub r#path: String,
    /// Port to which the request should be sent.
    #[builder(into)]
    #[serde(rename = "port")]
    pub r#port: i32,
}
