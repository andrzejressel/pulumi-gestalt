#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct SubscriptionRuleCorrelationFilter {
    /// Content type of the message.
    #[builder(into)]
    pub r#content_type: Option<String>,
    /// Identifier of the correlation.
    #[builder(into)]
    pub r#correlation_id: Option<String>,
    /// Application specific label.
    #[builder(into)]
    pub r#label: Option<String>,
    /// Identifier of the message.
    #[builder(into)]
    pub r#message_id: Option<String>,
    /// A list of user defined properties to be included in the filter. Specified as a map of name/value pairs.
    /// 
    /// > **NOTE:** When creating a subscription rule of type `CorrelationFilter` at least one property must be set in the `correlation_filter` block.
    #[builder(into)]
    pub r#properties: Option<std::collections::BTreeMap<String, String>>,
    /// Address of the queue to reply to.
    #[builder(into)]
    pub r#reply_to: Option<String>,
    /// Session identifier to reply to.
    #[builder(into)]
    pub r#reply_to_session_id: Option<String>,
    /// Session identifier.
    #[builder(into)]
    pub r#session_id: Option<String>,
    /// Address to send to.
    #[builder(into)]
    pub r#to: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for SubscriptionRuleCorrelationFilter {
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
                    "contentType",
                    &self.r#content_type,
                ),
                to_pulumi_object_field(
                    "correlationId",
                    &self.r#correlation_id,
                ),
                to_pulumi_object_field(
                    "label",
                    &self.r#label,
                ),
                to_pulumi_object_field(
                    "messageId",
                    &self.r#message_id,
                ),
                to_pulumi_object_field(
                    "properties",
                    &self.r#properties,
                ),
                to_pulumi_object_field(
                    "replyTo",
                    &self.r#reply_to,
                ),
                to_pulumi_object_field(
                    "replyToSessionId",
                    &self.r#reply_to_session_id,
                ),
                to_pulumi_object_field(
                    "sessionId",
                    &self.r#session_id,
                ),
                to_pulumi_object_field(
                    "to",
                    &self.r#to,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
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
                        let field_value = match fields_map.get("contentType") {
                            Some(value) => value,
                            None => bail!("Missing field 'contentType' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#correlation_id: {
                        let field_value = match fields_map.get("correlationId") {
                            Some(value) => value,
                            None => bail!("Missing field 'correlationId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("messageId") {
                            Some(value) => value,
                            None => bail!("Missing field 'messageId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("replyTo") {
                            Some(value) => value,
                            None => bail!("Missing field 'replyTo' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#reply_to_session_id: {
                        let field_value = match fields_map.get("replyToSessionId") {
                            Some(value) => value,
                            None => bail!("Missing field 'replyToSessionId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#session_id: {
                        let field_value = match fields_map.get("sessionId") {
                            Some(value) => value,
                            None => bail!("Missing field 'sessionId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
