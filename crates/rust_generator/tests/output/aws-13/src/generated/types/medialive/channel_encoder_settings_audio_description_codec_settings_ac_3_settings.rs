#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ChannelEncoderSettingsAudioDescriptionCodecSettingsAc3Settings {
    /// Average bitrate in bits/second.
    #[builder(into)]
    #[serde(rename = "bitrate")]
    pub r#bitrate: Option<f64>,
    /// Specifies the bitstream mode (bsmod) for the emitted AC-3 stream.
    #[builder(into)]
    #[serde(rename = "bitstreamMode")]
    pub r#bitstream_mode: Option<String>,
    /// Dolby Digital coding mode.
    #[builder(into)]
    #[serde(rename = "codingMode")]
    pub r#coding_mode: Option<String>,
    /// Sets the dialnorm of the output.
    #[builder(into)]
    #[serde(rename = "dialnorm")]
    pub r#dialnorm: Option<i32>,
    /// If set to filmStandard, adds dynamic range compression signaling to the output bitstream as defined in the Dolby Digital specification.
    #[builder(into)]
    #[serde(rename = "drcProfile")]
    pub r#drc_profile: Option<String>,
    /// When set to enabled, applies a 120Hz lowpass filter to the LFE channel prior to encoding.
    #[builder(into)]
    #[serde(rename = "lfeFilter")]
    pub r#lfe_filter: Option<String>,
    /// Metadata control.
    #[builder(into)]
    #[serde(rename = "metadataControl")]
    pub r#metadata_control: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ChannelEncoderSettingsAudioDescriptionCodecSettingsAc3Settings {
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
                    "bitrate",
                    &self.r#bitrate,
                ),
                to_pulumi_object_field(
                    "bitstream_mode",
                    &self.r#bitstream_mode,
                ),
                to_pulumi_object_field(
                    "coding_mode",
                    &self.r#coding_mode,
                ),
                to_pulumi_object_field(
                    "dialnorm",
                    &self.r#dialnorm,
                ),
                to_pulumi_object_field(
                    "drc_profile",
                    &self.r#drc_profile,
                ),
                to_pulumi_object_field(
                    "lfe_filter",
                    &self.r#lfe_filter,
                ),
                to_pulumi_object_field(
                    "metadata_control",
                    &self.r#metadata_control,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ChannelEncoderSettingsAudioDescriptionCodecSettingsAc3Settings {
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
                    r#bitrate: {
                        let field_value = match fields_map.get("bitrate") {
                            Some(value) => value,
                            None => bail!("Missing field 'bitrate' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#bitstream_mode: {
                        let field_value = match fields_map.get("bitstream_mode") {
                            Some(value) => value,
                            None => bail!("Missing field 'bitstream_mode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#coding_mode: {
                        let field_value = match fields_map.get("coding_mode") {
                            Some(value) => value,
                            None => bail!("Missing field 'coding_mode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#dialnorm: {
                        let field_value = match fields_map.get("dialnorm") {
                            Some(value) => value,
                            None => bail!("Missing field 'dialnorm' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#drc_profile: {
                        let field_value = match fields_map.get("drc_profile") {
                            Some(value) => value,
                            None => bail!("Missing field 'drc_profile' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#lfe_filter: {
                        let field_value = match fields_map.get("lfe_filter") {
                            Some(value) => value,
                            None => bail!("Missing field 'lfe_filter' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#metadata_control: {
                        let field_value = match fields_map.get("metadata_control") {
                            Some(value) => value,
                            None => bail!("Missing field 'metadata_control' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
