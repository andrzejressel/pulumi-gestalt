#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct TableAttribute {
    /// Name of the attribute
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// Attribute type. Valid values are `S` (string), `N` (number), `B` (binary).
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: String,
}
