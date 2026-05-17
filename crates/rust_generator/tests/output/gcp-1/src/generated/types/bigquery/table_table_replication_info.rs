#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct TableTableReplicationInfo {
    /// The interval at which the source
    /// materialized view is polled for updates. The default is 300000.
    #[builder(into)]
    #[serde(rename = "replicationIntervalMs")]
    pub r#replication_interval_ms: Option<i32>,
    /// The ID of the source dataset.
    #[builder(into)]
    #[serde(rename = "sourceDatasetId")]
    pub r#source_dataset_id: String,
    /// The ID of the source project.
    #[builder(into)]
    #[serde(rename = "sourceProjectId")]
    pub r#source_project_id: String,
    /// The ID of the source materialized view.
    #[builder(into)]
    #[serde(rename = "sourceTableId")]
    pub r#source_table_id: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for TableTableReplicationInfo {
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
                "replication_interval_ms".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#replication_interval_ms,
                )
                .await,
            );
            map.insert(
                "source_dataset_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#source_dataset_id,
                )
                .await,
            );
            map.insert(
                "source_project_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#source_project_id,
                )
                .await,
            );
            map.insert(
                "source_table_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#source_table_id,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for TableTableReplicationInfo {
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
                    r#replication_interval_ms: {
                        let field_value = match fields_map.get("replication_interval_ms") {
                            Some(value) => value,
                            None => bail!("Missing field 'replication_interval_ms' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#source_dataset_id: {
                        let field_value = match fields_map.get("source_dataset_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'source_dataset_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#source_project_id: {
                        let field_value = match fields_map.get("source_project_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'source_project_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#source_table_id: {
                        let field_value = match fields_map.get("source_table_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'source_table_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
