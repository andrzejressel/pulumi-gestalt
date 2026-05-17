#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct SubscriptionRuleCorrelationFilter {
    /// Content type of the message.
    #[builder(into)]
    #[serde(rename = "contentType")]
    pub r#content_type: Option<String>,
    /// Identifier of the correlation.
    #[builder(into)]
    #[serde(rename = "correlationId")]
    pub r#correlation_id: Option<String>,
    /// Application specific label.
    #[builder(into)]
    #[serde(rename = "label")]
    pub r#label: Option<String>,
    /// Identifier of the message.
    #[builder(into)]
    #[serde(rename = "messageId")]
    pub r#message_id: Option<String>,
    /// A list of user defined properties to be included in the filter. Specified as a map of name/value pairs.
    /// 
    /// > **NOTE:** When creating a subscription rule of type `CorrelationFilter` at least one property must be set in the `correlation_filter` block.
    #[builder(into)]
    #[serde(rename = "properties")]
    pub r#properties: Option<std::collections::HashMap<String, String>>,
    /// Address of the queue to reply to.
    #[builder(into)]
    #[serde(rename = "replyTo")]
    pub r#reply_to: Option<String>,
    /// Session identifier to reply to.
    #[builder(into)]
    #[serde(rename = "replyToSessionId")]
    pub r#reply_to_session_id: Option<String>,
    /// Session identifier.
    #[builder(into)]
    #[serde(rename = "sessionId")]
    pub r#session_id: Option<String>,
    /// Address to send to.
    #[builder(into)]
    #[serde(rename = "to")]
    pub r#to: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for SubscriptionRuleCorrelationFilter {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "content_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#content_type,
                )
                .await,
            );
            map.insert(
                "correlation_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#correlation_id,
                )
                .await,
            );
            map.insert(
                "label".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#label,
                )
                .await,
            );
            map.insert(
                "message_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#message_id,
                )
                .await,
            );
            map.insert(
                "properties".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#properties,
                )
                .await,
            );
            map.insert(
                "reply_to".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#reply_to,
                )
                .await,
            );
            map.insert(
                "reply_to_session_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#reply_to_session_id,
                )
                .await,
            );
            map.insert(
                "session_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#session_id,
                )
                .await,
            );
            map.insert(
                "to".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#to,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for SubscriptionRuleCorrelationFilter {
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
                    r#content_type: {
                        let field_value = match fields_map.get("content_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'content_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#correlation_id: {
                        let field_value = match fields_map.get("correlation_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'correlation_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#label: {
                        let field_value = match fields_map.get("label") {
                            Some(value) => value,
                            None => bail!("Missing field 'label' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#message_id: {
                        let field_value = match fields_map.get("message_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'message_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#properties: {
                        let field_value = match fields_map.get("properties") {
                            Some(value) => value,
                            None => bail!("Missing field 'properties' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#reply_to: {
                        let field_value = match fields_map.get("reply_to") {
                            Some(value) => value,
                            None => bail!("Missing field 'reply_to' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#reply_to_session_id: {
                        let field_value = match fields_map.get("reply_to_session_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'reply_to_session_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#session_id: {
                        let field_value = match fields_map.get("session_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'session_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#to: {
                        let field_value = match fields_map.get("to") {
                            Some(value) => value,
                            None => bail!("Missing field 'to' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
