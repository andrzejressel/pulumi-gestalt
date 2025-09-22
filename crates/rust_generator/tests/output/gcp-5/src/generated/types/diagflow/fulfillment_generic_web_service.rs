#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct FulfillmentGenericWebService {
    /// The password for HTTP Basic authentication.
    #[builder(into)]
    #[serde(rename = "password")]
    pub r#password: Option<String>,
    /// The HTTP request headers to send together with fulfillment requests.
    #[builder(into)]
    #[serde(rename = "requestHeaders")]
    pub r#request_headers: Option<std::collections::HashMap<String, String>>,
    /// The fulfillment URI for receiving POST requests. It must use https protocol.
    #[builder(into)]
    #[serde(rename = "uri")]
    pub r#uri: String,
    /// The user name for HTTP Basic authentication.
    #[builder(into)]
    #[serde(rename = "username")]
    pub r#username: Option<String>,
}
