#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct EndpointGlobalDeliveryRuleUrlRedirectAction {
    /// Specifies the fragment part of the URL. This value must not start with a `#`.
    #[builder(into)]
    #[serde(rename = "fragment")]
    pub r#fragment: Option<String>,
    /// Specifies the hostname part of the URL.
    #[builder(into)]
    #[serde(rename = "hostname")]
    pub r#hostname: Option<String>,
    /// Specifies the path part of the URL. This value must begin with a `/`.
    #[builder(into)]
    #[serde(rename = "path")]
    pub r#path: Option<String>,
    /// Specifies the protocol part of the URL. Valid values are `MatchRequest`, `Http` and `Https`. Defaults to `MatchRequest`.
    #[builder(into)]
    #[serde(rename = "protocol")]
    pub r#protocol: Option<String>,
    /// Specifies the query string part of the URL. This value must not start with a `?` or `&` and must be in `<key>=<value>` format separated by `&`.
    #[builder(into)]
    #[serde(rename = "queryString")]
    pub r#query_string: Option<String>,
    /// Type of the redirect. Valid values are `Found`, `Moved`, `PermanentRedirect` and `TemporaryRedirect`.
    #[builder(into)]
    #[serde(rename = "redirectType")]
    pub r#redirect_type: String,
}
