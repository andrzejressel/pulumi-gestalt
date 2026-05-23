#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetWorkspaceEnhancedSecurityCompliance {
    /// Whether automatic cluster updates for this workspace is enabled.
    #[builder(into)]
    pub r#automatic_cluster_update_enabled: bool,
    /// Whether compliance security profile for this workspace is enabled.
    #[builder(into)]
    pub r#compliance_security_profile_enabled: bool,
    /// A list of standards enforced on this workspace.
    #[builder(into)]
    pub r#compliance_security_profile_standards: Vec<String>,
    /// Whether enhanced security monitoring for this workspace is enabled.
    #[builder(into)]
    pub r#enhanced_security_monitoring_enabled: bool,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetWorkspaceEnhancedSecurityCompliance {
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
                    "automaticClusterUpdateEnabled",
                    &self.r#automatic_cluster_update_enabled,
                ),
                to_pulumi_object_field(
                    "complianceSecurityProfileEnabled",
                    &self.r#compliance_security_profile_enabled,
                ),
                to_pulumi_object_field(
                    "complianceSecurityProfileStandards",
                    &self.r#compliance_security_profile_standards,
                ),
                to_pulumi_object_field(
                    "enhancedSecurityMonitoringEnabled",
                    &self.r#enhanced_security_monitoring_enabled,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetWorkspaceEnhancedSecurityCompliance {
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
                        let field_value = match fields_map.get("automaticClusterUpdateEnabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'automaticClusterUpdateEnabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#compliance_security_profile_enabled: {
                        let field_value = match fields_map.get("complianceSecurityProfileEnabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'complianceSecurityProfileEnabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#compliance_security_profile_standards: {
                        let field_value = match fields_map.get("complianceSecurityProfileStandards") {
                            Some(value) => value,
                            None => bail!("Missing field 'complianceSecurityProfileStandards' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#enhanced_security_monitoring_enabled: {
                        let field_value = match fields_map.get("enhancedSecurityMonitoringEnabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'enhancedSecurityMonitoringEnabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
