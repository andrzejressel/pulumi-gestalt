#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ReceiptRuleBounceAction {
    /// The message to send
    #[builder(into)]
    #[serde(rename = "message")]
    pub r#message: String,
    /// The position of the action in the receipt rule
    #[builder(into)]
    #[serde(rename = "position")]
    pub r#position: i32,
    /// The email address of the sender
    #[builder(into)]
    #[serde(rename = "sender")]
    pub r#sender: String,
    /// The RFC 5321 SMTP reply code
    #[builder(into)]
    #[serde(rename = "smtpReplyCode")]
    pub r#smtp_reply_code: String,
    /// The RFC 3463 SMTP enhanced status code
    #[builder(into)]
    #[serde(rename = "statusCode")]
    pub r#status_code: Option<String>,
    /// The ARN of an SNS topic to notify
    #[builder(into)]
    #[serde(rename = "topicArn")]
    pub r#topic_arn: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ReceiptRuleBounceAction {
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
                "message".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#message,
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
                "sender".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#sender,
                )
                .await,
            );
            map.insert(
                "smtp_reply_code".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#smtp_reply_code,
                )
                .await,
            );
            map.insert(
                "status_code".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#status_code,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ReceiptRuleBounceAction {
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
                    r#message: {
                        let field_value = match fields_map.get("message") {
                            Some(value) => value,
                            None => bail!("Missing field 'message' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#sender: {
                        let field_value = match fields_map.get("sender") {
                            Some(value) => value,
                            None => bail!("Missing field 'sender' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#smtp_reply_code: {
                        let field_value = match fields_map.get("smtp_reply_code") {
                            Some(value) => value,
                            None => bail!("Missing field 'smtp_reply_code' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#status_code: {
                        let field_value = match fields_map.get("status_code") {
                            Some(value) => value,
                            None => bail!("Missing field 'status_code' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
