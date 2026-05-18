#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DataSourceDynamodbConfigDeltaSyncConfig {
    /// The number of minutes that an Item is stored in the data source.
    #[builder(into)]
    #[serde(rename = "baseTableTtl")]
    pub r#base_table_ttl: Option<i32>,
    /// The table name.
    #[builder(into)]
    #[serde(rename = "deltaSyncTableName")]
    pub r#delta_sync_table_name: String,
    /// The number of minutes that a Delta Sync log entry is stored in the Delta Sync table.
    #[builder(into)]
    #[serde(rename = "deltaSyncTableTtl")]
    pub r#delta_sync_table_ttl: Option<i32>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for DataSourceDynamodbConfigDeltaSyncConfig {
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
                    "base_table_ttl",
                    &self.r#base_table_ttl,
                ),
                to_pulumi_object_field(
                    "delta_sync_table_name",
                    &self.r#delta_sync_table_name,
                ),
                to_pulumi_object_field(
                    "delta_sync_table_ttl",
                    &self.r#delta_sync_table_ttl,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for DataSourceDynamodbConfigDeltaSyncConfig {
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
                    r#base_table_ttl: {
                        let field_value = match fields_map.get("base_table_ttl") {
                            Some(value) => value,
                            None => bail!("Missing field 'base_table_ttl' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#delta_sync_table_name: {
                        let field_value = match fields_map.get("delta_sync_table_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'delta_sync_table_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#delta_sync_table_ttl: {
                        let field_value = match fields_map.get("delta_sync_table_ttl") {
                            Some(value) => value,
                            None => bail!("Missing field 'delta_sync_table_ttl' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
