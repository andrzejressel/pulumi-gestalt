#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct TableMaintenanceConfiguration {
    /// A single Iceberg compaction settings block.
    /// See `iceberg_compaction` below
    #[builder(into)]
    #[serde(rename = "icebergCompaction")]
    pub r#iceberg_compaction: Box<super::super::types::s3tables::TableMaintenanceConfigurationIcebergCompaction>,
    /// A single Iceberg snapshot management settings block.
    /// See `iceberg_snapshot_management` below
    #[builder(into)]
    #[serde(rename = "icebergSnapshotManagement")]
    pub r#iceberg_snapshot_management: Box<super::super::types::s3tables::TableMaintenanceConfigurationIcebergSnapshotManagement>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for TableMaintenanceConfiguration {
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
                "iceberg_compaction".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#iceberg_compaction,
                )
                .await,
            );
            map.insert(
                "iceberg_snapshot_management".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#iceberg_snapshot_management,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for TableMaintenanceConfiguration {
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
                    r#iceberg_compaction: {
                        let field_value = match fields_map.get("iceberg_compaction") {
                            Some(value) => value,
                            None => bail!("Missing field 'iceberg_compaction' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#iceberg_snapshot_management: {
                        let field_value = match fields_map.get("iceberg_snapshot_management") {
                            Some(value) => value,
                            None => bail!("Missing field 'iceberg_snapshot_management' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
