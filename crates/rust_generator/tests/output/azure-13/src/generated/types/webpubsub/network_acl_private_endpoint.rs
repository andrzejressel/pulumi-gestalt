#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct NetworkAclPrivateEndpoint {
    /// The allowed request types for the Private Endpoint Connection. Possible values are `ClientConnection`, `ServerConnection`, `RESTAPI` and `Trace`.
    #[builder(into)]
    #[serde(rename = "allowedRequestTypes")]
    pub r#allowed_request_types: Option<Vec<String>>,
    /// The denied request types for the Private Endpoint Connection. Possible values are `ClientConnection`, `ServerConnection`, `RESTAPI` and `Trace`.
    /// 
    /// > **NOTE:** When `default_action` is `Allow`, `allowed_request_types`cannot be set. When `default_action` is `Deny`, `denied_request_types`cannot be set.
    #[builder(into)]
    #[serde(rename = "deniedRequestTypes")]
    pub r#denied_request_types: Option<Vec<String>>,
    /// The ID of the Private Endpoint which is based on the Web Pubsub service.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: String,
}
