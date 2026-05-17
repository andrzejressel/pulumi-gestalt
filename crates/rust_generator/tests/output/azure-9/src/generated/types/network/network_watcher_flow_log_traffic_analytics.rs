#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct NetworkWatcherFlowLogTrafficAnalytics {
    /// Boolean flag to enable/disable traffic analytics.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: bool,
    /// How frequently service should do flow analytics in minutes. Defaults to `60`.
    #[builder(into)]
    #[serde(rename = "intervalInMinutes")]
    pub r#interval_in_minutes: Option<i32>,
    /// The resource GUID of the attached workspace.
    #[builder(into)]
    #[serde(rename = "workspaceId")]
    pub r#workspace_id: String,
    /// The location of the attached workspace.
    #[builder(into)]
    #[serde(rename = "workspaceRegion")]
    pub r#workspace_region: String,
    /// The resource ID of the attached workspace.
    #[builder(into)]
    #[serde(rename = "workspaceResourceId")]
    pub r#workspace_resource_id: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for NetworkWatcherFlowLogTrafficAnalytics {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "enabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#enabled,
                )
                .await,
            );
            map.insert(
                "interval_in_minutes".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#interval_in_minutes,
                )
                .await,
            );
            map.insert(
                "workspace_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#workspace_id,
                )
                .await,
            );
            map.insert(
                "workspace_region".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#workspace_region,
                )
                .await,
            );
            map.insert(
                "workspace_resource_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#workspace_resource_id,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for NetworkWatcherFlowLogTrafficAnalytics {
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
                    r#enabled: {
                        let field_value = match fields_map.get("enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#interval_in_minutes: {
                        let field_value = match fields_map.get("interval_in_minutes") {
                            Some(value) => value,
                            None => bail!("Missing field 'interval_in_minutes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#workspace_id: {
                        let field_value = match fields_map.get("workspace_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'workspace_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#workspace_region: {
                        let field_value = match fields_map.get("workspace_region") {
                            Some(value) => value,
                            None => bail!("Missing field 'workspace_region' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#workspace_resource_id: {
                        let field_value = match fields_map.get("workspace_resource_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'workspace_resource_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
