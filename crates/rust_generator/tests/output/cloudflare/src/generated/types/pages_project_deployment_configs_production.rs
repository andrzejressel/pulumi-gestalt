#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PagesProjectDeploymentConfigsProduction {
    /// Use latest compatibility date for Pages Functions. Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "alwaysUseLatestCompatibilityDate")]
    pub r#always_use_latest_compatibility_date: Option<bool>,
    /// Compatibility date used for Pages Functions.
    #[builder(into)]
    #[serde(rename = "compatibilityDate")]
    pub r#compatibility_date: Option<String>,
    /// Compatibility flags used for Pages Functions.
    #[builder(into)]
    #[serde(rename = "compatibilityFlags")]
    pub r#compatibility_flags: Option<Vec<String>>,
    /// D1 Databases used for Pages Functions. Defaults to `map[]`.
    #[builder(into)]
    #[serde(rename = "d1Databases")]
    pub r#d_1_databases: Option<std::collections::HashMap<String, String>>,
    /// Durable Object namespaces used for Pages Functions. Defaults to `map[]`.
    #[builder(into)]
    #[serde(rename = "durableObjectNamespaces")]
    pub r#durable_object_namespaces: Option<std::collections::HashMap<String, String>>,
    /// Environment variables for Pages Functions. Defaults to `map[]`.
    #[builder(into)]
    #[serde(rename = "environmentVariables")]
    pub r#environment_variables: Option<std::collections::HashMap<String, String>>,
    /// Fail open used for Pages Functions. Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "failOpen")]
    pub r#fail_open: Option<bool>,
    /// KV namespaces used for Pages Functions. Defaults to `map[]`.
    #[builder(into)]
    #[serde(rename = "kvNamespaces")]
    pub r#kv_namespaces: Option<std::collections::HashMap<String, String>>,
    /// Configuration for placement in the Cloudflare Pages project.
    #[builder(into)]
    #[serde(rename = "placement")]
    pub r#placement: Box<Option<super::types::PagesProjectDeploymentConfigsProductionPlacement>>,
    /// R2 Buckets used for Pages Functions. Defaults to `map[]`.
    #[builder(into)]
    #[serde(rename = "r2Buckets")]
    pub r#r_2_buckets: Option<std::collections::HashMap<String, String>>,
    /// Encrypted environment variables for Pages Functions. Defaults to `map[]`.
    #[builder(into)]
    #[serde(rename = "secrets")]
    pub r#secrets: Option<std::collections::HashMap<String, String>>,
    /// Services used for Pages Functions.
    #[builder(into)]
    #[serde(rename = "serviceBindings")]
    pub r#service_bindings: Option<Vec<super::types::PagesProjectDeploymentConfigsProductionServiceBinding>>,
    /// Usage model used for Pages Functions. Available values: `unbound`, `bundled`, `standard`. Defaults to `bundled`.
    #[builder(into)]
    #[serde(rename = "usageModel")]
    pub r#usage_model: Option<String>,
}
