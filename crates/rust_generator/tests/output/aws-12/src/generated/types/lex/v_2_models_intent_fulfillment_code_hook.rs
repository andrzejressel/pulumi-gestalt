#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct V2ModelsIntentFulfillmentCodeHook {
    /// Whether the fulfillment code hook is used. When active is false, the code hook doesn't run.
    #[builder(into)]
    #[serde(rename = "active")]
    pub r#active: Option<bool>,
    /// Whether a Lambda function should be invoked to fulfill a specific intent.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: bool,
    /// Configuration block for settings for update messages sent to the user for long-running Lambda fulfillment functions. Fulfillment updates can be used only with streaming conversations. See `fulfillment_updates_specification`.
    #[builder(into)]
    #[serde(rename = "fulfillmentUpdatesSpecification")]
    pub r#fulfillment_updates_specification: Option<Box<super::super::types::lex::V2ModelsIntentFulfillmentCodeHookFulfillmentUpdatesSpecification>>,
    /// Configuration block for settings for messages sent to the user for after the Lambda fulfillment function completes. Post-fulfillment messages can be sent for both streaming and non-streaming conversations. See `post_fulfillment_status_specification`.
    #[builder(into)]
    #[serde(rename = "postFulfillmentStatusSpecification")]
    pub r#post_fulfillment_status_specification: Option<Box<super::super::types::lex::V2ModelsIntentFulfillmentCodeHookPostFulfillmentStatusSpecification>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for V2ModelsIntentFulfillmentCodeHook {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("active".to_string(), self.r#active.to_pulumi_value().await);
            map.insert("enabled".to_string(), self.r#enabled.to_pulumi_value().await);
            map.insert("fulfillment_updates_specification".to_string(), self.r#fulfillment_updates_specification.to_pulumi_value().await);
            map.insert("post_fulfillment_status_specification".to_string(), self.r#post_fulfillment_status_specification.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for V2ModelsIntentFulfillmentCodeHook {
    fn from_pulumi_value(
        value: &pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    ) -> pulumi_gestalt_rust::__private::rootcause::Result<Self> {
        use std::collections::BTreeMap;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValueContent;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue;
        use pulumi_gestalt_rust::__private::rootcause::bail;

        match value.content {
            PulumiValueContent::Object(ref obj) => {
                let fields_map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> =
                    obj.iter().cloned().collect();

                Ok(Self {
                    r#active: {
                        let field_value = match fields_map.get("active") {
                            Some(value) => value,
                            None => bail!("Missing field 'active' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<bool> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#enabled: {
                        let field_value = match fields_map.get("enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <bool as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#fulfillment_updates_specification: {
                        let field_value = match fields_map.get("fulfillment_updates_specification") {
                            Some(value) => value,
                            None => bail!("Missing field 'fulfillment_updates_specification' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::lex::V2ModelsIntentFulfillmentCodeHookFulfillmentUpdatesSpecification>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#post_fulfillment_status_specification: {
                        let field_value = match fields_map.get("post_fulfillment_status_specification") {
                            Some(value) => value,
                            None => bail!("Missing field 'post_fulfillment_status_specification' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::lex::V2ModelsIntentFulfillmentCodeHookPostFulfillmentStatusSpecification>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
