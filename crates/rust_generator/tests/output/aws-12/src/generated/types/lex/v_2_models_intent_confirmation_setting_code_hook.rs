#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct V2ModelsIntentConfirmationSettingCodeHook {
    /// Whether a dialog code hook is used when the intent is activated.
    #[builder(into)]
    #[serde(rename = "active")]
    pub r#active: bool,
    /// Whether a Lambda function should be invoked for the dialog.
    #[builder(into)]
    #[serde(rename = "enableCodeHookInvocation")]
    pub r#enable_code_hook_invocation: bool,
    /// Label that indicates the dialog step from which the dialog code hook is happening.
    #[builder(into)]
    #[serde(rename = "invocationLabel")]
    pub r#invocation_label: Option<String>,
    /// Configuration block that contains the responses and actions that Amazon Lex takes after the Lambda function is complete. See `post_code_hook_specification`.
    #[builder(into)]
    #[serde(rename = "postCodeHookSpecification")]
    pub r#post_code_hook_specification: Box<super::super::types::lex::V2ModelsIntentConfirmationSettingCodeHookPostCodeHookSpecification>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for V2ModelsIntentConfirmationSettingCodeHook {
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
            map.insert("enable_code_hook_invocation".to_string(), self.r#enable_code_hook_invocation.to_pulumi_value().await);
            map.insert("invocation_label".to_string(), self.r#invocation_label.to_pulumi_value().await);
            map.insert("post_code_hook_specification".to_string(), self.r#post_code_hook_specification.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for V2ModelsIntentConfirmationSettingCodeHook {
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
                    r#enable_code_hook_invocation: {
                        let field_value = match fields_map.get("enable_code_hook_invocation") {
                            Some(value) => value,
                            None => bail!("Missing field 'enable_code_hook_invocation' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <bool as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#invocation_label: {
                        let field_value = match fields_map.get("invocation_label") {
                            Some(value) => value,
                            None => bail!("Missing field 'invocation_label' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#post_code_hook_specification: {
                        let field_value = match fields_map.get("post_code_hook_specification") {
                            Some(value) => value,
                            None => bail!("Missing field 'post_code_hook_specification' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Box<super::super::types::lex::V2ModelsIntentConfirmationSettingCodeHookPostCodeHookSpecification> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
