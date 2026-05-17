#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct JobConfigMuxStream {
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
    pub r#segment_settings: Option<Box<super::super::types::transcoder::JobConfigMuxStreamSegmentSettings>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for JobConfigMuxStream {
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
                    "container",
                    &self.r#container,
                ),
                to_pulumi_object_field(
                    "elementary_streams",
                    &self.r#elementary_streams,
                ),
                to_pulumi_object_field(
                    "encryption_id",
                    &self.r#encryption_id,
                ),
                to_pulumi_object_field(
                    "file_name",
                    &self.r#file_name,
                ),
                to_pulumi_object_field(
                    "key",
                    &self.r#key,
                ),
                to_pulumi_object_field(
                    "segment_settings",
                    &self.r#segment_settings,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for JobConfigMuxStream {
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
