#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetListenerDefaultActionAuthenticateCognito {
    #[builder(into)]
    #[serde(rename = "authenticationRequestExtraParams")]
    pub r#authentication_request_extra_params: std::collections::HashMap<String, String>,
    #[builder(into)]
    #[serde(rename = "onUnauthenticatedRequest")]
    pub r#on_unauthenticated_request: String,
    #[builder(into)]
    #[serde(rename = "scope")]
    pub r#scope: String,
    #[builder(into)]
    #[serde(rename = "sessionCookieName")]
    pub r#session_cookie_name: String,
    #[builder(into)]
    #[serde(rename = "sessionTimeout")]
    pub r#session_timeout: i32,
    #[builder(into)]
    #[serde(rename = "userPoolArn")]
    pub r#user_pool_arn: String,
    #[builder(into)]
    #[serde(rename = "userPoolClientId")]
    pub r#user_pool_client_id: String,
    #[builder(into)]
    #[serde(rename = "userPoolDomain")]
    pub r#user_pool_domain: String,
}
