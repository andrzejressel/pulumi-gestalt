#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct FeatureFleetDefaultMemberConfigPolicycontrollerPolicyControllerHubConfig {
    /// Interval for Policy Controller Audit scans (in seconds). When set to 0, this disables audit functionality altogether.
    #[builder(into)]
    #[serde(rename = "auditIntervalSeconds")]
    pub r#audit_interval_seconds: Option<i32>,
    /// The maximum number of audit violations to be stored in a constraint. If not set, the internal default of 20 will be used.
    #[builder(into)]
    #[serde(rename = "constraintViolationLimit")]
    pub r#constraint_violation_limit: Option<i32>,
    /// Map of deployment configs to deployments ("admission", "audit", "mutation").
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "deploymentConfigs")]
    pub r#deployment_configs: Option<Vec<super::super::types::gkehub::FeatureFleetDefaultMemberConfigPolicycontrollerPolicyControllerHubConfigDeploymentConfig>>,
    /// The set of namespaces that are excluded from Policy Controller checks. Namespaces do not need to currently exist on the cluster.
    #[builder(into)]
    #[serde(rename = "exemptableNamespaces")]
    pub r#exemptable_namespaces: Option<Vec<String>>,
    /// Configures the mode of the Policy Controller installation
    /// Possible values are: `INSTALL_SPEC_UNSPECIFIED`, `INSTALL_SPEC_NOT_INSTALLED`, `INSTALL_SPEC_ENABLED`, `INSTALL_SPEC_SUSPENDED`, `INSTALL_SPEC_DETACHED`.
    #[builder(into)]
    #[serde(rename = "installSpec")]
    pub r#install_spec: String,
    /// Logs all denies and dry run failures.
    #[builder(into)]
    #[serde(rename = "logDeniesEnabled")]
    pub r#log_denies_enabled: Option<bool>,
    /// Monitoring specifies the configuration of monitoring Policy Controller.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "monitoring")]
    pub r#monitoring: Option<Box<super::super::types::gkehub::FeatureFleetDefaultMemberConfigPolicycontrollerPolicyControllerHubConfigMonitoring>>,
    /// Enables the ability to mutate resources using Policy Controller.
    #[builder(into)]
    #[serde(rename = "mutationEnabled")]
    pub r#mutation_enabled: Option<bool>,
    /// Specifies the desired policy content on the cluster.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "policyContent")]
    pub r#policy_content: Option<Box<super::super::types::gkehub::FeatureFleetDefaultMemberConfigPolicycontrollerPolicyControllerHubConfigPolicyContent>>,
    /// Enables the ability to use Constraint Templates that reference to objects other than the object currently being evaluated.
    #[builder(into)]
    #[serde(rename = "referentialRulesEnabled")]
    pub r#referential_rules_enabled: Option<bool>,
}
