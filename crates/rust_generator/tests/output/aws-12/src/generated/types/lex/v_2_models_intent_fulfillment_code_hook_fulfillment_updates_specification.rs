#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct V2ModelsIntentFulfillmentCodeHookFulfillmentUpdatesSpecification {
    /// Whether fulfillment updates are sent to the user. When this field is true, updates are sent. If the active field is set to true, the `start_response`, `update_response`, and `timeout_in_seconds` fields are required.
    #[builder(into)]
    #[serde(rename = "active")]
    pub r#active: bool,
    /// Configuration block for the message sent to users when the fulfillment Lambda functions starts running.
    #[builder(into)]
    #[serde(rename = "startResponse")]
    pub r#start_response: Option<Box<super::super::types::lex::V2ModelsIntentFulfillmentCodeHookFulfillmentUpdatesSpecificationStartResponse>>,
    /// Length of time that the fulfillment Lambda function should run before it times out.
    #[builder(into)]
    #[serde(rename = "timeoutInSeconds")]
    pub r#timeout_in_seconds: Option<i32>,
    /// Configuration block for messages sent periodically to the user while the fulfillment Lambda function is running.
    #[builder(into)]
    #[serde(rename = "updateResponse")]
    pub r#update_response: Option<Box<super::super::types::lex::V2ModelsIntentFulfillmentCodeHookFulfillmentUpdatesSpecificationUpdateResponse>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for V2ModelsIntentFulfillmentCodeHookFulfillmentUpdatesSpecification {
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
            map.insert("start_response".to_string(), self.r#start_response.to_pulumi_value().await);
            map.insert("timeout_in_seconds".to_string(), self.r#timeout_in_seconds.to_pulumi_value().await);
            map.insert("update_response".to_string(), self.r#update_response.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for V2ModelsIntentFulfillmentCodeHookFulfillmentUpdatesSpecification {
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
                        <bool as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#start_response: {
                        let field_value = match fields_map.get("start_response") {
                            Some(value) => value,
                            None => bail!("Missing field 'start_response' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::lex::V2ModelsIntentFulfillmentCodeHookFulfillmentUpdatesSpecificationStartResponse>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#timeout_in_seconds: {
                        let field_value = match fields_map.get("timeout_in_seconds") {
                            Some(value) => value,
                            None => bail!("Missing field 'timeout_in_seconds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<i32> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#update_response: {
                        let field_value = match fields_map.get("update_response") {
                            Some(value) => value,
                            None => bail!("Missing field 'update_response' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::lex::V2ModelsIntentFulfillmentCodeHookFulfillmentUpdatesSpecificationUpdateResponse>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
