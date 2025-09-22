#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ConfigBlockingFunctions {
    /// The user credentials to include in the JWT payload that is sent to the registered Blocking Functions.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "forwardInboundCredentials")]
    pub r#forward_inbound_credentials: Option<Box<super::super::types::identityplatform::ConfigBlockingFunctionsForwardInboundCredentials>>,
    /// Map of Trigger to event type. Key should be one of the supported event types: "beforeCreate", "beforeSignIn".
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "triggers")]
    pub r#triggers: Vec<super::super::types::identityplatform::ConfigBlockingFunctionsTrigger>,
}
