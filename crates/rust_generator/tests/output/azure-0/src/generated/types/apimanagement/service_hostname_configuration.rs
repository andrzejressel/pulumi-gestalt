#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ServiceHostnameConfiguration {
    /// One or more `developer_portal` blocks as documented below.
    #[builder(into)]
    #[serde(rename = "developerPortals")]
    pub r#developer_portals: Option<Vec<super::super::types::apimanagement::ServiceHostnameConfigurationDeveloperPortal>>,
    /// One or more `management` blocks as documented below.
    #[builder(into)]
    #[serde(rename = "managements")]
    pub r#managements: Option<Vec<super::super::types::apimanagement::ServiceHostnameConfigurationManagement>>,
    /// One or more `portal` blocks as documented below.
    #[builder(into)]
    #[serde(rename = "portals")]
    pub r#portals: Option<Vec<super::super::types::apimanagement::ServiceHostnameConfigurationPortal>>,
    /// One or more `proxy` blocks as documented below.
    #[builder(into)]
    #[serde(rename = "proxies")]
    pub r#proxies: Option<Vec<super::super::types::apimanagement::ServiceHostnameConfigurationProxy>>,
    /// One or more `scm` blocks as documented below.
    #[builder(into)]
    #[serde(rename = "scms")]
    pub r#scms: Option<Vec<super::super::types::apimanagement::ServiceHostnameConfigurationScm>>,
}
