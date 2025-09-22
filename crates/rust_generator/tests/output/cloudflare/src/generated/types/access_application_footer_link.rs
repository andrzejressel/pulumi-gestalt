#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct AccessApplicationFooterLink {
    /// The name of the footer link.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// The URL of the footer link.
    #[builder(into)]
    #[serde(rename = "url")]
    pub r#url: Option<String>,
}
