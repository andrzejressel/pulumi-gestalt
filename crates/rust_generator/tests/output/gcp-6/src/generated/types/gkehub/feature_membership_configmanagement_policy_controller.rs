#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct FeatureMembershipConfigmanagementPolicyController {
    /// Sets the interval for Policy Controller Audit Scans (in seconds). When set to 0, this disables audit functionality altogether.
    #[builder(into)]
    #[serde(rename = "auditIntervalSeconds")]
    pub r#audit_interval_seconds: Option<String>,
    /// Enables the installation of Policy Controller. If false, the rest of PolicyController fields take no effect.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Option<bool>,
    /// The set of namespaces that are excluded from Policy Controller checks. Namespaces do not need to currently exist on the cluster.
    #[builder(into)]
    #[serde(rename = "exemptableNamespaces")]
    pub r#exemptable_namespaces: Option<Vec<String>>,
    /// Logs all denies and dry run failures.
    #[builder(into)]
    #[serde(rename = "logDeniesEnabled")]
    pub r#log_denies_enabled: Option<bool>,
    /// Specifies the backends Policy Controller should export metrics to. For example, to specify metrics should be exported to Cloud Monitoring and Prometheus, specify backends: ["cloudmonitoring", "prometheus"]. Default: ["cloudmonitoring", "prometheus"]
    #[builder(into)]
    #[serde(rename = "monitoring")]
    pub r#monitoring: Box<Option<super::super::types::gkehub::FeatureMembershipConfigmanagementPolicyControllerMonitoring>>,
    /// Enables mutation in policy controller. If true, mutation CRDs, webhook, and controller deployment will be deployed to the cluster.
    #[builder(into)]
    #[serde(rename = "mutationEnabled")]
    pub r#mutation_enabled: Option<bool>,
    /// Enables the ability to use Constraint Templates that reference to objects other than the object currently being evaluated.
    #[builder(into)]
    #[serde(rename = "referentialRulesEnabled")]
    pub r#referential_rules_enabled: Option<bool>,
    /// Installs the default template library along with Policy Controller.
    #[builder(into)]
    #[serde(rename = "templateLibraryInstalled")]
    pub r#template_library_installed: Option<bool>,
}
