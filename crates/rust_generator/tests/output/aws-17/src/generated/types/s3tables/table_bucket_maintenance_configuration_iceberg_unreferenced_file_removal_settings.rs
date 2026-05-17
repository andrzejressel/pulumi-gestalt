#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct TableBucketMaintenanceConfigurationIcebergUnreferencedFileRemovalSettings {
    /// Data objects marked for deletion are deleted after this many days.
    /// Must be at least `1`.
    #[builder(into)]
    #[serde(rename = "nonCurrentDays")]
    pub r#non_current_days: f64,
    /// Unreferenced data objects are marked for deletion after this many days.
    /// Must be at least `1`.
    #[builder(into)]
    #[serde(rename = "unreferencedDays")]
    pub r#unreferenced_days: f64,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for TableBucketMaintenanceConfigurationIcebergUnreferencedFileRemovalSettings {
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
                "non_current_days".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#non_current_days,
                )
                .await,
            );
            map.insert(
                "unreferenced_days".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#unreferenced_days,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for TableBucketMaintenanceConfigurationIcebergUnreferencedFileRemovalSettings {
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
                    r#non_current_days: {
                        let field_value = match fields_map.get("non_current_days") {
                            Some(value) => value,
                            None => bail!("Missing field 'non_current_days' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#unreferenced_days: {
                        let field_value = match fields_map.get("unreferenced_days") {
                            Some(value) => value,
                            None => bail!("Missing field 'unreferenced_days' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
