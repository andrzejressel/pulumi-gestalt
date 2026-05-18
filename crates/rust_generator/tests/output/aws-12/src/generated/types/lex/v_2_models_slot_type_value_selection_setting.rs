#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct V2ModelsSlotTypeValueSelectionSetting {
    /// Provides settings that enable advanced recognition settings for slot values.
    /// You can use this to enable using slot values as a custom vocabulary for recognizing user utterances.
    /// See `advanced_recognition_setting` argument reference below.
    #[builder(into)]
    #[serde(rename = "advancedRecognitionSettings")]
    pub r#advanced_recognition_settings: Option<Vec<super::super::types::lex::V2ModelsSlotTypeValueSelectionSettingAdvancedRecognitionSetting>>,
    /// Used to validate the value of the slot.
    /// See `regex_filter` argument reference below.
    #[builder(into)]
    #[serde(rename = "regexFilters")]
    pub r#regex_filters: Option<Vec<super::super::types::lex::V2ModelsSlotTypeValueSelectionSettingRegexFilter>>,
    /// Determines the slot resolution strategy that Amazon Lex uses to return slot type values.
    /// Valid values are `OriginalValue`, `TopResolution`, and `Concatenation`.
    #[builder(into)]
    #[serde(rename = "resolutionStrategy")]
    pub r#resolution_strategy: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for V2ModelsSlotTypeValueSelectionSetting {
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
                    "advanced_recognition_settings",
                    &self.r#advanced_recognition_settings,
                ),
                to_pulumi_object_field(
                    "regex_filters",
                    &self.r#regex_filters,
                ),
                to_pulumi_object_field(
                    "resolution_strategy",
                    &self.r#resolution_strategy,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for V2ModelsSlotTypeValueSelectionSetting {
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
                    r#advanced_recognition_settings: {
                        let field_value = match fields_map.get("advanced_recognition_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'advanced_recognition_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#regex_filters: {
                        let field_value = match fields_map.get("regex_filters") {
                            Some(value) => value,
                            None => bail!("Missing field 'regex_filters' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#resolution_strategy: {
                        let field_value = match fields_map.get("resolution_strategy") {
                            Some(value) => value,
                            None => bail!("Missing field 'resolution_strategy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
