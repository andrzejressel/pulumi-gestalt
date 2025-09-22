#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct FunctionAppSlotSiteCredential {
    /// The password associated with the username, which can be used to publish to this App Service.
    #[builder(into)]
    #[serde(rename = "password")]
    pub r#password: Option<String>,
    /// The username which can be used to publish to this App Service
    #[builder(into)]
    #[serde(rename = "username")]
    pub r#username: Option<String>,
}
