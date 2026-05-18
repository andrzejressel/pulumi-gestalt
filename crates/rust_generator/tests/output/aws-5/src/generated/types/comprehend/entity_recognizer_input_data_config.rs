#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct EntityRecognizerInputDataConfig {
    /// Specifies location of the document annotation data.
    /// See the `annotations` Configuration Block section below.
    /// One of `annotations` or `entity_list` is required.
    #[builder(into)]
    #[serde(rename = "annotations")]
    pub r#annotations: Option<Box<super::super::types::comprehend::EntityRecognizerInputDataConfigAnnotations>>,
    /// List of training datasets produced by Amazon SageMaker Ground Truth.
    /// Used if `data_format` is `AUGMENTED_MANIFEST`.
    /// See the `augmented_manifests` Configuration Block section below.
    #[builder(into)]
    #[serde(rename = "augmentedManifests")]
    pub r#augmented_manifests: Option<Vec<super::super::types::comprehend::EntityRecognizerInputDataConfigAugmentedManifest>>,
    /// The format for the training data.
    /// One of `COMPREHEND_CSV` or `AUGMENTED_MANIFEST`.
    #[builder(into)]
    #[serde(rename = "dataFormat")]
    pub r#data_format: Option<String>,
    /// Specifies a collection of training documents.
    /// Used if `data_format` is `COMPREHEND_CSV`.
    /// See the `documents` Configuration Block section below.
    #[builder(into)]
    #[serde(rename = "documents")]
    pub r#documents: Option<Box<super::super::types::comprehend::EntityRecognizerInputDataConfigDocuments>>,
    /// Specifies location of the entity list data.
    /// See the `entity_list` Configuration Block section below.
    /// One of `entity_list` or `annotations` is required.
    #[builder(into)]
    #[serde(rename = "entityList")]
    pub r#entity_list: Option<Box<super::super::types::comprehend::EntityRecognizerInputDataConfigEntityList>>,
    /// Set of entity types to be recognized.
    /// Has a maximum of 25 items.
    /// See the `entity_types` Configuration Block section below.
    #[builder(into)]
    #[serde(rename = "entityTypes")]
    pub r#entity_types: Vec<super::super::types::comprehend::EntityRecognizerInputDataConfigEntityType>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for EntityRecognizerInputDataConfig {
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
                    "annotations",
                    &self.r#annotations,
                ),
                to_pulumi_object_field(
                    "augmented_manifests",
                    &self.r#augmented_manifests,
                ),
                to_pulumi_object_field(
                    "data_format",
                    &self.r#data_format,
                ),
                to_pulumi_object_field(
                    "documents",
                    &self.r#documents,
                ),
                to_pulumi_object_field(
                    "entity_list",
                    &self.r#entity_list,
                ),
                to_pulumi_object_field(
                    "entity_types",
                    &self.r#entity_types,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for EntityRecognizerInputDataConfig {
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
                    r#annotations: {
                        let field_value = match fields_map.get("annotations") {
                            Some(value) => value,
                            None => bail!("Missing field 'annotations' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#augmented_manifests: {
                        let field_value = match fields_map.get("augmented_manifests") {
                            Some(value) => value,
                            None => bail!("Missing field 'augmented_manifests' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#data_format: {
                        let field_value = match fields_map.get("data_format") {
                            Some(value) => value,
                            None => bail!("Missing field 'data_format' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#documents: {
                        let field_value = match fields_map.get("documents") {
                            Some(value) => value,
                            None => bail!("Missing field 'documents' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#entity_list: {
                        let field_value = match fields_map.get("entity_list") {
                            Some(value) => value,
                            None => bail!("Missing field 'entity_list' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#entity_types: {
                        let field_value = match fields_map.get("entity_types") {
                            Some(value) => value,
                            None => bail!("Missing field 'entity_types' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
