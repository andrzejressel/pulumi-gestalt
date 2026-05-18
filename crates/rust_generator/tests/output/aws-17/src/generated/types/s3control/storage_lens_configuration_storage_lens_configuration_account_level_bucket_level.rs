#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct StorageLensConfigurationStorageLensConfigurationAccountLevelBucketLevel {
    /// S3 Storage Lens activity metrics. See Activity Metrics above for more details.
    #[builder(into)]
    #[serde(rename = "activityMetrics")]
    pub r#activity_metrics: Option<Box<super::super::types::s3control::StorageLensConfigurationStorageLensConfigurationAccountLevelBucketLevelActivityMetrics>>,
    /// Advanced cost-optimization metrics for S3 Storage Lens. See Advanced Cost-Optimization Metrics above for more details.
    #[builder(into)]
    #[serde(rename = "advancedCostOptimizationMetrics")]
    pub r#advanced_cost_optimization_metrics: Option<Box<super::super::types::s3control::StorageLensConfigurationStorageLensConfigurationAccountLevelBucketLevelAdvancedCostOptimizationMetrics>>,
    /// Advanced data-protection metrics for S3 Storage Lens. See Advanced Data-Protection Metrics above for more details.
    #[builder(into)]
    #[serde(rename = "advancedDataProtectionMetrics")]
    pub r#advanced_data_protection_metrics: Option<Box<super::super::types::s3control::StorageLensConfigurationStorageLensConfigurationAccountLevelBucketLevelAdvancedDataProtectionMetrics>>,
    /// Detailed status code metrics for S3 Storage Lens. See Detailed Status Code Metrics above for more details.
    #[builder(into)]
    #[serde(rename = "detailedStatusCodeMetrics")]
    pub r#detailed_status_code_metrics: Option<Box<super::super::types::s3control::StorageLensConfigurationStorageLensConfigurationAccountLevelBucketLevelDetailedStatusCodeMetrics>>,
    /// Prefix-level metrics for S3 Storage Lens. See Prefix Level below for more details.
    #[builder(into)]
    #[serde(rename = "prefixLevel")]
    pub r#prefix_level: Option<Box<super::super::types::s3control::StorageLensConfigurationStorageLensConfigurationAccountLevelBucketLevelPrefixLevel>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for StorageLensConfigurationStorageLensConfigurationAccountLevelBucketLevel {
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
                    "activity_metrics",
                    &self.r#activity_metrics,
                ),
                to_pulumi_object_field(
                    "advanced_cost_optimization_metrics",
                    &self.r#advanced_cost_optimization_metrics,
                ),
                to_pulumi_object_field(
                    "advanced_data_protection_metrics",
                    &self.r#advanced_data_protection_metrics,
                ),
                to_pulumi_object_field(
                    "detailed_status_code_metrics",
                    &self.r#detailed_status_code_metrics,
                ),
                to_pulumi_object_field(
                    "prefix_level",
                    &self.r#prefix_level,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for StorageLensConfigurationStorageLensConfigurationAccountLevelBucketLevel {
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
                    r#activity_metrics: {
                        let field_value = match fields_map.get("activity_metrics") {
                            Some(value) => value,
                            None => bail!("Missing field 'activity_metrics' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#advanced_cost_optimization_metrics: {
                        let field_value = match fields_map.get("advanced_cost_optimization_metrics") {
                            Some(value) => value,
                            None => bail!("Missing field 'advanced_cost_optimization_metrics' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#advanced_data_protection_metrics: {
                        let field_value = match fields_map.get("advanced_data_protection_metrics") {
                            Some(value) => value,
                            None => bail!("Missing field 'advanced_data_protection_metrics' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#detailed_status_code_metrics: {
                        let field_value = match fields_map.get("detailed_status_code_metrics") {
                            Some(value) => value,
                            None => bail!("Missing field 'detailed_status_code_metrics' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#prefix_level: {
                        let field_value = match fields_map.get("prefix_level") {
                            Some(value) => value,
                            None => bail!("Missing field 'prefix_level' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
