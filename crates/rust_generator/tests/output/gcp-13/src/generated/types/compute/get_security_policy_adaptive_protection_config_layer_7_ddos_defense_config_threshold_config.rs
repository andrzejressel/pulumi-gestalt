#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetSecurityPolicyAdaptiveProtectionConfigLayer7DdosDefenseConfigThresholdConfig {
    #[builder(into)]
    pub r#auto_deploy_confidence_threshold: f64,
    #[builder(into)]
    pub r#auto_deploy_expiration_sec: i32,
    #[builder(into)]
    pub r#auto_deploy_impacted_baseline_threshold: f64,
    #[builder(into)]
    pub r#auto_deploy_load_threshold: f64,
    #[builder(into)]
    pub r#detection_absolute_qps: f64,
    #[builder(into)]
    pub r#detection_load_threshold: f64,
    #[builder(into)]
    pub r#detection_relative_to_baseline_qps: f64,
    /// The name of the security policy. Provide either this or a `self_link`.
    #[builder(into)]
    pub r#name: String,
    #[builder(into)]
    pub r#traffic_granularity_configs: Vec<super::super::types::compute::GetSecurityPolicyAdaptiveProtectionConfigLayer7DdosDefenseConfigThresholdConfigTrafficGranularityConfig>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetSecurityPolicyAdaptiveProtectionConfigLayer7DdosDefenseConfigThresholdConfig {
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
                    "autoDeployConfidenceThreshold",
                    &self.r#auto_deploy_confidence_threshold,
                ),
                to_pulumi_object_field(
                    "autoDeployExpirationSec",
                    &self.r#auto_deploy_expiration_sec,
                ),
                to_pulumi_object_field(
                    "autoDeployImpactedBaselineThreshold",
                    &self.r#auto_deploy_impacted_baseline_threshold,
                ),
                to_pulumi_object_field(
                    "autoDeployLoadThreshold",
                    &self.r#auto_deploy_load_threshold,
                ),
                to_pulumi_object_field(
                    "detectionAbsoluteQps",
                    &self.r#detection_absolute_qps,
                ),
                to_pulumi_object_field(
                    "detectionLoadThreshold",
                    &self.r#detection_load_threshold,
                ),
                to_pulumi_object_field(
                    "detectionRelativeToBaselineQps",
                    &self.r#detection_relative_to_baseline_qps,
                ),
                to_pulumi_object_field(
                    "name",
                    &self.r#name,
                ),
                to_pulumi_object_field(
                    "trafficGranularityConfigs",
                    &self.r#traffic_granularity_configs,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
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
                        let field_value = match fields_map.get("autoDeployConfidenceThreshold") {
                            Some(value) => value,
                            None => bail!("Missing field 'autoDeployConfidenceThreshold' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#auto_deploy_expiration_sec: {
                        let field_value = match fields_map.get("autoDeployExpirationSec") {
                            Some(value) => value,
                            None => bail!("Missing field 'autoDeployExpirationSec' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#auto_deploy_impacted_baseline_threshold: {
                        let field_value = match fields_map.get("autoDeployImpactedBaselineThreshold") {
                            Some(value) => value,
                            None => bail!("Missing field 'autoDeployImpactedBaselineThreshold' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#auto_deploy_load_threshold: {
                        let field_value = match fields_map.get("autoDeployLoadThreshold") {
                            Some(value) => value,
                            None => bail!("Missing field 'autoDeployLoadThreshold' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#detection_absolute_qps: {
                        let field_value = match fields_map.get("detectionAbsoluteQps") {
                            Some(value) => value,
                            None => bail!("Missing field 'detectionAbsoluteQps' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#detection_load_threshold: {
                        let field_value = match fields_map.get("detectionLoadThreshold") {
                            Some(value) => value,
                            None => bail!("Missing field 'detectionLoadThreshold' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#detection_relative_to_baseline_qps: {
                        let field_value = match fields_map.get("detectionRelativeToBaselineQps") {
                            Some(value) => value,
                            None => bail!("Missing field 'detectionRelativeToBaselineQps' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("trafficGranularityConfigs") {
                            Some(value) => value,
                            None => bail!("Missing field 'trafficGranularityConfigs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
