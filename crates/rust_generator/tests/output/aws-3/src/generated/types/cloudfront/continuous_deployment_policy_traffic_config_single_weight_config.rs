#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ContinuousDeploymentPolicyTrafficConfigSingleWeightConfig {
    /// Session stickiness provides the ability to define multiple requests from a single viewer as a single session. This prevents the potentially inconsistent experience of sending some of a given user's requests to the staging distribution, while others are sent to the primary distribution. Define the session duration using TTL values. See `session_stickiness_config`.
    #[builder(into)]
    #[serde(rename = "sessionStickinessConfig")]
    pub r#session_stickiness_config: Option<Box<super::super::types::cloudfront::ContinuousDeploymentPolicyTrafficConfigSingleWeightConfigSessionStickinessConfig>>,
    /// The percentage of traffic to send to a staging distribution, expressed as a decimal number between `0` and `.15`.
    #[builder(into)]
    #[serde(rename = "weight")]
    pub r#weight: f64,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ContinuousDeploymentPolicyTrafficConfigSingleWeightConfig {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("session_stickiness_config".to_string(), self.r#session_stickiness_config.to_pulumi_value().await);
            map.insert("weight".to_string(), self.r#weight.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ContinuousDeploymentPolicyTrafficConfigSingleWeightConfig {
    fn from_pulumi_value(
        value: &pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    ) -> pulumi_gestalt_rust::__private::rootcause::Result<Self> {
        use std::collections::BTreeMap;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValueContent;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue;
        use pulumi_gestalt_rust::__private::rootcause::bail;

        match value.content {
            PulumiValueContent::Object(ref obj) => {
                let fields_map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> =
                    obj.iter().cloned().collect();

                Ok(Self {
                    r#session_stickiness_config: {
                        let field_value = match fields_map.get("session_stickiness_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'session_stickiness_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::cloudfront::ContinuousDeploymentPolicyTrafficConfigSingleWeightConfigSessionStickinessConfig>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#weight: {
                        let field_value = match fields_map.get("weight") {
                            Some(value) => value,
                            None => bail!("Missing field 'weight' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <f64 as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
