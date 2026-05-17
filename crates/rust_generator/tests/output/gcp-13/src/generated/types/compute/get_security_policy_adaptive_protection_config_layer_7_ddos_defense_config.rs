#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetSecurityPolicyAdaptiveProtectionConfigLayer7DdosDefenseConfig {
    /// If set to true, enables CAAP for L7 DDoS detection.
    #[builder(into)]
    #[serde(rename = "enable")]
    pub r#enable: bool,
    /// Rule visibility. Supported values include: "STANDARD", "PREMIUM".
    #[builder(into)]
    #[serde(rename = "ruleVisibility")]
    pub r#rule_visibility: String,
    /// Configuration options for layer7 adaptive protection for various customizable thresholds.
    #[builder(into)]
    #[serde(rename = "thresholdConfigs")]
    pub r#threshold_configs: Vec<super::super::types::compute::GetSecurityPolicyAdaptiveProtectionConfigLayer7DdosDefenseConfigThresholdConfig>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetSecurityPolicyAdaptiveProtectionConfigLayer7DdosDefenseConfig {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "enable",
                    &self.r#enable,
                ),
                to_pulumi_object_field(
                    "rule_visibility",
                    &self.r#rule_visibility,
                ),
                to_pulumi_object_field(
                    "threshold_configs",
                    &self.r#threshold_configs,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetSecurityPolicyAdaptiveProtectionConfigLayer7DdosDefenseConfig {
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
                    r#enable: {
                        let field_value = match fields_map.get("enable") {
                            Some(value) => value,
                            None => bail!("Missing field 'enable' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#rule_visibility: {
                        let field_value = match fields_map.get("rule_visibility") {
                            Some(value) => value,
                            None => bail!("Missing field 'rule_visibility' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#threshold_configs: {
                        let field_value = match fields_map.get("threshold_configs") {
                            Some(value) => value,
                            None => bail!("Missing field 'threshold_configs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
