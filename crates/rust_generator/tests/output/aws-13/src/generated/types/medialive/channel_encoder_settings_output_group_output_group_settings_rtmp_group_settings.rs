#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ChannelEncoderSettingsOutputGroupOutputGroupSettingsRtmpGroupSettings {
    /// The ad marker type for this output group.
    #[builder(into)]
    #[serde(rename = "adMarkers")]
    pub r#ad_markers: Option<Vec<String>>,
    /// Authentication scheme to use when connecting with CDN.
    #[builder(into)]
    #[serde(rename = "authenticationScheme")]
    pub r#authentication_scheme: Option<String>,
    /// Controls behavior when content cache fills up.
    #[builder(into)]
    #[serde(rename = "cacheFullBehavior")]
    pub r#cache_full_behavior: Option<String>,
    /// Cache length in seconds, is used to calculate buffer size.
    #[builder(into)]
    #[serde(rename = "cacheLength")]
    pub r#cache_length: Option<i32>,
    /// Controls the types of data that passes to onCaptionInfo outputs.
    #[builder(into)]
    #[serde(rename = "captionData")]
    pub r#caption_data: Option<String>,
    /// Controls the behavior of the RTMP group if input becomes unavailable.
    #[builder(into)]
    #[serde(rename = "inputLossAction")]
    pub r#input_loss_action: Option<String>,
    /// Number of seconds to wait until a restart is initiated.
    #[builder(into)]
    #[serde(rename = "restartDelay")]
    pub r#restart_delay: Option<i32>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ChannelEncoderSettingsOutputGroupOutputGroupSettingsRtmpGroupSettings {
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
                    "adMarkers",
                    &self.r#ad_markers,
                ),
                to_pulumi_object_field(
                    "authenticationScheme",
                    &self.r#authentication_scheme,
                ),
                to_pulumi_object_field(
                    "cacheFullBehavior",
                    &self.r#cache_full_behavior,
                ),
                to_pulumi_object_field(
                    "cacheLength",
                    &self.r#cache_length,
                ),
                to_pulumi_object_field(
                    "captionData",
                    &self.r#caption_data,
                ),
                to_pulumi_object_field(
                    "inputLossAction",
                    &self.r#input_loss_action,
                ),
                to_pulumi_object_field(
                    "restartDelay",
                    &self.r#restart_delay,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ChannelEncoderSettingsOutputGroupOutputGroupSettingsRtmpGroupSettings {
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
                    r#ad_markers: {
                        let field_value = match fields_map.get("adMarkers") {
                            Some(value) => value,
                            None => bail!("Missing field 'adMarkers' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#authentication_scheme: {
                        let field_value = match fields_map.get("authenticationScheme") {
                            Some(value) => value,
                            None => bail!("Missing field 'authenticationScheme' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cache_full_behavior: {
                        let field_value = match fields_map.get("cacheFullBehavior") {
                            Some(value) => value,
                            None => bail!("Missing field 'cacheFullBehavior' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cache_length: {
                        let field_value = match fields_map.get("cacheLength") {
                            Some(value) => value,
                            None => bail!("Missing field 'cacheLength' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#caption_data: {
                        let field_value = match fields_map.get("captionData") {
                            Some(value) => value,
                            None => bail!("Missing field 'captionData' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#input_loss_action: {
                        let field_value = match fields_map.get("inputLossAction") {
                            Some(value) => value,
                            None => bail!("Missing field 'inputLossAction' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#restart_delay: {
                        let field_value = match fields_map.get("restartDelay") {
                            Some(value) => value,
                            None => bail!("Missing field 'restartDelay' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
