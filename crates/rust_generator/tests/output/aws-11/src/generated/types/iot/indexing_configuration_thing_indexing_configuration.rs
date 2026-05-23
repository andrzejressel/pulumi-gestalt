#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct IndexingConfigurationThingIndexingConfiguration {
    /// Contains custom field names and their data type. See below.
    #[builder(into)]
    pub r#custom_fields: Option<Vec<super::super::types::iot::IndexingConfigurationThingIndexingConfigurationCustomField>>,
    /// Device Defender indexing mode. Valid values: `VIOLATIONS`, `OFF`. Default: `OFF`.
    #[builder(into)]
    pub r#device_defender_indexing_mode: Option<String>,
    /// Required if `named_shadow_indexing_mode` is `ON`. Enables to add named shadows filtered by `filter` to fleet indexing configuration.
    #[builder(into)]
    pub r#filter: Option<Box<super::super::types::iot::IndexingConfigurationThingIndexingConfigurationFilter>>,
    /// Contains fields that are indexed and whose types are already known by the Fleet Indexing service. See below.
    #[builder(into)]
    pub r#managed_fields: Option<Vec<super::super::types::iot::IndexingConfigurationThingIndexingConfigurationManagedField>>,
    /// [Named shadow](https://docs.aws.amazon.com/iot/latest/developerguide/iot-device-shadows.html) indexing mode. Valid values: `ON`, `OFF`. Default: `OFF`.
    #[builder(into)]
    pub r#named_shadow_indexing_mode: Option<String>,
    /// Thing connectivity indexing mode. Valid values: `STATUS`, `OFF`. Default: `OFF`.
    #[builder(into)]
    pub r#thing_connectivity_indexing_mode: Option<String>,
    /// Thing indexing mode. Valid values: `REGISTRY`, `REGISTRY_AND_SHADOW`, `OFF`.
    #[builder(into)]
    pub r#thing_indexing_mode: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for IndexingConfigurationThingIndexingConfiguration {
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
                    "customFields",
                    &self.r#custom_fields,
                ),
                to_pulumi_object_field(
                    "deviceDefenderIndexingMode",
                    &self.r#device_defender_indexing_mode,
                ),
                to_pulumi_object_field(
                    "filter",
                    &self.r#filter,
                ),
                to_pulumi_object_field(
                    "managedFields",
                    &self.r#managed_fields,
                ),
                to_pulumi_object_field(
                    "namedShadowIndexingMode",
                    &self.r#named_shadow_indexing_mode,
                ),
                to_pulumi_object_field(
                    "thingConnectivityIndexingMode",
                    &self.r#thing_connectivity_indexing_mode,
                ),
                to_pulumi_object_field(
                    "thingIndexingMode",
                    &self.r#thing_indexing_mode,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
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
                        let field_value = match fields_map.get("customFields") {
                            Some(value) => value,
                            None => bail!("Missing field 'customFields' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#device_defender_indexing_mode: {
                        let field_value = match fields_map.get("deviceDefenderIndexingMode") {
                            Some(value) => value,
                            None => bail!("Missing field 'deviceDefenderIndexingMode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("managedFields") {
                            Some(value) => value,
                            None => bail!("Missing field 'managedFields' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#named_shadow_indexing_mode: {
                        let field_value = match fields_map.get("namedShadowIndexingMode") {
                            Some(value) => value,
                            None => bail!("Missing field 'namedShadowIndexingMode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#thing_connectivity_indexing_mode: {
                        let field_value = match fields_map.get("thingConnectivityIndexingMode") {
                            Some(value) => value,
                            None => bail!("Missing field 'thingConnectivityIndexingMode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#thing_indexing_mode: {
                        let field_value = match fields_map.get("thingIndexingMode") {
                            Some(value) => value,
                            None => bail!("Missing field 'thingIndexingMode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
