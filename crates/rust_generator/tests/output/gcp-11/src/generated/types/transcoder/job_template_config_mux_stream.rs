#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct JobTemplateConfigMuxStream {
    /// The container format. The default is `mp4`.
    #[builder(into)]
    #[serde(rename = "container")]
    pub r#container: Option<String>,
    /// List of ElementaryStream.key values multiplexed in this stream.
    #[builder(into)]
    #[serde(rename = "elementaryStreams")]
    pub r#elementary_streams: Option<Vec<String>>,
    /// Identifier of the encryption configuration to use.
    #[builder(into)]
    #[serde(rename = "encryptionId")]
    pub r#encryption_id: Option<String>,
    /// The name of the generated file.
    #[builder(into)]
    #[serde(rename = "fileName")]
    pub r#file_name: Option<String>,
    /// A unique key for this multiplexed stream.
    #[builder(into)]
    #[serde(rename = "key")]
    pub r#key: Option<String>,
    /// Segment settings for ts, fmp4 and vtt.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "segmentSettings")]
    pub r#segment_settings: Option<Box<super::super::types::transcoder::JobTemplateConfigMuxStreamSegmentSettings>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for JobTemplateConfigMuxStream {
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
                "container".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#container,
                )
                .await,
            );
            map.insert(
                "elementary_streams".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#elementary_streams,
                )
                .await,
            );
            map.insert(
                "encryption_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#encryption_id,
                )
                .await,
            );
            map.insert(
                "file_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#file_name,
                )
                .await,
            );
            map.insert(
                "key".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#key,
                )
                .await,
            );
            map.insert(
                "segment_settings".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#segment_settings,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for JobTemplateConfigMuxStream {
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
                    r#container: {
                        let field_value = match fields_map.get("container") {
                            Some(value) => value,
                            None => bail!("Missing field 'container' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#elementary_streams: {
                        let field_value = match fields_map.get("elementary_streams") {
                            Some(value) => value,
                            None => bail!("Missing field 'elementary_streams' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#encryption_id: {
                        let field_value = match fields_map.get("encryption_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'encryption_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#file_name: {
                        let field_value = match fields_map.get("file_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'file_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#key: {
                        let field_value = match fields_map.get("key") {
                            Some(value) => value,
                            None => bail!("Missing field 'key' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#segment_settings: {
                        let field_value = match fields_map.get("segment_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'segment_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
