#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct V2ModelsIntentConfirmationSettingCodeHookPostCodeHookSpecificationTimeoutNextStep {
    /// Configuration block for action that the bot executes at runtime when the conversation reaches this step. See `dialog_action`.
    #[builder(into)]
    #[serde(rename = "dialogAction")]
    pub r#dialog_action: Option<Box<super::super::types::lex::V2ModelsIntentConfirmationSettingCodeHookPostCodeHookSpecificationTimeoutNextStepDialogAction>>,
    /// Configuration block for override settings to configure the intent state. See `intent`.
    #[builder(into)]
    #[serde(rename = "intent")]
    pub r#intent: Option<Box<super::super::types::lex::V2ModelsIntentConfirmationSettingCodeHookPostCodeHookSpecificationTimeoutNextStepIntent>>,
    /// Map of key/value pairs representing session-specific context information. It contains application information passed between Amazon Lex and a client application.
    #[builder(into)]
    #[serde(rename = "sessionAttributes")]
    pub r#session_attributes: Option<std::collections::HashMap<String, String>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for V2ModelsIntentConfirmationSettingCodeHookPostCodeHookSpecificationTimeoutNextStep {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("dialog_action".to_string(), self.r#dialog_action.to_pulumi_value().await);
            map.insert("intent".to_string(), self.r#intent.to_pulumi_value().await);
            map.insert("session_attributes".to_string(), self.r#session_attributes.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for V2ModelsIntentConfirmationSettingCodeHookPostCodeHookSpecificationTimeoutNextStep {
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
                    r#dialog_action: {
                        let field_value = match fields_map.get("dialog_action") {
                            Some(value) => value,
                            None => bail!("Missing field 'dialog_action' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::lex::V2ModelsIntentConfirmationSettingCodeHookPostCodeHookSpecificationTimeoutNextStepDialogAction>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#intent: {
                        let field_value = match fields_map.get("intent") {
                            Some(value) => value,
                            None => bail!("Missing field 'intent' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::lex::V2ModelsIntentConfirmationSettingCodeHookPostCodeHookSpecificationTimeoutNextStepIntent>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#session_attributes: {
                        let field_value = match fields_map.get("session_attributes") {
                            Some(value) => value,
                            None => bail!("Missing field 'session_attributes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<std::collections::HashMap<String, String>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
