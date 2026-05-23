#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct V2ModelsIntentConfirmationSettingCodeHook {
    /// Whether a dialog code hook is used when the intent is activated.
    #[builder(into)]
    pub r#active: bool,
    /// Whether a Lambda function should be invoked for the dialog.
    #[builder(into)]
    pub r#enable_code_hook_invocation: bool,
    /// Label that indicates the dialog step from which the dialog code hook is happening.
    #[builder(into)]
    pub r#invocation_label: Option<String>,
    /// Configuration block that contains the responses and actions that Amazon Lex takes after the Lambda function is complete. See `post_code_hook_specification`.
    #[builder(into)]
    pub r#post_code_hook_specification: Box<super::super::types::lex::V2ModelsIntentConfirmationSettingCodeHookPostCodeHookSpecification>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for V2ModelsIntentConfirmationSettingCodeHook {
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
                    "active",
                    &self.r#active,
                ),
                to_pulumi_object_field(
                    "enableCodeHookInvocation",
                    &self.r#enable_code_hook_invocation,
                ),
                to_pulumi_object_field(
                    "invocationLabel",
                    &self.r#invocation_label,
                ),
                to_pulumi_object_field(
                    "postCodeHookSpecification",
                    &self.r#post_code_hook_specification,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for V2ModelsIntentConfirmationSettingCodeHook {
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
                    r#active: {
                        let field_value = match fields_map.get("active") {
                            Some(value) => value,
                            None => bail!("Missing field 'active' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#enable_code_hook_invocation: {
                        let field_value = match fields_map.get("enableCodeHookInvocation") {
                            Some(value) => value,
                            None => bail!("Missing field 'enableCodeHookInvocation' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#invocation_label: {
                        let field_value = match fields_map.get("invocationLabel") {
                            Some(value) => value,
                            None => bail!("Missing field 'invocationLabel' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#post_code_hook_specification: {
                        let field_value = match fields_map.get("postCodeHookSpecification") {
                            Some(value) => value,
                            None => bail!("Missing field 'postCodeHookSpecification' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
