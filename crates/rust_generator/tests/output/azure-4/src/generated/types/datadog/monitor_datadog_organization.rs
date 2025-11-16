#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct MonitorDatadogOrganization {
    /// Api key associated to the Datadog organization. Changing this forces a new Datadog Monitor to be created.
    #[builder(into)]
    #[serde(rename = "apiKey")]
    pub r#api_key: String,
    /// Application key associated to the Datadog organization. Changing this forces a new Datadog Monitor to be created.
    #[builder(into)]
    #[serde(rename = "applicationKey")]
    pub r#application_key: String,
    /// The ID of the enterprise_app. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "enterpriseAppId")]
    pub r#enterprise_app_id: Option<String>,
    /// The ID of the Datadog Monitor.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Option<String>,
    /// The auth code used to linking to an existing Datadog organization. Changing this forces a new Datadog Monitor to be created.
    #[builder(into)]
    #[serde(rename = "linkingAuthCode")]
    pub r#linking_auth_code: Option<String>,
    /// The ID of the linking_client. Changing this forces a new Datadog Monitor to be created.
    #[builder(into)]
    #[serde(rename = "linkingClientId")]
    pub r#linking_client_id: Option<String>,
    /// The name of the user that will be associated with the Datadog Monitor. Changing this forces a new Datadog Monitor to be created.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// The redirect uri for linking. Changing this forces a new Datadog Monitor to be created.
    #[builder(into)]
    #[serde(rename = "redirectUri")]
    pub r#redirect_uri: Option<String>,
}
