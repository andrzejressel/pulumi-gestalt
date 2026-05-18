#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AiIndexMetadata {
    /// The configuration of the Matching Engine Index.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "config")]
    pub r#config: Option<Box<super::super::types::vertex::AiIndexMetadataConfig>>,
    /// Allows inserting, updating  or deleting the contents of the Matching Engine Index.
    /// The string must be a valid Cloud Storage directory path. If this
    /// field is set when calling IndexService.UpdateIndex, then no other
    /// Index field can be also updated as part of the same call.
    /// The expected structure and format of the files this URI points to is
    /// described at https://cloud.google.com/vertex-ai/docs/matching-engine/using-matching-engine#input-data-format
    #[builder(into)]
    #[serde(rename = "contentsDeltaUri")]
    pub r#contents_delta_uri: String,
    /// If this field is set together with contentsDeltaUri when calling IndexService.UpdateIndex,
    /// then existing content of the Index will be replaced by the data from the contentsDeltaUri.
    #[builder(into)]
    #[serde(rename = "isCompleteOverwrite")]
    pub r#is_complete_overwrite: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for AiIndexMetadata {
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
                    "config",
                    &self.r#config,
                ),
                to_pulumi_object_field(
                    "contents_delta_uri",
                    &self.r#contents_delta_uri,
                ),
                to_pulumi_object_field(
                    "is_complete_overwrite",
                    &self.r#is_complete_overwrite,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for AiIndexMetadata {
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
                    r#config: {
                        let field_value = match fields_map.get("config") {
                            Some(value) => value,
                            None => bail!("Missing field 'config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#contents_delta_uri: {
                        let field_value = match fields_map.get("contents_delta_uri") {
                            Some(value) => value,
                            None => bail!("Missing field 'contents_delta_uri' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#is_complete_overwrite: {
                        let field_value = match fields_map.get("is_complete_overwrite") {
                            Some(value) => value,
                            None => bail!("Missing field 'is_complete_overwrite' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
