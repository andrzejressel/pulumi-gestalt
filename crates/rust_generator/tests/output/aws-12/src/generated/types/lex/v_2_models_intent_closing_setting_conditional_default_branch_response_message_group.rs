#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct V2ModelsIntentClosingSettingConditionalDefaultBranchResponseMessageGroup {
    /// Configuration block for the primary message that Amazon Lex should send to the user. See `message`.
    #[builder(into)]
    #[serde(rename = "message")]
    pub r#message: Box<super::super::types::lex::V2ModelsIntentClosingSettingConditionalDefaultBranchResponseMessageGroupMessage>,
    /// Configuration blocks for message variations to send to the user. When variations are defined, Amazon Lex chooses the primary message or one of the variations to send to the user. See `variation`.
    #[builder(into)]
    #[serde(rename = "variations")]
    pub r#variations: Option<Vec<super::super::types::lex::V2ModelsIntentClosingSettingConditionalDefaultBranchResponseMessageGroupVariation>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for V2ModelsIntentClosingSettingConditionalDefaultBranchResponseMessageGroup {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("message".to_string(), self.r#message.to_pulumi_value().await);
            map.insert("variations".to_string(), self.r#variations.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for V2ModelsIntentClosingSettingConditionalDefaultBranchResponseMessageGroup {
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
                    r#message: {
                        let field_value = match fields_map.get("message") {
                            Some(value) => value,
                            None => bail!("Missing field 'message' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Box<super::super::types::lex::V2ModelsIntentClosingSettingConditionalDefaultBranchResponseMessageGroupMessage> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#variations: {
                        let field_value = match fields_map.get("variations") {
                            Some(value) => value,
                            None => bail!("Missing field 'variations' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<super::super::types::lex::V2ModelsIntentClosingSettingConditionalDefaultBranchResponseMessageGroupVariation>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
