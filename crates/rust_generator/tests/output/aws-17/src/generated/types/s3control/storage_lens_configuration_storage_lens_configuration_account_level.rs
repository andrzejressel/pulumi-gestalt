#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct StorageLensConfigurationStorageLensConfigurationAccountLevel {
    /// S3 Storage Lens activity metrics. See Activity Metrics below for more details.
    #[builder(into)]
    #[serde(rename = "activityMetrics")]
    pub r#activity_metrics: Option<Box<super::super::types::s3control::StorageLensConfigurationStorageLensConfigurationAccountLevelActivityMetrics>>,
    /// Advanced cost-optimization metrics for S3 Storage Lens. See Advanced Cost-Optimization Metrics below for more details.
    #[builder(into)]
    #[serde(rename = "advancedCostOptimizationMetrics")]
    pub r#advanced_cost_optimization_metrics: Option<Box<super::super::types::s3control::StorageLensConfigurationStorageLensConfigurationAccountLevelAdvancedCostOptimizationMetrics>>,
    /// Advanced data-protection metrics for S3 Storage Lens. See Advanced Data-Protection Metrics below for more details.
    #[builder(into)]
    #[serde(rename = "advancedDataProtectionMetrics")]
    pub r#advanced_data_protection_metrics: Option<Box<super::super::types::s3control::StorageLensConfigurationStorageLensConfigurationAccountLevelAdvancedDataProtectionMetrics>>,
    /// S3 Storage Lens bucket-level configuration. See Bucket Level below for more details.
    #[builder(into)]
    #[serde(rename = "bucketLevel")]
    pub r#bucket_level: Box<super::super::types::s3control::StorageLensConfigurationStorageLensConfigurationAccountLevelBucketLevel>,
    /// Detailed status code metrics for S3 Storage Lens. See Detailed Status Code Metrics below for more details.
    #[builder(into)]
    #[serde(rename = "detailedStatusCodeMetrics")]
    pub r#detailed_status_code_metrics: Option<Box<super::super::types::s3control::StorageLensConfigurationStorageLensConfigurationAccountLevelDetailedStatusCodeMetrics>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for StorageLensConfigurationStorageLensConfigurationAccountLevel {
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
                "activity_metrics".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#activity_metrics,
                )
                .await,
            );
            map.insert(
                "advanced_cost_optimization_metrics".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#advanced_cost_optimization_metrics,
                )
                .await,
            );
            map.insert(
                "advanced_data_protection_metrics".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#advanced_data_protection_metrics,
                )
                .await,
            );
            map.insert(
                "bucket_level".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#bucket_level,
                )
                .await,
            );
            map.insert(
                "detailed_status_code_metrics".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#detailed_status_code_metrics,
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
                    r#bucket_level: {
                        let field_value = match fields_map.get("bucket_level") {
                            Some(value) => value,
                            None => bail!("Missing field 'bucket_level' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
