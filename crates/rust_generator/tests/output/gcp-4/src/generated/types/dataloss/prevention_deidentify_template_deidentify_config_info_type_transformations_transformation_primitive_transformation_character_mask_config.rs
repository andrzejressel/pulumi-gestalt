#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PreventionDeidentifyTemplateDeidentifyConfigInfoTypeTransformationsTransformationPrimitiveTransformationCharacterMaskConfig {
    /// Characters to skip when doing de-identification of a value. These will be left alone and skipped.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "charactersToIgnores")]
    pub r#characters_to_ignores: Option<Vec<super::super::types::dataloss::PreventionDeidentifyTemplateDeidentifyConfigInfoTypeTransformationsTransformationPrimitiveTransformationCharacterMaskConfigCharactersToIgnore>>,
    /// Character to use to mask the sensitive values—for example, * for an alphabetic string such as a name, or 0 for a numeric string
    /// such as ZIP code or credit card number. This string must have a length of 1. If not supplied, this value defaults to * for
    /// strings, and 0 for digits.
    #[builder(into)]
    #[serde(rename = "maskingCharacter")]
    pub r#masking_character: Option<String>,
    /// Number of characters to mask. If not set, all matching chars will be masked. Skipped characters do not count towards this tally.
    /// If number_to_mask is negative, this denotes inverse masking. Cloud DLP masks all but a number of characters. For example, suppose you have the following values:
    #[builder(into)]
    #[serde(rename = "numberToMask")]
    pub r#number_to_mask: Option<i32>,
    /// Mask characters in reverse order. For example, if masking_character is 0, number_to_mask is 14, and reverse_order is `false`, then the
    /// input string `1234-5678-9012-3456` is masked as `00000000000000-3456`.
    #[builder(into)]
    #[serde(rename = "reverseOrder")]
    pub r#reverse_order: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for PreventionDeidentifyTemplateDeidentifyConfigInfoTypeTransformationsTransformationPrimitiveTransformationCharacterMaskConfig {
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
                    "characters_to_ignores",
                    &self.r#characters_to_ignores,
                ),
                to_pulumi_object_field(
                    "masking_character",
                    &self.r#masking_character,
                ),
                to_pulumi_object_field(
                    "number_to_mask",
                    &self.r#number_to_mask,
                ),
                to_pulumi_object_field(
                    "reverse_order",
                    &self.r#reverse_order,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for PreventionDeidentifyTemplateDeidentifyConfigInfoTypeTransformationsTransformationPrimitiveTransformationCharacterMaskConfig {
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
                    r#characters_to_ignores: {
                        let field_value = match fields_map.get("characters_to_ignores") {
                            Some(value) => value,
                            None => bail!("Missing field 'characters_to_ignores' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#masking_character: {
                        let field_value = match fields_map.get("masking_character") {
                            Some(value) => value,
                            None => bail!("Missing field 'masking_character' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#number_to_mask: {
                        let field_value = match fields_map.get("number_to_mask") {
                            Some(value) => value,
                            None => bail!("Missing field 'number_to_mask' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#reverse_order: {
                        let field_value = match fields_map.get("reverse_order") {
                            Some(value) => value,
                            None => bail!("Missing field 'reverse_order' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
