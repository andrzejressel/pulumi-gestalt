#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct SharedDirectoryTarget {
    /// Identifier of the directory consumer account.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: String,
    /// Type of identifier to be used in the `id` field. Valid value is `ACCOUNT`. Default is `ACCOUNT`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Option<String>,
}
