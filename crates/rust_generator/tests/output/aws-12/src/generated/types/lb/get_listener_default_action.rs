#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetListenerDefaultAction {
    #[builder(into)]
    #[serde(rename = "authenticateCognitos")]
    pub r#authenticate_cognitos: Vec<super::super::types::lb::GetListenerDefaultActionAuthenticateCognito>,
    #[builder(into)]
    #[serde(rename = "authenticateOidcs")]
    pub r#authenticate_oidcs: Vec<super::super::types::lb::GetListenerDefaultActionAuthenticateOidc>,
    #[builder(into)]
    #[serde(rename = "fixedResponses")]
    pub r#fixed_responses: Vec<super::super::types::lb::GetListenerDefaultActionFixedResponse>,
    #[builder(into)]
    #[serde(rename = "forwards")]
    pub r#forwards: Vec<super::super::types::lb::GetListenerDefaultActionForward>,
    #[builder(into)]
    #[serde(rename = "order")]
    pub r#order: i32,
    #[builder(into)]
    #[serde(rename = "redirects")]
    pub r#redirects: Vec<super::super::types::lb::GetListenerDefaultActionRedirect>,
    #[builder(into)]
    #[serde(rename = "targetGroupArn")]
    pub r#target_group_arn: String,
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: String,
}
