#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct JobTemplateConfigMuxStream {
    /// The container format. The default is `mp4`.
    #[builder(into)]
    pub r#container: Option<String>,
    /// List of ElementaryStream.key values multiplexed in this stream.
    #[builder(into)]
    pub r#elementary_streams: Option<Vec<String>>,
    /// Identifier of the encryption configuration to use.
    #[builder(into)]
    pub r#encryption_id: Option<String>,
    /// The name of the generated file.
    #[builder(into)]
    pub r#file_name: Option<String>,
    /// A unique key for this multiplexed stream.
    #[builder(into)]
    pub r#key: Option<String>,
    /// Segment settings for ts, fmp4 and vtt.
    /// Structure is documented below.
    #[builder(into)]
    pub r#segment_settings: Option<Box<super::super::types::transcoder::JobTemplateConfigMuxStreamSegmentSettings>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for JobTemplateConfigMuxStream {
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
                    "container",
                    &self.r#container,
                ),
                to_pulumi_object_field(
                    "elementaryStreams",
                    &self.r#elementary_streams,
                ),
                to_pulumi_object_field(
                    "encryptionId",
                    &self.r#encryption_id,
                ),
                to_pulumi_object_field(
                    "fileName",
                    &self.r#file_name,
                ),
                to_pulumi_object_field(
                    "key",
                    &self.r#key,
                ),
                to_pulumi_object_field(
                    "segmentSettings",
                    &self.r#segment_settings,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
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
                        let field_value = match fields_map.get("elementaryStreams") {
                            Some(value) => value,
                            None => bail!("Missing field 'elementaryStreams' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#encryption_id: {
                        let field_value = match fields_map.get("encryptionId") {
                            Some(value) => value,
                            None => bail!("Missing field 'encryptionId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#file_name: {
                        let field_value = match fields_map.get("fileName") {
                            Some(value) => value,
                            None => bail!("Missing field 'fileName' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("segmentSettings") {
                            Some(value) => value,
                            None => bail!("Missing field 'segmentSettings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
