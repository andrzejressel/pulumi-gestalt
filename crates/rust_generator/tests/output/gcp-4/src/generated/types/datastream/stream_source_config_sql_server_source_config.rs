#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct StreamSourceConfigSqlServerSourceConfig {
    /// CDC reader reads from change tables.
    #[builder(into)]
    #[serde(rename = "changeTables")]
    pub r#change_tables: Option<Box<super::super::types::datastream::StreamSourceConfigSqlServerSourceConfigChangeTables>>,
    /// SQL Server objects to exclude from the stream.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "excludeObjects")]
    pub r#exclude_objects: Option<Box<super::super::types::datastream::StreamSourceConfigSqlServerSourceConfigExcludeObjects>>,
    /// SQL Server objects to retrieve from the source.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "includeObjects")]
    pub r#include_objects: Option<Box<super::super::types::datastream::StreamSourceConfigSqlServerSourceConfigIncludeObjects>>,
    /// Max concurrent backfill tasks.
    #[builder(into)]
    #[serde(rename = "maxConcurrentBackfillTasks")]
    pub r#max_concurrent_backfill_tasks: Option<i32>,
    /// Max concurrent CDC tasks.
    #[builder(into)]
    #[serde(rename = "maxConcurrentCdcTasks")]
    pub r#max_concurrent_cdc_tasks: Option<i32>,
    /// CDC reader reads from transaction logs.
    #[builder(into)]
    #[serde(rename = "transactionLogs")]
    pub r#transaction_logs: Option<Box<super::super::types::datastream::StreamSourceConfigSqlServerSourceConfigTransactionLogs>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for StreamSourceConfigSqlServerSourceConfig {
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
                "change_tables".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#change_tables,
                )
                .await,
            );
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
            map.insert(
                "transaction_logs".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#transaction_logs,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for StreamSourceConfigSqlServerSourceConfig {
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
                    r#change_tables: {
                        let field_value = match fields_map.get("change_tables") {
                            Some(value) => value,
                            None => bail!("Missing field 'change_tables' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
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
                    r#transaction_logs: {
                        let field_value = match fields_map.get("transaction_logs") {
                            Some(value) => value,
                            None => bail!("Missing field 'transaction_logs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
