#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GrpcRouteRuleMatchHeader {
    /// Required. The key of the header.
    #[builder(into)]
    #[serde(rename = "key")]
    pub r#key: String,
    /// The type of match.
    /// Default value is `EXACT`.
    /// Possible values are: `TYPE_UNSPECIFIED`, `EXACT`, `REGULAR_EXPRESSION`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Option<String>,
    /// Required. The value of the header.
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: String,
}
