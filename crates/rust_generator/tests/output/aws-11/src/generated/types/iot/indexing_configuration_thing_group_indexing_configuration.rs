#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct IndexingConfigurationThingGroupIndexingConfiguration {
    /// A list of thing group fields to index. This list cannot contain any managed fields. See below.
    #[builder(into)]
    #[serde(rename = "customFields")]
    pub r#custom_fields: Option<Vec<super::super::types::iot::IndexingConfigurationThingGroupIndexingConfigurationCustomField>>,
    /// Contains fields that are indexed and whose types are already known by the Fleet Indexing service. See below.
    #[builder(into)]
    #[serde(rename = "managedFields")]
    pub r#managed_fields: Option<Vec<super::super::types::iot::IndexingConfigurationThingGroupIndexingConfigurationManagedField>>,
    /// Thing group indexing mode. Valid values: `OFF`, `ON`.
    #[builder(into)]
    #[serde(rename = "thingGroupIndexingMode")]
    pub r#thing_group_indexing_mode: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for IndexingConfigurationThingGroupIndexingConfiguration {
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
                "managed_fields".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#managed_fields,
                )
                .await,
            );
            map.insert(
                "thing_group_indexing_mode".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#thing_group_indexing_mode,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for IndexingConfigurationThingGroupIndexingConfiguration {
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
                    r#managed_fields: {
                        let field_value = match fields_map.get("managed_fields") {
                            Some(value) => value,
                            None => bail!("Missing field 'managed_fields' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#thing_group_indexing_mode: {
                        let field_value = match fields_map.get("thing_group_indexing_mode") {
                            Some(value) => value,
                            None => bail!("Missing field 'thing_group_indexing_mode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
