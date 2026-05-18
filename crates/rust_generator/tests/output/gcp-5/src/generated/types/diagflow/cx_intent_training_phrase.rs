#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct CxIntentTrainingPhrase {
    /// (Output)
    /// The unique identifier of the training phrase.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Option<String>,
    /// The ordered list of training phrase parts. The parts are concatenated in order to form the training phrase.
    /// Note: The API does not automatically annotate training phrases like the Dialogflow Console does.
    /// Note: Do not forget to include whitespace at part boundaries, so the training phrase is well formatted when the parts are concatenated.
    /// If the training phrase does not need to be annotated with parameters, you just need a single part with only the Part.text field set.
    /// If you want to annotate the training phrase, you must create multiple parts, where the fields of each part are populated in one of two ways:
    /// Part.text is set to a part of the phrase that has no parameters.
    /// Part.text is set to a part of the phrase that you want to annotate, and the parameterId field is set.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "parts")]
    pub r#parts: Vec<super::super::types::diagflow::CxIntentTrainingPhrasePart>,
    /// Indicates how many times this example was added to the intent.
    #[builder(into)]
    #[serde(rename = "repeatCount")]
    pub r#repeat_count: Option<i32>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for CxIntentTrainingPhrase {
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
                    "id",
                    &self.r#id,
                ),
                to_pulumi_object_field(
                    "parts",
                    &self.r#parts,
                ),
                to_pulumi_object_field(
                    "repeat_count",
                    &self.r#repeat_count,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for CxIntentTrainingPhrase {
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
                    r#id: {
                        let field_value = match fields_map.get("id") {
                            Some(value) => value,
                            None => bail!("Missing field 'id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#parts: {
                        let field_value = match fields_map.get("parts") {
                            Some(value) => value,
                            None => bail!("Missing field 'parts' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#repeat_count: {
                        let field_value = match fields_map.get("repeat_count") {
                            Some(value) => value,
                            None => bail!("Missing field 'repeat_count' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
