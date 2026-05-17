#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct CatalogTableOptimizerConfigurationRetentionConfigurationIcebergConfiguration {
    /// If set to `false`, snapshots are only deleted from table metadata, and the underlying data and metadata files are not deleted. Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "cleanExpiredFiles")]
    pub r#clean_expired_files: Option<bool>,
    /// The number of Iceberg snapshots to retain within the retention period. Defaults to `1` or the corresponding Iceberg table configuration field if it exists.
    #[builder(into)]
    #[serde(rename = "numberOfSnapshotsToRetain")]
    pub r#number_of_snapshots_to_retain: Option<f64>,
    /// The number of days to retain the Iceberg snapshots. Defaults to `5`, or the corresponding Iceberg table configuration field if it exists.
    #[builder(into)]
    #[serde(rename = "snapshotRetentionPeriodInDays")]
    pub r#snapshot_retention_period_in_days: Option<f64>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for CatalogTableOptimizerConfigurationRetentionConfigurationIcebergConfiguration {
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
                "clean_expired_files".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#clean_expired_files,
                )
                .await,
            );
            map.insert(
                "number_of_snapshots_to_retain".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#number_of_snapshots_to_retain,
                )
                .await,
            );
            map.insert(
                "snapshot_retention_period_in_days".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#snapshot_retention_period_in_days,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for CatalogTableOptimizerConfigurationRetentionConfigurationIcebergConfiguration {
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
                    r#clean_expired_files: {
                        let field_value = match fields_map.get("clean_expired_files") {
                            Some(value) => value,
                            None => bail!("Missing field 'clean_expired_files' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#number_of_snapshots_to_retain: {
                        let field_value = match fields_map.get("number_of_snapshots_to_retain") {
                            Some(value) => value,
                            None => bail!("Missing field 'number_of_snapshots_to_retain' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#snapshot_retention_period_in_days: {
                        let field_value = match fields_map.get("snapshot_retention_period_in_days") {
                            Some(value) => value,
                            None => bail!("Missing field 'snapshot_retention_period_in_days' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
