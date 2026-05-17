#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct WorkspaceEnhancedSecurityCompliance {
    /// Enables automatic cluster updates for this workspace. Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "automaticClusterUpdateEnabled")]
    pub r#automatic_cluster_update_enabled: Option<bool>,
    /// Enables compliance security profile for this workspace. Defaults to `false`.
    /// 
    /// > **Note:** Changing the value of `compliance_security_profile_enabled` from `true` to `false` forces a replacement of the Databricks workspace.
    /// 
    /// > **Note:** The attributes `automatic_cluster_update_enabled` and `enhanced_security_monitoring_enabled` must be set to `true` in order to set `compliance_security_profile_enabled` to `true`.
    #[builder(into)]
    #[serde(rename = "complianceSecurityProfileEnabled")]
    pub r#compliance_security_profile_enabled: Option<bool>,
    /// A list of standards to enforce on this workspace. Possible values include `HIPAA` and `PCI_DSS`.
    /// 
    /// > **Note:** `compliance_security_profile_enabled` must be set to `true` in order to use `compliance_security_profile_standards`.
    /// 
    /// > **Note:** Removing a standard from the `compliance_security_profile_standards` list forces a replacement of the Databricks workspace.
    #[builder(into)]
    #[serde(rename = "complianceSecurityProfileStandards")]
    pub r#compliance_security_profile_standards: Option<Vec<String>>,
    /// Enables enhanced security monitoring for this workspace. Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "enhancedSecurityMonitoringEnabled")]
    pub r#enhanced_security_monitoring_enabled: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for WorkspaceEnhancedSecurityCompliance {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "automatic_cluster_update_enabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#automatic_cluster_update_enabled,
                )
                .await,
            );
            map.insert(
                "compliance_security_profile_enabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#compliance_security_profile_enabled,
                )
                .await,
            );
            map.insert(
                "compliance_security_profile_standards".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#compliance_security_profile_standards,
                )
                .await,
            );
            map.insert(
                "enhanced_security_monitoring_enabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#enhanced_security_monitoring_enabled,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for WorkspaceEnhancedSecurityCompliance {
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
                    r#automatic_cluster_update_enabled: {
                        let field_value = match fields_map.get("automatic_cluster_update_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'automatic_cluster_update_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#compliance_security_profile_enabled: {
                        let field_value = match fields_map.get("compliance_security_profile_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'compliance_security_profile_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#compliance_security_profile_standards: {
                        let field_value = match fields_map.get("compliance_security_profile_standards") {
                            Some(value) => value,
                            None => bail!("Missing field 'compliance_security_profile_standards' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#enhanced_security_monitoring_enabled: {
                        let field_value = match fields_map.get("enhanced_security_monitoring_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'enhanced_security_monitoring_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
