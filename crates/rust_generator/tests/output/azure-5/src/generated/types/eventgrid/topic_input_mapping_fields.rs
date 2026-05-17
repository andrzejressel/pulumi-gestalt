#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct TopicInputMappingFields {
    /// Specifies the data version of the EventGrid Event to associate with the domain. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "dataVersion")]
    pub r#data_version: Option<String>,
    /// Specifies the event time of the EventGrid Event to associate with the domain. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "eventTime")]
    pub r#event_time: Option<String>,
    /// Specifies the event type of the EventGrid Event to associate with the domain. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "eventType")]
    pub r#event_type: Option<String>,
    /// Specifies the id of the EventGrid Event to associate with the domain. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Option<String>,
    /// Specifies the subject of the EventGrid Event to associate with the domain. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "subject")]
    pub r#subject: Option<String>,
    /// Specifies the topic of the EventGrid Event to associate with the domain. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "topic")]
    pub r#topic: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for TopicInputMappingFields {
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
                "data_version".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#data_version,
                )
                .await,
            );
            map.insert(
                "event_time".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#event_time,
                )
                .await,
            );
            map.insert(
                "event_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#event_type,
                )
                .await,
            );
            map.insert(
                "id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#id,
                )
                .await,
            );
            map.insert(
                "subject".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#subject,
                )
                .await,
            );
            map.insert(
                "topic".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#topic,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for TopicInputMappingFields {
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
                    r#data_version: {
                        let field_value = match fields_map.get("data_version") {
                            Some(value) => value,
                            None => bail!("Missing field 'data_version' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#event_time: {
                        let field_value = match fields_map.get("event_time") {
                            Some(value) => value,
                            None => bail!("Missing field 'event_time' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#event_type: {
                        let field_value = match fields_map.get("event_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'event_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#id: {
                        let field_value = match fields_map.get("id") {
                            Some(value) => value,
                            None => bail!("Missing field 'id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#subject: {
                        let field_value = match fields_map.get("subject") {
                            Some(value) => value,
                            None => bail!("Missing field 'subject' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#topic: {
                        let field_value = match fields_map.get("topic") {
                            Some(value) => value,
                            None => bail!("Missing field 'topic' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
