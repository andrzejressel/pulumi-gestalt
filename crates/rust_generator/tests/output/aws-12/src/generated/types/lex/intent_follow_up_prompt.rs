#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct IntentFollowUpPrompt {
    /// Prompts for information from the user. Attributes are documented under prompt.
    #[builder(into)]
    #[serde(rename = "prompt")]
    pub r#prompt: Box<super::super::types::lex::IntentFollowUpPromptPrompt>,
    /// If the user answers "no" to the question defined in the prompt field,
    /// Amazon Lex responds with this statement to acknowledge that the intent was canceled. Attributes are
    /// documented below under statement.
    #[builder(into)]
    #[serde(rename = "rejectionStatement")]
    pub r#rejection_statement: Box<super::super::types::lex::IntentFollowUpPromptRejectionStatement>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for IntentFollowUpPrompt {
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
                "prompt".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#prompt,
                )
                .await,
            );
            map.insert(
                "rejection_statement".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#rejection_statement,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for IntentFollowUpPrompt {
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
                    r#prompt: {
                        let field_value = match fields_map.get("prompt") {
                            Some(value) => value,
                            None => bail!("Missing field 'prompt' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#rejection_statement: {
                        let field_value = match fields_map.get("rejection_statement") {
                            Some(value) => value,
                            None => bail!("Missing field 'rejection_statement' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
