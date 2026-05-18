#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct TopicRuleCloudwatchAlarm {
    /// The CloudWatch alarm name.
    #[builder(into)]
    #[serde(rename = "alarmName")]
    pub r#alarm_name: String,
    /// The IAM role ARN that allows access to the CloudWatch alarm.
    #[builder(into)]
    #[serde(rename = "roleArn")]
    pub r#role_arn: String,
    /// The reason for the alarm change.
    #[builder(into)]
    #[serde(rename = "stateReason")]
    pub r#state_reason: String,
    /// The value of the alarm state. Acceptable values are: OK, ALARM, INSUFFICIENT_DATA.
    #[builder(into)]
    #[serde(rename = "stateValue")]
    pub r#state_value: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for TopicRuleCloudwatchAlarm {
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
                    "alarm_name",
                    &self.r#alarm_name,
                ),
                to_pulumi_object_field(
                    "role_arn",
                    &self.r#role_arn,
                ),
                to_pulumi_object_field(
                    "state_reason",
                    &self.r#state_reason,
                ),
                to_pulumi_object_field(
                    "state_value",
                    &self.r#state_value,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for TopicRuleCloudwatchAlarm {
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
                    r#alarm_name: {
                        let field_value = match fields_map.get("alarm_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'alarm_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#role_arn: {
                        let field_value = match fields_map.get("role_arn") {
                            Some(value) => value,
                            None => bail!("Missing field 'role_arn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#state_reason: {
                        let field_value = match fields_map.get("state_reason") {
                            Some(value) => value,
                            None => bail!("Missing field 'state_reason' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#state_value: {
                        let field_value = match fields_map.get("state_value") {
                            Some(value) => value,
                            None => bail!("Missing field 'state_value' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
