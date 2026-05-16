#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct V2ModelsSlotTypeSlotTypeValues {
    /// Value of the slot type entry.
    /// See `sample_value` argument reference below.
    #[builder(into)]
    #[serde(rename = "sampleValues")]
    pub r#sample_values: Option<Vec<super::super::types::lex::V2ModelsSlotTypeSlotTypeValuesSampleValue>>,
    /// A list of additional values related to the slot type entry.
    /// See `synonyms` argument reference below.
    #[builder(into)]
    #[serde(rename = "synonyms")]
    pub r#synonyms: Option<Vec<super::super::types::lex::V2ModelsSlotTypeSlotTypeValuesSynonym>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for V2ModelsSlotTypeSlotTypeValues {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("sample_values".to_string(), self.r#sample_values.to_pulumi_value().await);
            map.insert("synonyms".to_string(), self.r#synonyms.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for V2ModelsSlotTypeSlotTypeValues {
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
                    r#sample_values: {
                        let field_value = match fields_map.get("sample_values") {
                            Some(value) => value,
                            None => bail!("Missing field 'sample_values' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<super::super::types::lex::V2ModelsSlotTypeSlotTypeValuesSampleValue>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#synonyms: {
                        let field_value = match fields_map.get("synonyms") {
                            Some(value) => value,
                            None => bail!("Missing field 'synonyms' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<super::super::types::lex::V2ModelsSlotTypeSlotTypeValuesSynonym>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
