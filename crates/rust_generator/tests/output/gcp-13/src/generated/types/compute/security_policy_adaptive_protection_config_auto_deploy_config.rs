#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct SecurityPolicyAdaptiveProtectionConfigAutoDeployConfig {
    /// Rules are only automatically deployed for alerts on potential attacks with confidence scores greater than this threshold.
    #[builder(into)]
    #[serde(rename = "confidenceThreshold")]
    pub r#confidence_threshold: Option<f64>,
    /// Google Cloud Armor stops applying the action in the automatically deployed rule to an identified attacker after this duration. The rule continues to operate against new requests.
    #[builder(into)]
    #[serde(rename = "expirationSec")]
    pub r#expiration_sec: Option<i32>,
    /// Rules are only automatically deployed when the estimated impact to baseline traffic from the suggested mitigation is below this threshold.
    #[builder(into)]
    #[serde(rename = "impactedBaselineThreshold")]
    pub r#impacted_baseline_threshold: Option<f64>,
    /// Identifies new attackers only when the load to the backend service that is under attack exceeds this threshold.
    #[builder(into)]
    #[serde(rename = "loadThreshold")]
    pub r#load_threshold: Option<f64>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for SecurityPolicyAdaptiveProtectionConfigAutoDeployConfig {
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
                    "confidenceThreshold",
                    &self.r#confidence_threshold,
                ),
                to_pulumi_object_field(
                    "expirationSec",
                    &self.r#expiration_sec,
                ),
                to_pulumi_object_field(
                    "impactedBaselineThreshold",
                    &self.r#impacted_baseline_threshold,
                ),
                to_pulumi_object_field(
                    "loadThreshold",
                    &self.r#load_threshold,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for SecurityPolicyAdaptiveProtectionConfigAutoDeployConfig {
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
                    r#confidence_threshold: {
                        let field_value = match fields_map.get("confidenceThreshold") {
                            Some(value) => value,
                            None => bail!("Missing field 'confidenceThreshold' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#expiration_sec: {
                        let field_value = match fields_map.get("expirationSec") {
                            Some(value) => value,
                            None => bail!("Missing field 'expirationSec' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#impacted_baseline_threshold: {
                        let field_value = match fields_map.get("impactedBaselineThreshold") {
                            Some(value) => value,
                            None => bail!("Missing field 'impactedBaselineThreshold' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#load_threshold: {
                        let field_value = match fields_map.get("loadThreshold") {
                            Some(value) => value,
                            None => bail!("Missing field 'loadThreshold' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
