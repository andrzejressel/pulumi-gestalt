#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetResourceShareFilter {
    /// Name of the tag key to filter on.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// Value of the tag key.
    #[builder(into)]
    #[serde(rename = "values")]
    pub r#values: Vec<String>,
}
