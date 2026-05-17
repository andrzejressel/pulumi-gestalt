#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct HubEventHandler {
    /// An `auth` block as defined below.
    #[builder(into)]
    #[serde(rename = "auth")]
    pub r#auth: Option<Box<super::super::types::webpubsub::HubEventHandlerAuth>>,
    /// Specifies the list of system events. Supported values are `connect`, `connected` and `disconnected`.
    #[builder(into)]
    #[serde(rename = "systemEvents")]
    pub r#system_events: Option<Vec<String>>,
    /// The Event Handler URL Template. Two predefined parameters `{hub}` and `{event}` are available to use in the template. The value of the EventHandler URL is dynamically calculated when the client request comes in. Example: `http://example.com/api/{hub}/{event}`.
    #[builder(into)]
    #[serde(rename = "urlTemplate")]
    pub r#url_template: String,
    /// Specifies the matching event names. There are 3 kind of patterns supported: * `*` matches any event name * `,` Combine multiple events with `,` for example `event1,event2`, it matches event `event1` and `event2` * The single event name, for example `event1`, it matches `event1`.
    #[builder(into)]
    #[serde(rename = "userEventPattern")]
    pub r#user_event_pattern: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for HubEventHandler {
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
                "auth".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#auth,
                )
                .await,
            );
            map.insert(
                "system_events".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#system_events,
                )
                .await,
            );
            map.insert(
                "url_template".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#url_template,
                )
                .await,
            );
            map.insert(
                "user_event_pattern".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#user_event_pattern,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for HubEventHandler {
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
                    r#auth: {
                        let field_value = match fields_map.get("auth") {
                            Some(value) => value,
                            None => bail!("Missing field 'auth' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#system_events: {
                        let field_value = match fields_map.get("system_events") {
                            Some(value) => value,
                            None => bail!("Missing field 'system_events' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#url_template: {
                        let field_value = match fields_map.get("url_template") {
                            Some(value) => value,
                            None => bail!("Missing field 'url_template' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#user_event_pattern: {
                        let field_value = match fields_map.get("user_event_pattern") {
                            Some(value) => value,
                            None => bail!("Missing field 'user_event_pattern' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
