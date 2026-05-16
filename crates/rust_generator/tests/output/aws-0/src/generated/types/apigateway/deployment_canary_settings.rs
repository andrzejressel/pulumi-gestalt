#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DeploymentCanarySettings {
    /// Percentage (0.0-100.0) of traffic routed to the canary deployment.
    #[builder(into)]
    #[serde(rename = "percentTraffic")]
    pub r#percent_traffic: Option<f64>,
    /// Stage variable overrides used for the canary release deployment. They can override existing stage variables or add new stage variables for the canary release deployment. These stage variables are represented as a string-to-string map between stage variable names and their values.
    #[builder(into)]
    #[serde(rename = "stageVariableOverrides")]
    pub r#stage_variable_overrides: Option<std::collections::HashMap<String, String>>,
    /// Boolean flag to indicate whether the canary release deployment uses the stage cache or not.
    #[builder(into)]
    #[serde(rename = "useStageCache")]
    pub r#use_stage_cache: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for DeploymentCanarySettings {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("percent_traffic".to_string(), self.r#percent_traffic.to_pulumi_value().await);
            map.insert("stage_variable_overrides".to_string(), self.r#stage_variable_overrides.to_pulumi_value().await);
            map.insert("use_stage_cache".to_string(), self.r#use_stage_cache.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for DeploymentCanarySettings {
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
