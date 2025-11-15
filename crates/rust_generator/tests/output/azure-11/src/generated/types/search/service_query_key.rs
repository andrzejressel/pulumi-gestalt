#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ServiceQueryKey {
    /// The value of this Query Key.
    #[builder(into)]
    #[serde(rename = "key")]
    pub r#key: Option<String>,
    /// The Name which should be used for this Search Service. Changing this forces a new Search Service to be created.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Option<String>,
}
