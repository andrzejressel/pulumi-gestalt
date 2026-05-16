#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DeploymentConfigTrafficRoutingConfig {
    /// The time based canary configuration information. If `type` is `TimeBasedLinear`, use `time_based_linear` instead.
    #[builder(into)]
    #[serde(rename = "timeBasedCanary")]
    pub r#time_based_canary: Option<Box<super::super::types::codedeploy::DeploymentConfigTrafficRoutingConfigTimeBasedCanary>>,
    /// The time based linear configuration information. If `type` is `TimeBasedCanary`, use `time_based_canary` instead.
    #[builder(into)]
    #[serde(rename = "timeBasedLinear")]
    pub r#time_based_linear: Option<Box<super::super::types::codedeploy::DeploymentConfigTrafficRoutingConfigTimeBasedLinear>>,
    /// Type of traffic routing config. One of `TimeBasedCanary`, `TimeBasedLinear`, `AllAtOnce`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for DeploymentConfigTrafficRoutingConfig {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("time_based_canary".to_string(), self.r#time_based_canary.to_pulumi_value().await);
            map.insert("time_based_linear".to_string(), self.r#time_based_linear.to_pulumi_value().await);
            map.insert("type_".to_string(), self.r#type_.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for DeploymentConfigTrafficRoutingConfig {
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
                    r#time_based_canary: {
                        let field_value = match fields_map.get("time_based_canary") {
                            Some(value) => value,
                            None => bail!("Missing field 'time_based_canary' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::codedeploy::DeploymentConfigTrafficRoutingConfigTimeBasedCanary>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#time_based_linear: {
                        let field_value = match fields_map.get("time_based_linear") {
                            Some(value) => value,
                            None => bail!("Missing field 'time_based_linear' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::codedeploy::DeploymentConfigTrafficRoutingConfigTimeBasedLinear>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#type_: {
                        let field_value = match fields_map.get("type_") {
                            Some(value) => value,
                            None => bail!("Missing field 'type_' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
