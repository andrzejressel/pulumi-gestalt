#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
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
    pub r#monitoring: Option<Box<super::super::types::gkehub::FeatureMembershipConfigmanagementPolicyControllerMonitoring>>,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for FeatureMembershipConfigmanagementPolicyController {
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
                    "audit_interval_seconds",
                    &self.r#audit_interval_seconds,
                ),
                to_pulumi_object_field(
                    "enabled",
                    &self.r#enabled,
                ),
                to_pulumi_object_field(
                    "exemptable_namespaces",
                    &self.r#exemptable_namespaces,
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
                    "referential_rules_enabled",
                    &self.r#referential_rules_enabled,
                ),
                to_pulumi_object_field(
                    "template_library_installed",
                    &self.r#template_library_installed,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for FeatureMembershipConfigmanagementPolicyController {
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
                    r#enabled: {
                        let field_value = match fields_map.get("enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#referential_rules_enabled: {
                        let field_value = match fields_map.get("referential_rules_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'referential_rules_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#template_library_installed: {
                        let field_value = match fields_map.get("template_library_installed") {
                            Some(value) => value,
                            None => bail!("Missing field 'template_library_installed' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
