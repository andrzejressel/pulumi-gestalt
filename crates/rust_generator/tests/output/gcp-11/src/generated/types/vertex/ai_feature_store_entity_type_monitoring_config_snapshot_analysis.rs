#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AiFeatureStoreEntityTypeMonitoringConfigSnapshotAnalysis {
    /// The monitoring schedule for snapshot analysis. For EntityType-level config: unset / disabled = true indicates disabled by default for Features under it; otherwise by default enable snapshot analysis monitoring with monitoringInterval for Features under it.
    #[builder(into)]
    #[serde(rename = "disabled")]
    pub r#disabled: Option<bool>,
    /// Configuration of the snapshot analysis based monitoring pipeline running interval. The value is rolled up to full day.
    /// A duration in seconds with up to nine fractional digits, terminated by 's'. Example: "3.5s".
    /// 
    /// > **Warning:** `monitoring_interval` is deprecated and will be removed in a future release.
    #[builder(into)]
    #[serde(rename = "monitoringInterval")]
    pub r#monitoring_interval: Option<String>,
    /// Configuration of the snapshot analysis based monitoring pipeline running interval. The value indicates number of days. The default value is 1.
    /// If both FeaturestoreMonitoringConfig.SnapshotAnalysis.monitoring_interval_days and [FeaturestoreMonitoringConfig.SnapshotAnalysis.monitoring_interval][] are set when creating/updating EntityTypes/Features, FeaturestoreMonitoringConfig.SnapshotAnalysis.monitoring_interval_days will be used.
    #[builder(into)]
    #[serde(rename = "monitoringIntervalDays")]
    pub r#monitoring_interval_days: Option<i32>,
    /// Customized export features time window for snapshot analysis. Unit is one day. The default value is 21 days. Minimum value is 1 day. Maximum value is 4000 days.
    #[builder(into)]
    #[serde(rename = "stalenessDays")]
    pub r#staleness_days: Option<i32>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for AiFeatureStoreEntityTypeMonitoringConfigSnapshotAnalysis {
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
                "disabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#disabled,
                )
                .await,
            );
            map.insert(
                "monitoring_interval".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#monitoring_interval,
                )
                .await,
            );
            map.insert(
                "monitoring_interval_days".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#monitoring_interval_days,
                )
                .await,
            );
            map.insert(
                "staleness_days".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#staleness_days,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for AiFeatureStoreEntityTypeMonitoringConfigSnapshotAnalysis {
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
                    r#disabled: {
                        let field_value = match fields_map.get("disabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'disabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#monitoring_interval: {
                        let field_value = match fields_map.get("monitoring_interval") {
                            Some(value) => value,
                            None => bail!("Missing field 'monitoring_interval' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#monitoring_interval_days: {
                        let field_value = match fields_map.get("monitoring_interval_days") {
                            Some(value) => value,
                            None => bail!("Missing field 'monitoring_interval_days' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#staleness_days: {
                        let field_value = match fields_map.get("staleness_days") {
                            Some(value) => value,
                            None => bail!("Missing field 'staleness_days' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
