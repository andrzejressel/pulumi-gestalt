#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct FrontdoorRuleActionsUrlRedirectAction {
    /// The fragment to use in the redirect. The value must be a string between `0` and `1024` characters in length, leave blank to preserve the incoming fragment. Defaults to `""`.
    #[builder(into)]
    #[serde(rename = "destinationFragment")]
    pub r#destination_fragment: Option<String>,
    /// The host name you want the request to be redirected to. The value must be a string between `0` and `2048` characters in length, leave blank to preserve the incoming host.
    #[builder(into)]
    #[serde(rename = "destinationHostname")]
    pub r#destination_hostname: String,
    /// The path to use in the redirect. The value must be a string and include the leading `/`, leave blank to preserve the incoming path. Defaults to `""`.
    #[builder(into)]
    #[serde(rename = "destinationPath")]
    pub r#destination_path: Option<String>,
    /// The query string used in the redirect URL. The value must be in the &lt;key>=&lt;value> or &lt;key>={`action_server_variable`} format and must not include the leading `?`, leave blank to preserve the incoming query string. Maximum allowed length for this field is `2048` characters. Defaults to `""`.
    #[builder(into)]
    #[serde(rename = "queryString")]
    pub r#query_string: Option<String>,
    /// The protocol the request will be redirected as. Possible values include `MatchRequest`, `Http` or `Https`. Defaults to `MatchRequest`.
    #[builder(into)]
    #[serde(rename = "redirectProtocol")]
    pub r#redirect_protocol: Option<String>,
    /// The response type to return to the requestor. Possible values include `Moved`, `Found` , `TemporaryRedirect` or `PermanentRedirect`.
    #[builder(into)]
    #[serde(rename = "redirectType")]
    pub r#redirect_type: String,
}
