#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct HttpRouteRuleActionResponseHeaderModifier {
    /// Add the headers with given map where key is the name of the header, value is the value of the header.
    #[builder(into)]
    #[serde(rename = "add")]
    pub r#add: Option<std::collections::HashMap<String, String>>,
    /// Remove headers (matching by header names) specified in the list.
    #[builder(into)]
    #[serde(rename = "removes")]
    pub r#removes: Option<Vec<String>>,
    /// Completely overwrite/replace the headers with given map where key is the name of the header, value is the value of the header.
    #[builder(into)]
    #[serde(rename = "set")]
    pub r#set: Option<std::collections::HashMap<String, String>>,
}
