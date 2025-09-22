#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct EventConnectionAuthParametersOauthOauthHttpParameters {
    /// Contains additional body string parameters for the connection. You can include up to 100 additional body string parameters per request. Each additional parameter counts towards the event payload size, which cannot exceed 64 KB. Each parameter can contain the following:
    #[builder(into)]
    #[serde(rename = "bodies")]
    pub r#bodies: Option<Vec<super::super::types::cloudwatch::EventConnectionAuthParametersOauthOauthHttpParametersBody>>,
    /// Contains additional header parameters for the connection. You can include up to 100 additional body string parameters per request. Each additional parameter counts towards the event payload size, which cannot exceed 64 KB. Each parameter can contain the following:
    #[builder(into)]
    #[serde(rename = "headers")]
    pub r#headers: Option<Vec<super::super::types::cloudwatch::EventConnectionAuthParametersOauthOauthHttpParametersHeader>>,
    /// Contains additional query string parameters for the connection. You can include up to 100 additional body string parameters per request. Each additional parameter counts towards the event payload size, which cannot exceed 64 KB. Each parameter can contain the following:
    #[builder(into)]
    #[serde(rename = "queryStrings")]
    pub r#query_strings: Option<Vec<super::super::types::cloudwatch::EventConnectionAuthParametersOauthOauthHttpParametersQueryString>>,
}
