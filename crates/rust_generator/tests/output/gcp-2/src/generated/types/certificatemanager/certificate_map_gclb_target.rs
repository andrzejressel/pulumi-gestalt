#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct CertificateMapGclbTarget {
    /// An IP configuration where this Certificate Map is serving
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "ipConfigs")]
    pub r#ip_configs: Option<Vec<super::super::types::certificatemanager::CertificateMapGclbTargetIpConfig>>,
    /// Proxy name must be in the format projects/*/locations/*/targetHttpsProxies/*.
    /// This field is part of a union field `target_proxy`: Only one of `targetHttpsProxy` or
    /// `targetSslProxy` may be set.
    #[builder(into)]
    #[serde(rename = "targetHttpsProxy")]
    pub r#target_https_proxy: Option<String>,
    /// Proxy name must be in the format projects/*/locations/*/targetSslProxies/*.
    /// This field is part of a union field `target_proxy`: Only one of `targetHttpsProxy` or
    /// `targetSslProxy` may be set.
    #[builder(into)]
    #[serde(rename = "targetSslProxy")]
    pub r#target_ssl_proxy: Option<String>,
}
