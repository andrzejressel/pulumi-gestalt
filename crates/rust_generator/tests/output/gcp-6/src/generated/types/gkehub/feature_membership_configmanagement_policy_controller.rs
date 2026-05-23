#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FeatureMembershipConfigmanagementPolicyController {
    /// Sets the interval for Policy Controller Audit Scans (in seconds). When set to 0, this disables audit functionality altogether.
    #[builder(into)]
    pub r#audit_interval_seconds: Option<String>,
    /// Enables the installation of Policy Controller. If false, the rest of PolicyController fields take no effect.
    #[builder(into)]
    pub r#enabled: Option<bool>,
    /// The set of namespaces that are excluded from Policy Controller checks. Namespaces do not need to currently exist on the cluster.
    #[builder(into)]
    pub r#exemptable_namespaces: Option<Vec<String>>,
    /// Logs all denies and dry run failures.
    #[builder(into)]
    pub r#log_denies_enabled: Option<bool>,
    /// Specifies the backends Policy Controller should export metrics to. For example, to specify metrics should be exported to Cloud Monitoring and Prometheus, specify backends: ["cloudmonitoring", "prometheus"]. Default: ["cloudmonitoring", "prometheus"]
    #[builder(into)]
    pub r#monitoring: Option<Box<super::super::types::gkehub::FeatureMembershipConfigmanagementPolicyControllerMonitoring>>,
    /// Enables mutation in policy controller. If true, mutation CRDs, webhook, and controller deployment will be deployed to the cluster.
    #[builder(into)]
    pub r#mutation_enabled: Option<bool>,
    /// Enables the ability to use Constraint Templates that reference to objects other than the object currently being evaluated.
    #[builder(into)]
    pub r#referential_rules_enabled: Option<bool>,
    /// Installs the default template library along with Policy Controller.
    #[builder(into)]
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
                    "auditIntervalSeconds",
                    &self.r#audit_interval_seconds,
                ),
                to_pulumi_object_field(
                    "enabled",
                    &self.r#enabled,
                ),
                to_pulumi_object_field(
                    "exemptableNamespaces",
                    &self.r#exemptable_namespaces,
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
                    "referentialRulesEnabled",
                    &self.r#referential_rules_enabled,
                ),
                to_pulumi_object_field(
                    "templateLibraryInstalled",
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
                        let field_value = match fields_map.get("auditIntervalSeconds") {
                            Some(value) => value,
                            None => bail!("Missing field 'auditIntervalSeconds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("exemptableNamespaces") {
                            Some(value) => value,
                            None => bail!("Missing field 'exemptableNamespaces' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#referential_rules_enabled: {
                        let field_value = match fields_map.get("referentialRulesEnabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'referentialRulesEnabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#template_library_installed: {
                        let field_value = match fields_map.get("templateLibraryInstalled") {
                            Some(value) => value,
                            None => bail!("Missing field 'templateLibraryInstalled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
