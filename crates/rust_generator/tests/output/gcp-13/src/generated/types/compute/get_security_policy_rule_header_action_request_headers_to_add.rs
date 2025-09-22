#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetSecurityPolicyRuleHeaderActionRequestHeadersToAdd {
    /// The name of the header to set.
    #[builder(into)]
    #[serde(rename = "headerName")]
    pub r#header_name: String,
    /// The value to set the named header to.
    #[builder(into)]
    #[serde(rename = "headerValue")]
    pub r#header_value: String,
}
