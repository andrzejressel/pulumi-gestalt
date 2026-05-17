#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct JobTemplateConfig {
    /// Ad break.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "adBreaks")]
    pub r#ad_breaks: Option<Vec<super::super::types::transcoder::JobTemplateConfigAdBreak>>,
    /// List of input assets stored in Cloud Storage.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "editLists")]
    pub r#edit_lists: Option<Vec<super::super::types::transcoder::JobTemplateConfigEditList>>,
    /// List of input assets stored in Cloud Storage.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "elementaryStreams")]
    pub r#elementary_streams: Option<Vec<super::super::types::transcoder::JobTemplateConfigElementaryStream>>,
    /// List of encryption configurations for the content.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "encryptions")]
    pub r#encryptions: Option<Vec<super::super::types::transcoder::JobTemplateConfigEncryption>>,
    /// List of input assets stored in Cloud Storage.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "inputs")]
    pub r#inputs: Option<Vec<super::super::types::transcoder::JobTemplateConfigInput>>,
    /// Manifest configuration.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "manifests")]
    pub r#manifests: Option<Vec<super::super::types::transcoder::JobTemplateConfigManifest>>,
    /// Multiplexing settings for output stream.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "muxStreams")]
    pub r#mux_streams: Option<Vec<super::super::types::transcoder::JobTemplateConfigMuxStream>>,
    /// Location of output file(s) in a Cloud Storage bucket.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "output")]
    pub r#output: Option<Box<super::super::types::transcoder::JobTemplateConfigOutput>>,
    /// List of overlays on the output video, in descending Z-order.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "overlays")]
    pub r#overlays: Option<Vec<super::super::types::transcoder::JobTemplateConfigOverlay>>,
    /// Pub/Sub destination.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "pubsubDestination")]
    pub r#pubsub_destination: Option<Box<super::super::types::transcoder::JobTemplateConfigPubsubDestination>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for JobTemplateConfig {
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
                    "ad_breaks",
                    &self.r#ad_breaks,
                ),
                to_pulumi_object_field(
                    "edit_lists",
                    &self.r#edit_lists,
                ),
                to_pulumi_object_field(
                    "elementary_streams",
                    &self.r#elementary_streams,
                ),
                to_pulumi_object_field(
                    "encryptions",
                    &self.r#encryptions,
                ),
                to_pulumi_object_field(
                    "inputs",
                    &self.r#inputs,
                ),
                to_pulumi_object_field(
                    "manifests",
                    &self.r#manifests,
                ),
                to_pulumi_object_field(
                    "mux_streams",
                    &self.r#mux_streams,
                ),
                to_pulumi_object_field(
                    "output",
                    &self.r#output,
                ),
                to_pulumi_object_field(
                    "overlays",
                    &self.r#overlays,
                ),
                to_pulumi_object_field(
                    "pubsub_destination",
                    &self.r#pubsub_destination,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for JobTemplateConfig {
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
                    r#ad_breaks: {
                        let field_value = match fields_map.get("ad_breaks") {
                            Some(value) => value,
                            None => bail!("Missing field 'ad_breaks' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#edit_lists: {
                        let field_value = match fields_map.get("edit_lists") {
                            Some(value) => value,
                            None => bail!("Missing field 'edit_lists' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#encryptions: {
                        let field_value = match fields_map.get("encryptions") {
                            Some(value) => value,
                            None => bail!("Missing field 'encryptions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#inputs: {
                        let field_value = match fields_map.get("inputs") {
                            Some(value) => value,
                            None => bail!("Missing field 'inputs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#manifests: {
                        let field_value = match fields_map.get("manifests") {
                            Some(value) => value,
                            None => bail!("Missing field 'manifests' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#mux_streams: {
                        let field_value = match fields_map.get("mux_streams") {
                            Some(value) => value,
                            None => bail!("Missing field 'mux_streams' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#output: {
                        let field_value = match fields_map.get("output") {
                            Some(value) => value,
                            None => bail!("Missing field 'output' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#overlays: {
                        let field_value = match fields_map.get("overlays") {
                            Some(value) => value,
                            None => bail!("Missing field 'overlays' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#pubsub_destination: {
                        let field_value = match fields_map.get("pubsub_destination") {
                            Some(value) => value,
                            None => bail!("Missing field 'pubsub_destination' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
