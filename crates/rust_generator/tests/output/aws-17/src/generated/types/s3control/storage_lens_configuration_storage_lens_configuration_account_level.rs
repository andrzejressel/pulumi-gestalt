#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct StorageLensConfigurationStorageLensConfigurationAccountLevel {
    /// S3 Storage Lens activity metrics. See Activity Metrics below for more details.
    #[builder(into)]
    pub r#activity_metrics: Option<Box<super::super::types::s3control::StorageLensConfigurationStorageLensConfigurationAccountLevelActivityMetrics>>,
    /// Advanced cost-optimization metrics for S3 Storage Lens. See Advanced Cost-Optimization Metrics below for more details.
    #[builder(into)]
    pub r#advanced_cost_optimization_metrics: Option<Box<super::super::types::s3control::StorageLensConfigurationStorageLensConfigurationAccountLevelAdvancedCostOptimizationMetrics>>,
    /// Advanced data-protection metrics for S3 Storage Lens. See Advanced Data-Protection Metrics below for more details.
    #[builder(into)]
    pub r#advanced_data_protection_metrics: Option<Box<super::super::types::s3control::StorageLensConfigurationStorageLensConfigurationAccountLevelAdvancedDataProtectionMetrics>>,
    /// S3 Storage Lens bucket-level configuration. See Bucket Level below for more details.
    #[builder(into)]
    pub r#bucket_level: Box<super::super::types::s3control::StorageLensConfigurationStorageLensConfigurationAccountLevelBucketLevel>,
    /// Detailed status code metrics for S3 Storage Lens. See Detailed Status Code Metrics below for more details.
    #[builder(into)]
    pub r#detailed_status_code_metrics: Option<Box<super::super::types::s3control::StorageLensConfigurationStorageLensConfigurationAccountLevelDetailedStatusCodeMetrics>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for StorageLensConfigurationStorageLensConfigurationAccountLevel {
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
                    "activityMetrics",
                    &self.r#activity_metrics,
                ),
                to_pulumi_object_field(
                    "advancedCostOptimizationMetrics",
                    &self.r#advanced_cost_optimization_metrics,
                ),
                to_pulumi_object_field(
                    "advancedDataProtectionMetrics",
                    &self.r#advanced_data_protection_metrics,
                ),
                to_pulumi_object_field(
                    "bucketLevel",
                    &self.r#bucket_level,
                ),
                to_pulumi_object_field(
                    "detailedStatusCodeMetrics",
                    &self.r#detailed_status_code_metrics,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for StorageLensConfigurationStorageLensConfigurationAccountLevel {
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
                        let field_value = match fields_map.get("activityMetrics") {
                            Some(value) => value,
                            None => bail!("Missing field 'activityMetrics' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#advanced_cost_optimization_metrics: {
                        let field_value = match fields_map.get("advancedCostOptimizationMetrics") {
                            Some(value) => value,
                            None => bail!("Missing field 'advancedCostOptimizationMetrics' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#advanced_data_protection_metrics: {
                        let field_value = match fields_map.get("advancedDataProtectionMetrics") {
                            Some(value) => value,
                            None => bail!("Missing field 'advancedDataProtectionMetrics' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#bucket_level: {
                        let field_value = match fields_map.get("bucketLevel") {
                            Some(value) => value,
                            None => bail!("Missing field 'bucketLevel' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#detailed_status_code_metrics: {
                        let field_value = match fields_map.get("detailedStatusCodeMetrics") {
                            Some(value) => value,
                            None => bail!("Missing field 'detailedStatusCodeMetrics' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
