#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetSecurityPolicyAdvancedOptionsConfigJsonCustomConfig {
    /// A list of custom Content-Type header values to apply the JSON parsing.
    #[builder(into)]
    #[serde(rename = "contentTypes")]
    pub r#content_types: Box<Vec<String>>,
}
