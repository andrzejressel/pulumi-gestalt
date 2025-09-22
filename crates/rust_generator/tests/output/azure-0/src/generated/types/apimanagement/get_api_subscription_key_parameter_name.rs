#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetApiSubscriptionKeyParameterName {
    /// The name of the HTTP Header which should be used for the Subscription Key.
    #[builder(into)]
    #[serde(rename = "header")]
    pub r#header: String,
    /// The name of the QueryString parameter which should be used for the Subscription Key.
    #[builder(into)]
    #[serde(rename = "query")]
    pub r#query: String,
}
