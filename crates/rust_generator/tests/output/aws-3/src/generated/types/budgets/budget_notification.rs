#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct BudgetNotification {
    /// (Required) Comparison operator to use to evaluate the condition. Can be `LESS_THAN`, `EQUAL_TO` or `GREATER_THAN`.
    #[builder(into)]
    pub r#comparison_operator: String,
    /// (Required) What kind of budget value to notify on. Can be `ACTUAL` or `FORECASTED`
    #[builder(into)]
    pub r#notification_type: String,
    /// (Optional) E-Mail addresses to notify. Either this or `subscriber_sns_topic_arns` is required.
    #[builder(into)]
    pub r#subscriber_email_addresses: Option<Vec<String>>,
    /// (Optional) SNS topics to notify. Either this or `subscriber_email_addresses` is required.
    #[builder(into)]
    pub r#subscriber_sns_topic_arns: Option<Vec<String>>,
    /// (Required) Threshold when the notification should be sent.
    #[builder(into)]
    pub r#threshold: f64,
    /// (Required) What kind of threshold is defined. Can be `PERCENTAGE` OR `ABSOLUTE_VALUE`.
    #[builder(into)]
    pub r#threshold_type: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for BudgetNotification {
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
                    "comparisonOperator",
                    &self.r#comparison_operator,
                ),
                to_pulumi_object_field(
                    "notificationType",
                    &self.r#notification_type,
                ),
                to_pulumi_object_field(
                    "subscriberEmailAddresses",
                    &self.r#subscriber_email_addresses,
                ),
                to_pulumi_object_field(
                    "subscriberSnsTopicArns",
                    &self.r#subscriber_sns_topic_arns,
                ),
                to_pulumi_object_field(
                    "threshold",
                    &self.r#threshold,
                ),
                to_pulumi_object_field(
                    "thresholdType",
                    &self.r#threshold_type,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for BudgetNotification {
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
                    r#comparison_operator: {
                        let field_value = match fields_map.get("comparisonOperator") {
                            Some(value) => value,
                            None => bail!("Missing field 'comparisonOperator' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#notification_type: {
                        let field_value = match fields_map.get("notificationType") {
                            Some(value) => value,
                            None => bail!("Missing field 'notificationType' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#subscriber_email_addresses: {
                        let field_value = match fields_map.get("subscriberEmailAddresses") {
                            Some(value) => value,
                            None => bail!("Missing field 'subscriberEmailAddresses' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#subscriber_sns_topic_arns: {
                        let field_value = match fields_map.get("subscriberSnsTopicArns") {
                            Some(value) => value,
                            None => bail!("Missing field 'subscriberSnsTopicArns' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#threshold: {
                        let field_value = match fields_map.get("threshold") {
                            Some(value) => value,
                            None => bail!("Missing field 'threshold' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#threshold_type: {
                        let field_value = match fields_map.get("thresholdType") {
                            Some(value) => value,
                            None => bail!("Missing field 'thresholdType' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
