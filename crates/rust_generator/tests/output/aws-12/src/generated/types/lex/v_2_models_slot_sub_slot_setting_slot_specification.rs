#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct V2ModelsSlotSubSlotSettingSlotSpecification {
    #[builder(into)]
    #[serde(rename = "mapBlockKey")]
    pub r#map_block_key: String,
    /// Unique identifier assigned to the slot type.
    #[builder(into)]
    #[serde(rename = "slotTypeId")]
    pub r#slot_type_id: String,
    /// Elicitation setting details for constituent sub slots of a composite slot.
    /// See the `value_elicitation_setting` argument reference below.
    #[builder(into)]
    #[serde(rename = "valueElicitationSettings")]
    pub r#value_elicitation_settings: Option<Vec<super::super::types::lex::V2ModelsSlotSubSlotSettingSlotSpecificationValueElicitationSetting>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for V2ModelsSlotSubSlotSettingSlotSpecification {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("map_block_key".to_string(), self.r#map_block_key.to_pulumi_value().await);
            map.insert("slot_type_id".to_string(), self.r#slot_type_id.to_pulumi_value().await);
            map.insert("value_elicitation_settings".to_string(), self.r#value_elicitation_settings.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for V2ModelsSlotSubSlotSettingSlotSpecification {
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
                    r#map_block_key: {
                        let field_value = match fields_map.get("map_block_key") {
                            Some(value) => value,
                            None => bail!("Missing field 'map_block_key' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#slot_type_id: {
                        let field_value = match fields_map.get("slot_type_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'slot_type_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#value_elicitation_settings: {
                        let field_value = match fields_map.get("value_elicitation_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'value_elicitation_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<super::super::types::lex::V2ModelsSlotSubSlotSettingSlotSpecificationValueElicitationSetting>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
