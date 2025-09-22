#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct TableTtl {
    /// Name of the table attribute to store the TTL timestamp in.
    /// Required if `enabled` is `true`, must not be set otherwise.
    #[builder(into)]
    #[serde(rename = "attributeName")]
    pub r#attribute_name: Option<String>,
    /// Whether TTL is enabled.
    /// Default value is `false`.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Option<bool>,
}
