#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ChannelDirectLineSite {
    /// Enables/Disables this site. Defaults to `true`.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Option<bool>,
    /// Is the endpoint parameters enabled for this site?
    #[builder(into)]
    #[serde(rename = "endpointParametersEnabled")]
    pub r#endpoint_parameters_enabled: Option<bool>,
    /// Enables additional security measures for this site, see [Enhanced Directline Authentication Features](https://blog.botframework.com/2018/09/25/enhanced-direct-line-authentication-features). Disabled by default.
    #[builder(into)]
    #[serde(rename = "enhancedAuthenticationEnabled")]
    pub r#enhanced_authentication_enabled: Option<bool>,
    /// Id for the site
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Option<String>,
    /// Primary key for accessing this site
    #[builder(into)]
    #[serde(rename = "key")]
    pub r#key: Option<String>,
    /// Secondary key for accessing this site
    #[builder(into)]
    #[serde(rename = "key2")]
    pub r#key_2: Option<String>,
    /// The name of the site
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// Is the storage site enabled for detailed logging? Defaults to `true`.
    #[builder(into)]
    #[serde(rename = "storageEnabled")]
    pub r#storage_enabled: Option<bool>,
    /// This field is required when `is_secure_site_enabled` is enabled. Determines which origins can establish a Directline conversation for this site.
    #[builder(into)]
    #[serde(rename = "trustedOrigins")]
    pub r#trusted_origins: Option<Vec<String>>,
    /// Is the user upload enabled for this site? Defaults to `true`.
    #[builder(into)]
    #[serde(rename = "userUploadEnabled")]
    pub r#user_upload_enabled: Option<bool>,
    /// Enables v1 of the Directline protocol for this site. Defaults to `true`.
    #[builder(into)]
    #[serde(rename = "v1Allowed")]
    pub r#v_1_allowed: Option<bool>,
    /// Enables v3 of the Directline protocol for this site. Defaults to `true`.
    #[builder(into)]
    #[serde(rename = "v3Allowed")]
    pub r#v_3_allowed: Option<bool>,
}
