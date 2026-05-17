#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct LaunchMetricMonitorMetricDefinition {
    /// Specifies the entity, such as a user or session, that does an action that causes a metric value to be recorded. An example is `userDetails.userID`.
    #[builder(into)]
    #[serde(rename = "entityIdKey")]
    pub r#entity_id_key: String,
    /// Specifies The EventBridge event pattern that defines how the metric is recorded.
    #[builder(into)]
    #[serde(rename = "eventPattern")]
    pub r#event_pattern: Option<String>,
    /// Specifies the name for the metric.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// Specifies a label for the units that the metric is measuring.
    #[builder(into)]
    #[serde(rename = "unitLabel")]
    pub r#unit_label: Option<String>,
    /// Specifies the value that is tracked to produce the metric.
    #[builder(into)]
    #[serde(rename = "valueKey")]
    pub r#value_key: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for LaunchMetricMonitorMetricDefinition {
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
                "entity_id_key".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#entity_id_key,
                )
                .await,
            );
            map.insert(
                "event_pattern".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#event_pattern,
                )
                .await,
            );
            map.insert(
                "name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#name,
                )
                .await,
            );
            map.insert(
                "unit_label".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#unit_label,
                )
                .await,
            );
            map.insert(
                "value_key".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#value_key,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for LaunchMetricMonitorMetricDefinition {
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
                    r#entity_id_key: {
                        let field_value = match fields_map.get("entity_id_key") {
                            Some(value) => value,
                            None => bail!("Missing field 'entity_id_key' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#event_pattern: {
                        let field_value = match fields_map.get("event_pattern") {
                            Some(value) => value,
                            None => bail!("Missing field 'event_pattern' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#name: {
                        let field_value = match fields_map.get("name") {
                            Some(value) => value,
                            None => bail!("Missing field 'name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#unit_label: {
                        let field_value = match fields_map.get("unit_label") {
                            Some(value) => value,
                            None => bail!("Missing field 'unit_label' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#value_key: {
                        let field_value = match fields_map.get("value_key") {
                            Some(value) => value,
                            None => bail!("Missing field 'value_key' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
