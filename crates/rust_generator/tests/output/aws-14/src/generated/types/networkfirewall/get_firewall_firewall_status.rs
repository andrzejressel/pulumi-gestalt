#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetFirewallFirewallStatus {
    /// Aggregated count of all resources used by reference sets in a firewall.
    #[builder(into)]
    #[serde(rename = "capacityUsageSummaries")]
    pub r#capacity_usage_summaries: Vec<super::super::types::networkfirewall::GetFirewallFirewallStatusCapacityUsageSummary>,
    /// Summary of sync states for all availability zones in which the firewall is configured.
    #[builder(into)]
    #[serde(rename = "configurationSyncStateSummary")]
    pub r#configuration_sync_state_summary: String,
    #[builder(into)]
    #[serde(rename = "status")]
    pub r#status: String,
    /// Set of subnets configured for use by the firewall.
    #[builder(into)]
    #[serde(rename = "syncStates")]
    pub r#sync_states: Vec<super::super::types::networkfirewall::GetFirewallFirewallStatusSyncState>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetFirewallFirewallStatus {
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
                    "capacity_usage_summaries",
                    &self.r#capacity_usage_summaries,
                ),
                to_pulumi_object_field(
                    "configuration_sync_state_summary",
                    &self.r#configuration_sync_state_summary,
                ),
                to_pulumi_object_field(
                    "status",
                    &self.r#status,
                ),
                to_pulumi_object_field(
                    "sync_states",
                    &self.r#sync_states,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetFirewallFirewallStatus {
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
                    r#capacity_usage_summaries: {
                        let field_value = match fields_map.get("capacity_usage_summaries") {
                            Some(value) => value,
                            None => bail!("Missing field 'capacity_usage_summaries' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#configuration_sync_state_summary: {
                        let field_value = match fields_map.get("configuration_sync_state_summary") {
                            Some(value) => value,
                            None => bail!("Missing field 'configuration_sync_state_summary' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#status: {
                        let field_value = match fields_map.get("status") {
                            Some(value) => value,
                            None => bail!("Missing field 'status' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#sync_states: {
                        let field_value = match fields_map.get("sync_states") {
                            Some(value) => value,
                            None => bail!("Missing field 'sync_states' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
