#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct StageCanarySettings {
    /// ID of the deployment that the canary points to.
    #[builder(into)]
    #[serde(rename = "deploymentId")]
    pub r#deployment_id: String,
    /// Percent `0.0` - `100.0` of traffic to divert to the canary deployment.
    #[builder(into)]
    #[serde(rename = "percentTraffic")]
    pub r#percent_traffic: Option<f64>,
    /// Map of overridden stage `variables` (including new variables) for the canary deployment.
    #[builder(into)]
    #[serde(rename = "stageVariableOverrides")]
    pub r#stage_variable_overrides: Option<std::collections::HashMap<String, String>>,
    /// Whether the canary deployment uses the stage cache. Defaults to false.
    #[builder(into)]
    #[serde(rename = "useStageCache")]
    pub r#use_stage_cache: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for StageCanarySettings {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("deployment_id".to_string(), self.r#deployment_id.to_pulumi_value().await);
            map.insert("percent_traffic".to_string(), self.r#percent_traffic.to_pulumi_value().await);
            map.insert("stage_variable_overrides".to_string(), self.r#stage_variable_overrides.to_pulumi_value().await);
            map.insert("use_stage_cache".to_string(), self.r#use_stage_cache.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for StageCanarySettings {
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
                    r#deployment_id: {
                        let field_value = match fields_map.get("deployment_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'deployment_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#percent_traffic: {
                        let field_value = match fields_map.get("percent_traffic") {
                            Some(value) => value,
                            None => bail!("Missing field 'percent_traffic' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<f64> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#stage_variable_overrides: {
                        let field_value = match fields_map.get("stage_variable_overrides") {
                            Some(value) => value,
                            None => bail!("Missing field 'stage_variable_overrides' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<std::collections::HashMap<String, String>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#use_stage_cache: {
                        let field_value = match fields_map.get("use_stage_cache") {
                            Some(value) => value,
                            None => bail!("Missing field 'use_stage_cache' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<bool> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
