#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct JobTemplateConfigElementaryStreamVideoStreamH264 {
    /// The video bitrate in bits per second.
    #[builder(into)]
    pub r#bitrate_bps: i32,
    /// Target CRF level. The default is '21'.
    #[builder(into)]
    pub r#crf_level: Option<i32>,
    /// The entropy coder to use. The default is 'cabac'.
    #[builder(into)]
    pub r#entropy_coder: Option<String>,
    /// The target video frame rate in frames per second (FPS).
    #[builder(into)]
    pub r#frame_rate: i32,
    /// Select the GOP size based on the specified duration. The default is '3s'.
    #[builder(into)]
    pub r#gop_duration: Option<String>,
    /// The height of the video in pixels.
    #[builder(into)]
    pub r#height_pixels: Option<i32>,
    /// HLG color format setting for H264.
    #[builder(into)]
    pub r#hlg: Option<Box<super::super::types::transcoder::JobTemplateConfigElementaryStreamVideoStreamH264Hlg>>,
    /// Pixel format to use. The default is 'yuv420p'.
    #[builder(into)]
    pub r#pixel_format: Option<String>,
    /// Enforces the specified codec preset. The default is 'veryfast'.
    #[builder(into)]
    pub r#preset: Option<String>,
    /// Enforces the specified codec profile.
    #[builder(into)]
    pub r#profile: Option<String>,
    /// Specify the mode. The default is 'vbr'.
    #[builder(into)]
    pub r#rate_control_mode: Option<String>,
    /// SDR color format setting for H264.
    #[builder(into)]
    pub r#sdr: Option<Box<super::super::types::transcoder::JobTemplateConfigElementaryStreamVideoStreamH264Sdr>>,
    /// Initial fullness of the Video Buffering Verifier (VBV) buffer in bits.
    #[builder(into)]
    pub r#vbv_fullness_bits: Option<i32>,
    /// Size of the Video Buffering Verifier (VBV) buffer in bits.
    #[builder(into)]
    pub r#vbv_size_bits: Option<i32>,
    /// The width of the video in pixels.
    #[builder(into)]
    pub r#width_pixels: Option<i32>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for JobTemplateConfigElementaryStreamVideoStreamH264 {
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
                    "bitrateBps",
                    &self.r#bitrate_bps,
                ),
                to_pulumi_object_field(
                    "crfLevel",
                    &self.r#crf_level,
                ),
                to_pulumi_object_field(
                    "entropyCoder",
                    &self.r#entropy_coder,
                ),
                to_pulumi_object_field(
                    "frameRate",
                    &self.r#frame_rate,
                ),
                to_pulumi_object_field(
                    "gopDuration",
                    &self.r#gop_duration,
                ),
                to_pulumi_object_field(
                    "heightPixels",
                    &self.r#height_pixels,
                ),
                to_pulumi_object_field(
                    "hlg",
                    &self.r#hlg,
                ),
                to_pulumi_object_field(
                    "pixelFormat",
                    &self.r#pixel_format,
                ),
                to_pulumi_object_field(
                    "preset",
                    &self.r#preset,
                ),
                to_pulumi_object_field(
                    "profile",
                    &self.r#profile,
                ),
                to_pulumi_object_field(
                    "rateControlMode",
                    &self.r#rate_control_mode,
                ),
                to_pulumi_object_field(
                    "sdr",
                    &self.r#sdr,
                ),
                to_pulumi_object_field(
                    "vbvFullnessBits",
                    &self.r#vbv_fullness_bits,
                ),
                to_pulumi_object_field(
                    "vbvSizeBits",
                    &self.r#vbv_size_bits,
                ),
                to_pulumi_object_field(
                    "widthPixels",
                    &self.r#width_pixels,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for JobTemplateConfigElementaryStreamVideoStreamH264 {
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
                    r#bitrate_bps: {
                        let field_value = match fields_map.get("bitrateBps") {
                            Some(value) => value,
                            None => bail!("Missing field 'bitrateBps' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#crf_level: {
                        let field_value = match fields_map.get("crfLevel") {
                            Some(value) => value,
                            None => bail!("Missing field 'crfLevel' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#entropy_coder: {
                        let field_value = match fields_map.get("entropyCoder") {
                            Some(value) => value,
                            None => bail!("Missing field 'entropyCoder' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#frame_rate: {
                        let field_value = match fields_map.get("frameRate") {
                            Some(value) => value,
                            None => bail!("Missing field 'frameRate' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#gop_duration: {
                        let field_value = match fields_map.get("gopDuration") {
                            Some(value) => value,
                            None => bail!("Missing field 'gopDuration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#height_pixels: {
                        let field_value = match fields_map.get("heightPixels") {
                            Some(value) => value,
                            None => bail!("Missing field 'heightPixels' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#hlg: {
                        let field_value = match fields_map.get("hlg") {
                            Some(value) => value,
                            None => bail!("Missing field 'hlg' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#pixel_format: {
                        let field_value = match fields_map.get("pixelFormat") {
                            Some(value) => value,
                            None => bail!("Missing field 'pixelFormat' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#preset: {
                        let field_value = match fields_map.get("preset") {
                            Some(value) => value,
                            None => bail!("Missing field 'preset' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#rate_control_mode: {
                        let field_value = match fields_map.get("rateControlMode") {
                            Some(value) => value,
                            None => bail!("Missing field 'rateControlMode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#sdr: {
                        let field_value = match fields_map.get("sdr") {
                            Some(value) => value,
                            None => bail!("Missing field 'sdr' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#vbv_fullness_bits: {
                        let field_value = match fields_map.get("vbvFullnessBits") {
                            Some(value) => value,
                            None => bail!("Missing field 'vbvFullnessBits' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#vbv_size_bits: {
                        let field_value = match fields_map.get("vbvSizeBits") {
                            Some(value) => value,
                            None => bail!("Missing field 'vbvSizeBits' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#width_pixels: {
                        let field_value = match fields_map.get("widthPixels") {
                            Some(value) => value,
                            None => bail!("Missing field 'widthPixels' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
