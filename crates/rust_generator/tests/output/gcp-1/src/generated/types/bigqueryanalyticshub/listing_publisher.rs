#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ListingPublisher {
    /// Name of the listing publisher.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Email or URL of the listing publisher.
    #[builder(into, default)]
    #[serde(rename = "primaryContact")]
    pub r#primary_contact: Box<Option<String>>,
}
