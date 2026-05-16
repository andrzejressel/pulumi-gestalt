#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct TableMaintenanceConfigurationIcebergSnapshotManagementSettings {
    /// Snapshots older than this will be marked for deletiion.
    /// Must be at least `1`.
    #[builder(into)]
    #[serde(rename = "maxSnapshotAgeHours")]
    pub r#max_snapshot_age_hours: f64,
    /// Minimum number of snapshots to keep.
    /// Must be at least `1`.
    #[builder(into)]
    #[serde(rename = "minSnapshotsToKeep")]
    pub r#min_snapshots_to_keep: f64,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for TableMaintenanceConfigurationIcebergSnapshotManagementSettings {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("max_snapshot_age_hours".to_string(), self.r#max_snapshot_age_hours.to_pulumi_value().await);
            map.insert("min_snapshots_to_keep".to_string(), self.r#min_snapshots_to_keep.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for TableMaintenanceConfigurationIcebergSnapshotManagementSettings {
    fn from_pulumi_value(
        value: &pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    ) -> pulumi_gestalt_rust::__private::rootcause::Result<Self> {
        use std::collections::BTreeMap;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValueContent;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue;
        use pulumi_gestalt_rust::__private::rootcause::bail;

        match value.content {
            PulumiValueContent::Object(ref obj) => {
                let fields_map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> =
                    obj.iter().cloned().collect();

                Ok(Self {
                    r#max_snapshot_age_hours: {
                        let field_value = match fields_map.get("max_snapshot_age_hours") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_snapshot_age_hours' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <f64 as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#min_snapshots_to_keep: {
                        let field_value = match fields_map.get("min_snapshots_to_keep") {
                            Some(value) => value,
                            None => bail!("Missing field 'min_snapshots_to_keep' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <f64 as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
