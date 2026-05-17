#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ReceiptRuleLambdaAction {
    /// The ARN of the Lambda function to invoke
    #[builder(into)]
    #[serde(rename = "functionArn")]
    pub r#function_arn: String,
    /// `Event` or `RequestResponse`
    #[builder(into)]
    #[serde(rename = "invocationType")]
    pub r#invocation_type: Option<String>,
    /// The position of the action in the receipt rule
    #[builder(into)]
    #[serde(rename = "position")]
    pub r#position: i32,
    /// The ARN of an SNS topic to notify
    #[builder(into)]
    #[serde(rename = "topicArn")]
    pub r#topic_arn: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ReceiptRuleLambdaAction {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "function_arn".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#function_arn,
                )
                .await,
            );
            map.insert(
                "invocation_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#invocation_type,
                )
                .await,
            );
            map.insert(
                "position".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#position,
                )
                .await,
            );
            map.insert(
                "topic_arn".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#topic_arn,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ReceiptRuleLambdaAction {
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
                    r#function_arn: {
                        let field_value = match fields_map.get("function_arn") {
                            Some(value) => value,
                            None => bail!("Missing field 'function_arn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#invocation_type: {
                        let field_value = match fields_map.get("invocation_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'invocation_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#position: {
                        let field_value = match fields_map.get("position") {
                            Some(value) => value,
                            None => bail!("Missing field 'position' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#topic_arn: {
                        let field_value = match fields_map.get("topic_arn") {
                            Some(value) => value,
                            None => bail!("Missing field 'topic_arn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
