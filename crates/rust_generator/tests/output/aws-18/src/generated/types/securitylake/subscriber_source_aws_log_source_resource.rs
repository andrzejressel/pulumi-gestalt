#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct SubscriberSourceAwsLogSourceResource {
    /// Provides data expiration details of Amazon Security Lake object.
    #[builder(into)]
    #[serde(rename = "sourceName")]
    pub r#source_name: String,
    /// Provides data storage transition details of Amazon Security Lake object.
    #[builder(into)]
    #[serde(rename = "sourceVersion")]
    pub r#source_version: Option<String>,
}
