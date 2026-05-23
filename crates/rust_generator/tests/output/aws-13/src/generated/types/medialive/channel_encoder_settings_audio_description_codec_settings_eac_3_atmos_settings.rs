#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ChannelEncoderSettingsAudioDescriptionCodecSettingsEac3AtmosSettings {
    /// Average bitrate in bits/second.
    #[builder(into)]
    #[serde(rename = "bitrate")]
    pub r#bitrate: Option<f64>,
    /// Dolby Digital Plus with Dolby Atmos coding mode.
    #[builder(into)]
    #[serde(rename = "codingMode")]
    pub r#coding_mode: Option<String>,
    /// Sets the dialnorm for the output.
    #[builder(into)]
    #[serde(rename = "dialnorm")]
    pub r#dialnorm: Option<f64>,
    /// Sets the Dolby dynamic range compression profile.
    #[builder(into)]
    #[serde(rename = "drcLine")]
    pub r#drc_line: Option<String>,
    /// Sets the profile for heavy Dolby dynamic range compression.
    #[builder(into)]
    #[serde(rename = "drcRf")]
    pub r#drc_rf: Option<String>,
    /// Height dimensional trim.
    #[builder(into)]
    #[serde(rename = "heightTrim")]
    pub r#height_trim: Option<f64>,
    /// Surround dimensional trim.
    #[builder(into)]
    #[serde(rename = "surroundTrim")]
    pub r#surround_trim: Option<f64>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ChannelEncoderSettingsAudioDescriptionCodecSettingsEac3AtmosSettings {
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
                    "codingMode",
                    &self.r#coding_mode,
                ),
                to_pulumi_object_field(
                    "dialnorm",
                    &self.r#dialnorm,
                ),
                to_pulumi_object_field(
                    "drcLine",
                    &self.r#drc_line,
                ),
                to_pulumi_object_field(
                    "drcRf",
                    &self.r#drc_rf,
                ),
                to_pulumi_object_field(
                    "heightTrim",
                    &self.r#height_trim,
                ),
                to_pulumi_object_field(
                    "surroundTrim",
                    &self.r#surround_trim,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ChannelEncoderSettingsAudioDescriptionCodecSettingsEac3AtmosSettings {
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
                    r#coding_mode: {
                        let field_value = match fields_map.get("codingMode") {
                            Some(value) => value,
                            None => bail!("Missing field 'codingMode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#drc_line: {
                        let field_value = match fields_map.get("drcLine") {
                            Some(value) => value,
                            None => bail!("Missing field 'drcLine' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#drc_rf: {
                        let field_value = match fields_map.get("drcRf") {
                            Some(value) => value,
                            None => bail!("Missing field 'drcRf' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#height_trim: {
                        let field_value = match fields_map.get("heightTrim") {
                            Some(value) => value,
                            None => bail!("Missing field 'heightTrim' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#surround_trim: {
                        let field_value = match fields_map.get("surroundTrim") {
                            Some(value) => value,
                            None => bail!("Missing field 'surroundTrim' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
