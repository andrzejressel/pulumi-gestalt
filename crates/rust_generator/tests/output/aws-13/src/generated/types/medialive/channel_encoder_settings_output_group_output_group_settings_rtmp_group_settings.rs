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
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "ad_markers".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#ad_markers,
                )
                .await,
            );
            map.insert(
                "authentication_scheme".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#authentication_scheme,
                )
                .await,
            );
            map.insert(
                "cache_full_behavior".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#cache_full_behavior,
                )
                .await,
            );
            map.insert(
                "cache_length".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#cache_length,
                )
                .await,
            );
            map.insert(
                "caption_data".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#caption_data,
                )
                .await,
            );
            map.insert(
                "input_loss_action".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#input_loss_action,
                )
                .await,
            );
            map.insert(
                "restart_delay".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#restart_delay,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
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
                        let field_value = match fields_map.get("ad_markers") {
                            Some(value) => value,
                            None => bail!("Missing field 'ad_markers' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#authentication_scheme: {
                        let field_value = match fields_map.get("authentication_scheme") {
                            Some(value) => value,
                            None => bail!("Missing field 'authentication_scheme' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cache_full_behavior: {
                        let field_value = match fields_map.get("cache_full_behavior") {
                            Some(value) => value,
                            None => bail!("Missing field 'cache_full_behavior' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cache_length: {
                        let field_value = match fields_map.get("cache_length") {
                            Some(value) => value,
                            None => bail!("Missing field 'cache_length' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#caption_data: {
                        let field_value = match fields_map.get("caption_data") {
                            Some(value) => value,
                            None => bail!("Missing field 'caption_data' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#input_loss_action: {
                        let field_value = match fields_map.get("input_loss_action") {
                            Some(value) => value,
                            None => bail!("Missing field 'input_loss_action' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#restart_delay: {
                        let field_value = match fields_map.get("restart_delay") {
                            Some(value) => value,
                            None => bail!("Missing field 'restart_delay' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
