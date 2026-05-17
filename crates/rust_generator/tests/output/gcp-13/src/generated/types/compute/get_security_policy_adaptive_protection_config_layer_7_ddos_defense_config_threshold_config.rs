#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetSecurityPolicyAdaptiveProtectionConfigLayer7DdosDefenseConfigThresholdConfig {
    #[builder(into)]
    #[serde(rename = "autoDeployConfidenceThreshold")]
    pub r#auto_deploy_confidence_threshold: f64,
    #[builder(into)]
    #[serde(rename = "autoDeployExpirationSec")]
    pub r#auto_deploy_expiration_sec: i32,
    #[builder(into)]
    #[serde(rename = "autoDeployImpactedBaselineThreshold")]
    pub r#auto_deploy_impacted_baseline_threshold: f64,
    #[builder(into)]
    #[serde(rename = "autoDeployLoadThreshold")]
    pub r#auto_deploy_load_threshold: f64,
    #[builder(into)]
    #[serde(rename = "detectionAbsoluteQps")]
    pub r#detection_absolute_qps: f64,
    #[builder(into)]
    #[serde(rename = "detectionLoadThreshold")]
    pub r#detection_load_threshold: f64,
    #[builder(into)]
    #[serde(rename = "detectionRelativeToBaselineQps")]
    pub r#detection_relative_to_baseline_qps: f64,
    /// The name of the security policy. Provide either this or a `self_link`.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    #[builder(into)]
    #[serde(rename = "trafficGranularityConfigs")]
    pub r#traffic_granularity_configs: Vec<super::super::types::compute::GetSecurityPolicyAdaptiveProtectionConfigLayer7DdosDefenseConfigThresholdConfigTrafficGranularityConfig>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetSecurityPolicyAdaptiveProtectionConfigLayer7DdosDefenseConfigThresholdConfig {
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
                "auto_deploy_confidence_threshold".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#auto_deploy_confidence_threshold,
                )
                .await,
            );
            map.insert(
                "auto_deploy_expiration_sec".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#auto_deploy_expiration_sec,
                )
                .await,
            );
            map.insert(
                "auto_deploy_impacted_baseline_threshold".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#auto_deploy_impacted_baseline_threshold,
                )
                .await,
            );
            map.insert(
                "auto_deploy_load_threshold".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#auto_deploy_load_threshold,
                )
                .await,
            );
            map.insert(
                "detection_absolute_qps".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#detection_absolute_qps,
                )
                .await,
            );
            map.insert(
                "detection_load_threshold".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#detection_load_threshold,
                )
                .await,
            );
            map.insert(
                "detection_relative_to_baseline_qps".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#detection_relative_to_baseline_qps,
                )
                .await,
            );
            map.insert(
                "name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#name,
                )
                .await,
            );
            map.insert(
                "traffic_granularity_configs".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#traffic_granularity_configs,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetSecurityPolicyAdaptiveProtectionConfigLayer7DdosDefenseConfigThresholdConfig {
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
                    r#auto_deploy_confidence_threshold: {
                        let field_value = match fields_map.get("auto_deploy_confidence_threshold") {
                            Some(value) => value,
                            None => bail!("Missing field 'auto_deploy_confidence_threshold' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#auto_deploy_expiration_sec: {
                        let field_value = match fields_map.get("auto_deploy_expiration_sec") {
                            Some(value) => value,
                            None => bail!("Missing field 'auto_deploy_expiration_sec' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#auto_deploy_impacted_baseline_threshold: {
                        let field_value = match fields_map.get("auto_deploy_impacted_baseline_threshold") {
                            Some(value) => value,
                            None => bail!("Missing field 'auto_deploy_impacted_baseline_threshold' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#auto_deploy_load_threshold: {
                        let field_value = match fields_map.get("auto_deploy_load_threshold") {
                            Some(value) => value,
                            None => bail!("Missing field 'auto_deploy_load_threshold' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#detection_absolute_qps: {
                        let field_value = match fields_map.get("detection_absolute_qps") {
                            Some(value) => value,
                            None => bail!("Missing field 'detection_absolute_qps' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#detection_load_threshold: {
                        let field_value = match fields_map.get("detection_load_threshold") {
                            Some(value) => value,
                            None => bail!("Missing field 'detection_load_threshold' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#detection_relative_to_baseline_qps: {
                        let field_value = match fields_map.get("detection_relative_to_baseline_qps") {
                            Some(value) => value,
                            None => bail!("Missing field 'detection_relative_to_baseline_qps' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#name: {
                        let field_value = match fields_map.get("name") {
                            Some(value) => value,
                            None => bail!("Missing field 'name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#traffic_granularity_configs: {
                        let field_value = match fields_map.get("traffic_granularity_configs") {
                            Some(value) => value,
                            None => bail!("Missing field 'traffic_granularity_configs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
