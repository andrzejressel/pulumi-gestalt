#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetKubernetesClusterOmsAgent {
    /// The ID of the Log Analytics Workspace to which the OMS Agent should send data.
    #[builder(into)]
    #[serde(rename = "logAnalyticsWorkspaceId")]
    pub r#log_analytics_workspace_id: String,
    /// Is managed identity authentication for monitoring enabled?
    #[builder(into)]
    #[serde(rename = "msiAuthForMonitoringEnabled")]
    pub r#msi_auth_for_monitoring_enabled: bool,
    /// An `oms_agent_identity` block as defined below.
    #[builder(into)]
    #[serde(rename = "omsAgentIdentities")]
    pub r#oms_agent_identities: Vec<super::super::types::containerservice::GetKubernetesClusterOmsAgentOmsAgentIdentity>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetKubernetesClusterOmsAgent {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_concurrent, to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "log_analytics_workspace_id",
                    &self.r#log_analytics_workspace_id,
                ),
                to_pulumi_object_field(
                    "msi_auth_for_monitoring_enabled",
                    &self.r#msi_auth_for_monitoring_enabled,
                ),
                to_pulumi_object_field(
                    "oms_agent_identities",
                    &self.r#oms_agent_identities,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetKubernetesClusterOmsAgent {
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
                    r#log_analytics_workspace_id: {
                        let field_value = match fields_map.get("log_analytics_workspace_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'log_analytics_workspace_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#msi_auth_for_monitoring_enabled: {
                        let field_value = match fields_map.get("msi_auth_for_monitoring_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'msi_auth_for_monitoring_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#oms_agent_identities: {
                        let field_value = match fields_map.get("oms_agent_identities") {
                            Some(value) => value,
                            None => bail!("Missing field 'oms_agent_identities' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
