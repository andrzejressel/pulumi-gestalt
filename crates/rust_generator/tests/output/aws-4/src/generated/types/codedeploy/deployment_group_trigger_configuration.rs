#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DeploymentGroupTriggerConfiguration {
    /// The event type or types for which notifications are triggered. Some values that are supported: `DeploymentStart`, `DeploymentSuccess`, `DeploymentFailure`, `DeploymentStop`, `DeploymentRollback`, `InstanceStart`, `InstanceSuccess`, `InstanceFailure`.  See [the CodeDeploy documentation](http://docs.aws.amazon.com/codedeploy/latest/userguide/monitoring-sns-event-notifications-create-trigger.html) for all possible values.
    #[builder(into)]
    #[serde(rename = "triggerEvents")]
    pub r#trigger_events: Vec<String>,
    /// The name of the notification trigger.
    #[builder(into)]
    #[serde(rename = "triggerName")]
    pub r#trigger_name: String,
    /// The ARN of the SNS topic through which notifications are sent.
    #[builder(into)]
    #[serde(rename = "triggerTargetArn")]
    pub r#trigger_target_arn: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for DeploymentGroupTriggerConfiguration {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("trigger_events".to_string(), self.r#trigger_events.to_pulumi_value().await);
            map.insert("trigger_name".to_string(), self.r#trigger_name.to_pulumi_value().await);
            map.insert("trigger_target_arn".to_string(), self.r#trigger_target_arn.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for DeploymentGroupTriggerConfiguration {
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
                    r#trigger_events: {
                        let field_value = match fields_map.get("trigger_events") {
                            Some(value) => value,
                            None => bail!("Missing field 'trigger_events' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Vec<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#trigger_name: {
                        let field_value = match fields_map.get("trigger_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'trigger_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#trigger_target_arn: {
                        let field_value = match fields_map.get("trigger_target_arn") {
                            Some(value) => value,
                            None => bail!("Missing field 'trigger_target_arn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
