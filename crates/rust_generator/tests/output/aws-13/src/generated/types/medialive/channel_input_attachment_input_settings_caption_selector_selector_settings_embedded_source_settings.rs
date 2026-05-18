#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ChannelInputAttachmentInputSettingsCaptionSelectorSelectorSettingsEmbeddedSourceSettings {
    /// If upconvert, 608 data is both passed through via the “608 compatibility bytes” fields of the 708 wrapper as well as translated into 708. 708 data present in the source content will be discarded.
    #[builder(into)]
    #[serde(rename = "convert608To708")]
    pub r#convert_608_to_708: Option<String>,
    /// Set to “auto” to handle streams with intermittent and/or non-aligned SCTE-20 and Embedded captions.
    #[builder(into)]
    #[serde(rename = "scte20Detection")]
    pub r#scte_20_detection: Option<String>,
    /// Specifies the 608/708 channel number within the video track from which to extract captions. Unused for passthrough.
    #[builder(into)]
    #[serde(rename = "source608ChannelNumber")]
    pub r#source_608_channel_number: Option<i32>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ChannelInputAttachmentInputSettingsCaptionSelectorSelectorSettingsEmbeddedSourceSettings {
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
                    "convert_608_to_708",
                    &self.r#convert_608_to_708,
                ),
                to_pulumi_object_field(
                    "scte_20_detection",
                    &self.r#scte_20_detection,
                ),
                to_pulumi_object_field(
                    "source_608_channel_number",
                    &self.r#source_608_channel_number,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ChannelInputAttachmentInputSettingsCaptionSelectorSelectorSettingsEmbeddedSourceSettings {
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
                    r#convert_608_to_708: {
                        let field_value = match fields_map.get("convert_608_to_708") {
                            Some(value) => value,
                            None => bail!("Missing field 'convert_608_to_708' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#scte_20_detection: {
                        let field_value = match fields_map.get("scte_20_detection") {
                            Some(value) => value,
                            None => bail!("Missing field 'scte_20_detection' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#source_608_channel_number: {
                        let field_value = match fields_map.get("source_608_channel_number") {
                            Some(value) => value,
                            None => bail!("Missing field 'source_608_channel_number' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
