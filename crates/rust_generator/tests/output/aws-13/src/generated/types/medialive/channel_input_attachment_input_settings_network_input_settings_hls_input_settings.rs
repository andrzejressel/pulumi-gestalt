#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ChannelInputAttachmentInputSettingsNetworkInputSettingsHlsInputSettings {
    /// The bitrate is specified in bits per second, as in an HLS manifest.
    #[builder(into)]
    #[serde(rename = "bandwidth")]
    pub r#bandwidth: Option<i32>,
    /// Buffer segments.
    #[builder(into)]
    #[serde(rename = "bufferSegments")]
    pub r#buffer_segments: Option<i32>,
    /// The number of consecutive times that attempts to read a manifest or segment must fail before the input is considered unavailable.
    #[builder(into)]
    #[serde(rename = "retries")]
    pub r#retries: Option<i32>,
    /// The number of seconds between retries when an attempt to read a manifest or segment fails.
    #[builder(into)]
    #[serde(rename = "retryInterval")]
    pub r#retry_interval: Option<i32>,
    #[builder(into)]
    #[serde(rename = "scte35Source")]
    pub r#scte_35_source: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ChannelInputAttachmentInputSettingsNetworkInputSettingsHlsInputSettings {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "bandwidth",
                    &self.r#bandwidth,
                ),
                to_pulumi_object_field(
                    "buffer_segments",
                    &self.r#buffer_segments,
                ),
                to_pulumi_object_field(
                    "retries",
                    &self.r#retries,
                ),
                to_pulumi_object_field(
                    "retry_interval",
                    &self.r#retry_interval,
                ),
                to_pulumi_object_field(
                    "scte_35_source",
                    &self.r#scte_35_source,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ChannelInputAttachmentInputSettingsNetworkInputSettingsHlsInputSettings {
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
                    r#bandwidth: {
                        let field_value = match fields_map.get("bandwidth") {
                            Some(value) => value,
                            None => bail!("Missing field 'bandwidth' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#buffer_segments: {
                        let field_value = match fields_map.get("buffer_segments") {
                            Some(value) => value,
                            None => bail!("Missing field 'buffer_segments' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#retries: {
                        let field_value = match fields_map.get("retries") {
                            Some(value) => value,
                            None => bail!("Missing field 'retries' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#retry_interval: {
                        let field_value = match fields_map.get("retry_interval") {
                            Some(value) => value,
                            None => bail!("Missing field 'retry_interval' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#scte_35_source: {
                        let field_value = match fields_map.get("scte_35_source") {
                            Some(value) => value,
                            None => bail!("Missing field 'scte_35_source' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
