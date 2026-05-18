#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AiFeatureStoreEntityTypeMonitoringConfig {
    /// Threshold for categorical features of anomaly detection. This is shared by all types of Featurestore Monitoring for categorical features (i.e. Features with type (Feature.ValueType) BOOL or STRING).
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "categoricalThresholdConfig")]
    pub r#categorical_threshold_config: Option<Box<super::super::types::vertex::AiFeatureStoreEntityTypeMonitoringConfigCategoricalThresholdConfig>>,
    /// The config for ImportFeatures Analysis Based Feature Monitoring.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "importFeaturesAnalysis")]
    pub r#import_features_analysis: Option<Box<super::super::types::vertex::AiFeatureStoreEntityTypeMonitoringConfigImportFeaturesAnalysis>>,
    /// Threshold for numerical features of anomaly detection. This is shared by all objectives of Featurestore Monitoring for numerical features (i.e. Features with type (Feature.ValueType) DOUBLE or INT64).
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "numericalThresholdConfig")]
    pub r#numerical_threshold_config: Option<Box<super::super::types::vertex::AiFeatureStoreEntityTypeMonitoringConfigNumericalThresholdConfig>>,
    /// The config for Snapshot Analysis Based Feature Monitoring.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "snapshotAnalysis")]
    pub r#snapshot_analysis: Option<Box<super::super::types::vertex::AiFeatureStoreEntityTypeMonitoringConfigSnapshotAnalysis>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for AiFeatureStoreEntityTypeMonitoringConfig {
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
                    "categorical_threshold_config",
                    &self.r#categorical_threshold_config,
                ),
                to_pulumi_object_field(
                    "import_features_analysis",
                    &self.r#import_features_analysis,
                ),
                to_pulumi_object_field(
                    "numerical_threshold_config",
                    &self.r#numerical_threshold_config,
                ),
                to_pulumi_object_field(
                    "snapshot_analysis",
                    &self.r#snapshot_analysis,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for AiFeatureStoreEntityTypeMonitoringConfig {
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
                    r#categorical_threshold_config: {
                        let field_value = match fields_map.get("categorical_threshold_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'categorical_threshold_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#import_features_analysis: {
                        let field_value = match fields_map.get("import_features_analysis") {
                            Some(value) => value,
                            None => bail!("Missing field 'import_features_analysis' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#numerical_threshold_config: {
                        let field_value = match fields_map.get("numerical_threshold_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'numerical_threshold_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#snapshot_analysis: {
                        let field_value = match fields_map.get("snapshot_analysis") {
                            Some(value) => value,
                            None => bail!("Missing field 'snapshot_analysis' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
