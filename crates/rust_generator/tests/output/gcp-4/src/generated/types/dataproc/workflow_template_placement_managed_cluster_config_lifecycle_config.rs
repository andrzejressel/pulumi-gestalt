#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct WorkflowTemplatePlacementManagedClusterConfigLifecycleConfig {
    /// The time when cluster will be auto-deleted (see JSON representation of [JSON Mapping - Language Guide (proto 3)](https://developers.google.com/protocol-buffers/docs/proto3#json)).
    #[builder(into)]
    #[serde(rename = "autoDeleteTime")]
    pub r#auto_delete_time: Option<String>,
    /// The lifetime duration of cluster. The cluster will be auto-deleted at the end of this period. Minimum value is 10 minutes; maximum value is 14 days (see JSON representation of [JSON Mapping - Language Guide (proto 3)](https://developers.google.com/protocol-buffers/docs/proto3#json)).
    #[builder(into)]
    #[serde(rename = "autoDeleteTtl")]
    pub r#auto_delete_ttl: Option<String>,
    /// The duration to keep the cluster alive while idling (when no jobs are running). Passing this threshold will cause the cluster to be deleted. Minimum value is 5 minutes; maximum value is 14 days (see JSON representation of [JSON Mapping - Language Guide (proto 3)](https://developers.google.com/protocol-buffers/docs/proto3#json).
    #[builder(into)]
    #[serde(rename = "idleDeleteTtl")]
    pub r#idle_delete_ttl: Option<String>,
    /// Output only. The time when cluster became idle (most recent job finished) and became eligible for deletion due to idleness (see JSON representation of [JSON Mapping - Language Guide (proto 3)](https://developers.google.com/protocol-buffers/docs/proto3#json)).
    #[builder(into)]
    #[serde(rename = "idleStartTime")]
    pub r#idle_start_time: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for WorkflowTemplatePlacementManagedClusterConfigLifecycleConfig {
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
                "auto_delete_time".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#auto_delete_time,
                )
                .await,
            );
            map.insert(
                "auto_delete_ttl".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#auto_delete_ttl,
                )
                .await,
            );
            map.insert(
                "idle_delete_ttl".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#idle_delete_ttl,
                )
                .await,
            );
            map.insert(
                "idle_start_time".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#idle_start_time,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for WorkflowTemplatePlacementManagedClusterConfigLifecycleConfig {
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
                    r#auto_delete_time: {
                        let field_value = match fields_map.get("auto_delete_time") {
                            Some(value) => value,
                            None => bail!("Missing field 'auto_delete_time' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#auto_delete_ttl: {
                        let field_value = match fields_map.get("auto_delete_ttl") {
                            Some(value) => value,
                            None => bail!("Missing field 'auto_delete_ttl' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#idle_delete_ttl: {
                        let field_value = match fields_map.get("idle_delete_ttl") {
                            Some(value) => value,
                            None => bail!("Missing field 'idle_delete_ttl' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#idle_start_time: {
                        let field_value = match fields_map.get("idle_start_time") {
                            Some(value) => value,
                            None => bail!("Missing field 'idle_start_time' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
