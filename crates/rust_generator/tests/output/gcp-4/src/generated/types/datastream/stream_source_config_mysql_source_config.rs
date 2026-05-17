#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct StreamSourceConfigMysqlSourceConfig {
    /// MySQL objects to exclude from the stream.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "excludeObjects")]
    pub r#exclude_objects: Option<Box<super::super::types::datastream::StreamSourceConfigMysqlSourceConfigExcludeObjects>>,
    /// MySQL objects to retrieve from the source.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "includeObjects")]
    pub r#include_objects: Option<Box<super::super::types::datastream::StreamSourceConfigMysqlSourceConfigIncludeObjects>>,
    /// Maximum number of concurrent backfill tasks. The number should be non negative.
    /// If not set (or set to 0), the system's default value will be used.
    #[builder(into)]
    #[serde(rename = "maxConcurrentBackfillTasks")]
    pub r#max_concurrent_backfill_tasks: Option<i32>,
    /// Maximum number of concurrent CDC tasks. The number should be non negative.
    /// If not set (or set to 0), the system's default value will be used.
    #[builder(into)]
    #[serde(rename = "maxConcurrentCdcTasks")]
    pub r#max_concurrent_cdc_tasks: Option<i32>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for StreamSourceConfigMysqlSourceConfig {
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
                "exclude_objects".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#exclude_objects,
                )
                .await,
            );
            map.insert(
                "include_objects".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#include_objects,
                )
                .await,
            );
            map.insert(
                "max_concurrent_backfill_tasks".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#max_concurrent_backfill_tasks,
                )
                .await,
            );
            map.insert(
                "max_concurrent_cdc_tasks".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#max_concurrent_cdc_tasks,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for StreamSourceConfigMysqlSourceConfig {
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
                    r#exclude_objects: {
                        let field_value = match fields_map.get("exclude_objects") {
                            Some(value) => value,
                            None => bail!("Missing field 'exclude_objects' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#include_objects: {
                        let field_value = match fields_map.get("include_objects") {
                            Some(value) => value,
                            None => bail!("Missing field 'include_objects' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_concurrent_backfill_tasks: {
                        let field_value = match fields_map.get("max_concurrent_backfill_tasks") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_concurrent_backfill_tasks' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_concurrent_cdc_tasks: {
                        let field_value = match fields_map.get("max_concurrent_cdc_tasks") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_concurrent_cdc_tasks' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
