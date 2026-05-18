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
    > + Send {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "custom_fields",
                    &self.r#custom_fields,
                ),
                to_pulumi_object_field(
                    "managed_fields",
                    &self.r#managed_fields,
                ),
                to_pulumi_object_field(
                    "thing_group_indexing_mode",
                    &self.r#thing_group_indexing_mode,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
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
