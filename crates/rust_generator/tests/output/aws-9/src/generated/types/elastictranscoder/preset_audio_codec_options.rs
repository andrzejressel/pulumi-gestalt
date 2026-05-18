#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PresetAudioCodecOptions {
    /// The bit depth of a sample is how many bits of information are included in the audio samples. Valid values are `16` and `24`. (FLAC/PCM Only)
    #[builder(into)]
    #[serde(rename = "bitDepth")]
    pub r#bit_depth: Option<String>,
    /// The order the bits of a PCM sample are stored in. The supported value is LittleEndian. (PCM Only)
    #[builder(into)]
    #[serde(rename = "bitOrder")]
    pub r#bit_order: Option<String>,
    /// If you specified AAC for Audio:Codec, choose the AAC profile for the output file.
    #[builder(into)]
    #[serde(rename = "profile")]
    pub r#profile: Option<String>,
    /// Whether audio samples are represented with negative and positive numbers (signed) or only positive numbers (unsigned). The supported value is Signed. (PCM Only)
    #[builder(into)]
    #[serde(rename = "signed")]
    pub r#signed: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for PresetAudioCodecOptions {
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
                    "bit_depth",
                    &self.r#bit_depth,
                ),
                to_pulumi_object_field(
                    "bit_order",
                    &self.r#bit_order,
                ),
                to_pulumi_object_field(
                    "profile",
                    &self.r#profile,
                ),
                to_pulumi_object_field(
                    "signed",
                    &self.r#signed,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for PresetAudioCodecOptions {
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
                    r#bit_depth: {
                        let field_value = match fields_map.get("bit_depth") {
                            Some(value) => value,
                            None => bail!("Missing field 'bit_depth' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#bit_order: {
                        let field_value = match fields_map.get("bit_order") {
                            Some(value) => value,
                            None => bail!("Missing field 'bit_order' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#profile: {
                        let field_value = match fields_map.get("profile") {
                            Some(value) => value,
                            None => bail!("Missing field 'profile' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#signed: {
                        let field_value = match fields_map.get("signed") {
                            Some(value) => value,
                            None => bail!("Missing field 'signed' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
