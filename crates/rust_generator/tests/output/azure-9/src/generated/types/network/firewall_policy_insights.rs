#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FirewallPolicyInsights {
    /// The ID of the default Log Analytics Workspace that the Firewalls associated with this Firewall Policy will send their logs to, when there is no location matches in the `log_analytics_workspace`.
    #[builder(into)]
    #[serde(rename = "defaultLogAnalyticsWorkspaceId")]
    pub r#default_log_analytics_workspace_id: String,
    /// Whether the insights functionality is enabled for this Firewall Policy.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: bool,
    /// A list of `log_analytics_workspace` block as defined below.
    #[builder(into)]
    #[serde(rename = "logAnalyticsWorkspaces")]
    pub r#log_analytics_workspaces: Option<Vec<super::super::types::network::FirewallPolicyInsightsLogAnalyticsWorkspace>>,
    /// The log retention period in days.
    #[builder(into)]
    #[serde(rename = "retentionInDays")]
    pub r#retention_in_days: Option<i32>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for FirewallPolicyInsights {
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
                "default_log_analytics_workspace_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#default_log_analytics_workspace_id,
                )
                .await,
            );
            map.insert(
                "enabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#enabled,
                )
                .await,
            );
            map.insert(
                "log_analytics_workspaces".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#log_analytics_workspaces,
                )
                .await,
            );
            map.insert(
                "retention_in_days".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#retention_in_days,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for FirewallPolicyInsights {
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
                    r#default_log_analytics_workspace_id: {
                        let field_value = match fields_map.get("default_log_analytics_workspace_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'default_log_analytics_workspace_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#log_analytics_workspaces: {
                        let field_value = match fields_map.get("log_analytics_workspaces") {
                            Some(value) => value,
                            None => bail!("Missing field 'log_analytics_workspaces' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#retention_in_days: {
                        let field_value = match fields_map.get("retention_in_days") {
                            Some(value) => value,
                            None => bail!("Missing field 'retention_in_days' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
