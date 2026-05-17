#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct CxPageEventHandler {
    /// The name of the event to handle.
    #[builder(into)]
    #[serde(rename = "event")]
    pub r#event: Option<String>,
    /// (Output)
    /// The unique identifier of this event handler.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// The target flow to transition to.
    /// Format: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/flows/<Flow ID>.
    #[builder(into)]
    #[serde(rename = "targetFlow")]
    pub r#target_flow: Option<String>,
    /// The target page to transition to.
    /// Format: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/flows/<Flow ID>/pages/<Page ID>.
    #[builder(into)]
    #[serde(rename = "targetPage")]
    pub r#target_page: Option<String>,
    /// The fulfillment to call when the event occurs. Handling webhook errors with a fulfillment enabled with webhook could cause infinite loop. It is invalid to specify such fulfillment for a handler handling webhooks.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "triggerFulfillment")]
    pub r#trigger_fulfillment: Option<Box<super::super::types::diagflow::CxPageEventHandlerTriggerFulfillment>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for CxPageEventHandler {
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
                "event".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#event,
                )
                .await,
            );
            map.insert(
                "name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#name,
                )
                .await,
            );
            map.insert(
                "target_flow".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#target_flow,
                )
                .await,
            );
            map.insert(
                "target_page".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#target_page,
                )
                .await,
            );
            map.insert(
                "trigger_fulfillment".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#trigger_fulfillment,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for CxPageEventHandler {
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
                    r#event: {
                        let field_value = match fields_map.get("event") {
                            Some(value) => value,
                            None => bail!("Missing field 'event' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#name: {
                        let field_value = match fields_map.get("name") {
                            Some(value) => value,
                            None => bail!("Missing field 'name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#target_flow: {
                        let field_value = match fields_map.get("target_flow") {
                            Some(value) => value,
                            None => bail!("Missing field 'target_flow' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#target_page: {
                        let field_value = match fields_map.get("target_page") {
                            Some(value) => value,
                            None => bail!("Missing field 'target_page' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#trigger_fulfillment: {
                        let field_value = match fields_map.get("trigger_fulfillment") {
                            Some(value) => value,
                            None => bail!("Missing field 'trigger_fulfillment' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
