#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for FeatureFleetDefaultMemberConfigPolicycontrollerPolicyControllerHubConfig {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "audit_interval_seconds",
                    &self.r#audit_interval_seconds,
                ),
                to_pulumi_object_field(
                    "constraint_violation_limit",
                    &self.r#constraint_violation_limit,
                ),
                to_pulumi_object_field(
                    "deployment_configs",
                    &self.r#deployment_configs,
                ),
                to_pulumi_object_field(
                    "exemptable_namespaces",
                    &self.r#exemptable_namespaces,
                ),
                to_pulumi_object_field(
                    "install_spec",
                    &self.r#install_spec,
                ),
                to_pulumi_object_field(
                    "log_denies_enabled",
                    &self.r#log_denies_enabled,
                ),
                to_pulumi_object_field(
                    "monitoring",
                    &self.r#monitoring,
                ),
                to_pulumi_object_field(
                    "mutation_enabled",
                    &self.r#mutation_enabled,
                ),
                to_pulumi_object_field(
                    "policy_content",
                    &self.r#policy_content,
                ),
                to_pulumi_object_field(
                    "referential_rules_enabled",
                    &self.r#referential_rules_enabled,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for FeatureFleetDefaultMemberConfigPolicycontrollerPolicyControllerHubConfig {
    fn from_pulumi_value(
        value: &pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    ) -> pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::rootcause::Result<Self> {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValueContent;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::rootcause::bail;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue;

        match value.content {
            PulumiValueContent::Object(ref _obj) => {
                use std::collections::BTreeMap;
                let fields_map: BTreeMap<String, PulumiValue> =
                    _obj.iter().cloned().collect();

                Ok(Self {
                    r#audit_interval_seconds: {
                        let field_value = match fields_map.get("audit_interval_seconds") {
                            Some(value) => value,
                            None => bail!("Missing field 'audit_interval_seconds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#constraint_violation_limit: {
                        let field_value = match fields_map.get("constraint_violation_limit") {
                            Some(value) => value,
                            None => bail!("Missing field 'constraint_violation_limit' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#deployment_configs: {
                        let field_value = match fields_map.get("deployment_configs") {
                            Some(value) => value,
                            None => bail!("Missing field 'deployment_configs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#exemptable_namespaces: {
                        let field_value = match fields_map.get("exemptable_namespaces") {
                            Some(value) => value,
                            None => bail!("Missing field 'exemptable_namespaces' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#install_spec: {
                        let field_value = match fields_map.get("install_spec") {
                            Some(value) => value,
                            None => bail!("Missing field 'install_spec' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#log_denies_enabled: {
                        let field_value = match fields_map.get("log_denies_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'log_denies_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#monitoring: {
                        let field_value = match fields_map.get("monitoring") {
                            Some(value) => value,
                            None => bail!("Missing field 'monitoring' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#mutation_enabled: {
                        let field_value = match fields_map.get("mutation_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'mutation_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#policy_content: {
                        let field_value = match fields_map.get("policy_content") {
                            Some(value) => value,
                            None => bail!("Missing field 'policy_content' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#referential_rules_enabled: {
                        let field_value = match fields_map.get("referential_rules_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'referential_rules_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
