#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FeatureMembershipPolicycontrollerPolicyControllerHubConfig {
    /// Sets the interval for Policy Controller Audit Scans (in seconds). When set to 0, this disables audit functionality altogether.
    #[builder(into)]
    pub r#audit_interval_seconds: Option<i32>,
    /// The maximum number of audit violations to be stored in a constraint. If not set, the  default of 20 will be used.
    #[builder(into)]
    pub r#constraint_violation_limit: Option<i32>,
    /// Map of deployment configs to deployments ("admission", "audit", "mutation").
    #[builder(into)]
    pub r#deployment_configs: Option<Vec<super::super::types::gkehub::FeatureMembershipPolicycontrollerPolicyControllerHubConfigDeploymentConfig>>,
    /// The set of namespaces that are excluded from Policy Controller checks. Namespaces do not need to currently exist on the cluster.
    #[builder(into)]
    pub r#exemptable_namespaces: Option<Vec<String>>,
    /// Configures the mode of the Policy Controller installation. Must be one of `INSTALL_SPEC_NOT_INSTALLED`, `INSTALL_SPEC_ENABLED`, `INSTALL_SPEC_SUSPENDED` or `INSTALL_SPEC_DETACHED`.
    #[builder(into)]
    pub r#install_spec: Option<String>,
    /// Logs all denies and dry run failures.
    #[builder(into)]
    pub r#log_denies_enabled: Option<bool>,
    /// Specifies the backends Policy Controller should export metrics to. Structure is documented below.
    #[builder(into)]
    pub r#monitoring: Option<Box<super::super::types::gkehub::FeatureMembershipPolicycontrollerPolicyControllerHubConfigMonitoring>>,
    /// Enables mutation in policy controller. If true, mutation CRDs, webhook, and controller deployment will be deployed to the cluster.
    #[builder(into)]
    pub r#mutation_enabled: Option<bool>,
    /// Specifies the desired policy content on the cluster. Structure is documented below.
    #[builder(into)]
    pub r#policy_content: Option<Box<super::super::types::gkehub::FeatureMembershipPolicycontrollerPolicyControllerHubConfigPolicyContent>>,
    /// Enables the ability to use Constraint Templates that reference to objects other than the object currently being evaluated.
    #[builder(into)]
    pub r#referential_rules_enabled: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for FeatureMembershipPolicycontrollerPolicyControllerHubConfig {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > + Send {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "auditIntervalSeconds",
                    &self.r#audit_interval_seconds,
                ),
                to_pulumi_object_field(
                    "constraintViolationLimit",
                    &self.r#constraint_violation_limit,
                ),
                to_pulumi_object_field(
                    "deploymentConfigs",
                    &self.r#deployment_configs,
                ),
                to_pulumi_object_field(
                    "exemptableNamespaces",
                    &self.r#exemptable_namespaces,
                ),
                to_pulumi_object_field(
                    "installSpec",
                    &self.r#install_spec,
                ),
                to_pulumi_object_field(
                    "logDeniesEnabled",
                    &self.r#log_denies_enabled,
                ),
                to_pulumi_object_field(
                    "monitoring",
                    &self.r#monitoring,
                ),
                to_pulumi_object_field(
                    "mutationEnabled",
                    &self.r#mutation_enabled,
                ),
                to_pulumi_object_field(
                    "policyContent",
                    &self.r#policy_content,
                ),
                to_pulumi_object_field(
                    "referentialRulesEnabled",
                    &self.r#referential_rules_enabled,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for FeatureMembershipPolicycontrollerPolicyControllerHubConfig {
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
                        let field_value = match fields_map.get("auditIntervalSeconds") {
                            Some(value) => value,
                            None => bail!("Missing field 'auditIntervalSeconds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#constraint_violation_limit: {
                        let field_value = match fields_map.get("constraintViolationLimit") {
                            Some(value) => value,
                            None => bail!("Missing field 'constraintViolationLimit' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#deployment_configs: {
                        let field_value = match fields_map.get("deploymentConfigs") {
                            Some(value) => value,
                            None => bail!("Missing field 'deploymentConfigs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#exemptable_namespaces: {
                        let field_value = match fields_map.get("exemptableNamespaces") {
                            Some(value) => value,
                            None => bail!("Missing field 'exemptableNamespaces' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#install_spec: {
                        let field_value = match fields_map.get("installSpec") {
                            Some(value) => value,
                            None => bail!("Missing field 'installSpec' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#log_denies_enabled: {
                        let field_value = match fields_map.get("logDeniesEnabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'logDeniesEnabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("mutationEnabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'mutationEnabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#policy_content: {
                        let field_value = match fields_map.get("policyContent") {
                            Some(value) => value,
                            None => bail!("Missing field 'policyContent' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#referential_rules_enabled: {
                        let field_value = match fields_map.get("referentialRulesEnabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'referentialRulesEnabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
