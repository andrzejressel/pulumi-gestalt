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
