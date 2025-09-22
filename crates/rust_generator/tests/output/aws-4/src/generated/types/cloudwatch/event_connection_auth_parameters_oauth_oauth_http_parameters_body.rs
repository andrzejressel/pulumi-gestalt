#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct EventConnectionAuthParametersOauthOauthHttpParametersBody {
    /// Specified whether the value is secret.
    #[builder(into)]
    #[serde(rename = "isValueSecret")]
    pub r#is_value_secret: Option<bool>,
    /// The key for the parameter.
    #[builder(into)]
    #[serde(rename = "key")]
    pub r#key: Option<String>,
    /// The value associated with the key. Created and stored in AWS Secrets Manager if is secret.
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: Option<String>,
}
