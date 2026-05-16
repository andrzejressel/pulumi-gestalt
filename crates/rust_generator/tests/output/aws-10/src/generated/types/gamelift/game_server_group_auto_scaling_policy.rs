#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GameServerGroupAutoScalingPolicy {
    /// Length of time, in seconds, it takes for a new instance to start
    /// new game server processes and register with GameLift FleetIQ.
    /// Specifying a warm-up time can be useful, particularly with game servers that take a long time to start up,
    /// because it avoids prematurely starting new instances. Defaults to `60`.
    #[builder(into)]
    #[serde(rename = "estimatedInstanceWarmup")]
    pub r#estimated_instance_warmup: Option<i32>,
    #[builder(into)]
    #[serde(rename = "targetTrackingConfiguration")]
    pub r#target_tracking_configuration: Box<super::super::types::gamelift::GameServerGroupAutoScalingPolicyTargetTrackingConfiguration>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GameServerGroupAutoScalingPolicy {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("estimated_instance_warmup".to_string(), self.r#estimated_instance_warmup.to_pulumi_value().await);
            map.insert("target_tracking_configuration".to_string(), self.r#target_tracking_configuration.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GameServerGroupAutoScalingPolicy {
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
                    r#estimated_instance_warmup: {
                        let field_value = match fields_map.get("estimated_instance_warmup") {
                            Some(value) => value,
                            None => bail!("Missing field 'estimated_instance_warmup' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<i32> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#target_tracking_configuration: {
                        let field_value = match fields_map.get("target_tracking_configuration") {
                            Some(value) => value,
                            None => bail!("Missing field 'target_tracking_configuration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Box<super::super::types::gamelift::GameServerGroupAutoScalingPolicyTargetTrackingConfiguration> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
