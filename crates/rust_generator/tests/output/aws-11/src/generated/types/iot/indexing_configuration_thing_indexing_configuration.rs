#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct IndexingConfigurationThingIndexingConfiguration {
    /// Contains custom field names and their data type. See below.
    #[builder(into)]
    #[serde(rename = "customFields")]
    pub r#custom_fields: Option<Vec<super::super::types::iot::IndexingConfigurationThingIndexingConfigurationCustomField>>,
    /// Device Defender indexing mode. Valid values: `VIOLATIONS`, `OFF`. Default: `OFF`.
    #[builder(into)]
    #[serde(rename = "deviceDefenderIndexingMode")]
    pub r#device_defender_indexing_mode: Option<String>,
    /// Required if `named_shadow_indexing_mode` is `ON`. Enables to add named shadows filtered by `filter` to fleet indexing configuration.
    #[builder(into)]
    #[serde(rename = "filter")]
    pub r#filter: Option<Box<super::super::types::iot::IndexingConfigurationThingIndexingConfigurationFilter>>,
    /// Contains fields that are indexed and whose types are already known by the Fleet Indexing service. See below.
    #[builder(into)]
    #[serde(rename = "managedFields")]
    pub r#managed_fields: Option<Vec<super::super::types::iot::IndexingConfigurationThingIndexingConfigurationManagedField>>,
    /// [Named shadow](https://docs.aws.amazon.com/iot/latest/developerguide/iot-device-shadows.html) indexing mode. Valid values: `ON`, `OFF`. Default: `OFF`.
    #[builder(into)]
    #[serde(rename = "namedShadowIndexingMode")]
    pub r#named_shadow_indexing_mode: Option<String>,
    /// Thing connectivity indexing mode. Valid values: `STATUS`, `OFF`. Default: `OFF`.
    #[builder(into)]
    #[serde(rename = "thingConnectivityIndexingMode")]
    pub r#thing_connectivity_indexing_mode: Option<String>,
    /// Thing indexing mode. Valid values: `REGISTRY`, `REGISTRY_AND_SHADOW`, `OFF`.
    #[builder(into)]
    #[serde(rename = "thingIndexingMode")]
    pub r#thing_indexing_mode: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for IndexingConfigurationThingIndexingConfiguration {
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
                "custom_fields".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#custom_fields,
                )
                .await,
            );
            map.insert(
                "device_defender_indexing_mode".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#device_defender_indexing_mode,
                )
                .await,
            );
            map.insert(
                "filter".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#filter,
                )
                .await,
            );
            map.insert(
                "managed_fields".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#managed_fields,
                )
                .await,
            );
            map.insert(
                "named_shadow_indexing_mode".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#named_shadow_indexing_mode,
                )
                .await,
            );
            map.insert(
                "thing_connectivity_indexing_mode".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#thing_connectivity_indexing_mode,
                )
                .await,
            );
            map.insert(
                "thing_indexing_mode".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#thing_indexing_mode,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for IndexingConfigurationThingIndexingConfiguration {
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
                    r#custom_fields: {
                        let field_value = match fields_map.get("custom_fields") {
                            Some(value) => value,
                            None => bail!("Missing field 'custom_fields' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#device_defender_indexing_mode: {
                        let field_value = match fields_map.get("device_defender_indexing_mode") {
                            Some(value) => value,
                            None => bail!("Missing field 'device_defender_indexing_mode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#filter: {
                        let field_value = match fields_map.get("filter") {
                            Some(value) => value,
                            None => bail!("Missing field 'filter' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#managed_fields: {
                        let field_value = match fields_map.get("managed_fields") {
                            Some(value) => value,
                            None => bail!("Missing field 'managed_fields' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#named_shadow_indexing_mode: {
                        let field_value = match fields_map.get("named_shadow_indexing_mode") {
                            Some(value) => value,
                            None => bail!("Missing field 'named_shadow_indexing_mode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#thing_connectivity_indexing_mode: {
                        let field_value = match fields_map.get("thing_connectivity_indexing_mode") {
                            Some(value) => value,
                            None => bail!("Missing field 'thing_connectivity_indexing_mode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#thing_indexing_mode: {
                        let field_value = match fields_map.get("thing_indexing_mode") {
                            Some(value) => value,
                            None => bail!("Missing field 'thing_indexing_mode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
