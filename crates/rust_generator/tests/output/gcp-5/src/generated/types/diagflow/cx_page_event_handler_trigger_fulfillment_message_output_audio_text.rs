#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct CxPageEventHandlerTriggerFulfillmentMessageOutputAudioText {
    /// (Output)
    /// Whether the playback of this message can be interrupted by the end user's speech and the client can then starts the next Dialogflow request.
    #[builder(into)]
    #[serde(rename = "allowPlaybackInterruption")]
    pub r#allow_playback_interruption: Option<bool>,
    /// The SSML text to be synthesized. For more information, see SSML.
    #[builder(into)]
    #[serde(rename = "ssml")]
    pub r#ssml: Option<String>,
    /// The raw text to be synthesized.
    #[builder(into)]
    #[serde(rename = "text")]
    pub r#text: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for CxPageEventHandlerTriggerFulfillmentMessageOutputAudioText {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_concurrent, to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "allow_playback_interruption",
                    &self.r#allow_playback_interruption,
                ),
                to_pulumi_object_field(
                    "ssml",
                    &self.r#ssml,
                ),
                to_pulumi_object_field(
                    "text",
                    &self.r#text,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for CxPageEventHandlerTriggerFulfillmentMessageOutputAudioText {
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
                    r#allow_playback_interruption: {
                        let field_value = match fields_map.get("allow_playback_interruption") {
                            Some(value) => value,
                            None => bail!("Missing field 'allow_playback_interruption' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ssml: {
                        let field_value = match fields_map.get("ssml") {
                            Some(value) => value,
                            None => bail!("Missing field 'ssml' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#text: {
                        let field_value = match fields_map.get("text") {
                            Some(value) => value,
                            None => bail!("Missing field 'text' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
