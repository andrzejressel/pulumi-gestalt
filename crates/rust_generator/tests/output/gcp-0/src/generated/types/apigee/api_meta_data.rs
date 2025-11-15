#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ApiMetaData {
    /// Time at which the API proxy was created, in milliseconds since epoch.
    #[builder(into)]
    #[serde(rename = "createdAt")]
    pub r#created_at: Option<String>,
    /// Time at which the API proxy was most recently modified, in milliseconds since epoch.
    #[builder(into)]
    #[serde(rename = "lastModifiedAt")]
    pub r#last_modified_at: Option<String>,
    /// The type of entity described
    #[builder(into)]
    #[serde(rename = "subType")]
    pub r#sub_type: Option<String>,
}
