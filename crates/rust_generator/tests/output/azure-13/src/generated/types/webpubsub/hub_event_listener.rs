#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct HubEventListener {
    /// Specifies the event hub name to receive the events.
    #[builder(into)]
    #[serde(rename = "eventhubName")]
    pub r#eventhub_name: String,
    /// Specifies the event hub namespace name to receive the events.
    #[builder(into)]
    #[serde(rename = "eventhubNamespaceName")]
    pub r#eventhub_namespace_name: String,
    /// Specifies the list of system events. Supported values are `connected` and `disconnected`.
    #[builder(into)]
    #[serde(rename = "systemEventNameFilters")]
    pub r#system_event_name_filters: Option<Vec<String>>,
    /// Specifies the list of matching user event names. `["*"]` can be used to match all events.
    #[builder(into)]
    #[serde(rename = "userEventNameFilters")]
    pub r#user_event_name_filters: Option<Vec<String>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for HubEventListener {
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
                "eventhub_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#eventhub_name,
                )
                .await,
            );
            map.insert(
                "eventhub_namespace_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#eventhub_namespace_name,
                )
                .await,
            );
            map.insert(
                "system_event_name_filters".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#system_event_name_filters,
                )
                .await,
            );
            map.insert(
                "user_event_name_filters".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#user_event_name_filters,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for HubEventListener {
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
                    r#eventhub_name: {
                        let field_value = match fields_map.get("eventhub_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'eventhub_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#eventhub_namespace_name: {
                        let field_value = match fields_map.get("eventhub_namespace_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'eventhub_namespace_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#system_event_name_filters: {
                        let field_value = match fields_map.get("system_event_name_filters") {
                            Some(value) => value,
                            None => bail!("Missing field 'system_event_name_filters' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#user_event_name_filters: {
                        let field_value = match fields_map.get("user_event_name_filters") {
                            Some(value) => value,
                            None => bail!("Missing field 'user_event_name_filters' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
