#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct WebAclRuleActionChallengeCustomRequestHandling {
    /// The `insert_header` blocks used to define HTTP headers added to the request. See `insert_header` below for details.
    #[builder(into)]
    #[serde(rename = "insertHeaders")]
    pub r#insert_headers: Vec<super::super::types::wafv2::WebAclRuleActionChallengeCustomRequestHandlingInsertHeader>,
}
