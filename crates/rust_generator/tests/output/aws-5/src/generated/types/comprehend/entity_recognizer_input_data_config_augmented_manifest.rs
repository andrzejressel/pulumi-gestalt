#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct EntityRecognizerInputDataConfigAugmentedManifest {
    /// Location of annotation files.
    #[builder(into)]
    #[serde(rename = "annotationDataS3Uri")]
    pub r#annotation_data_s_3_uri: Option<String>,
    /// The JSON attribute that contains the annotations for the training documents.
    #[builder(into)]
    #[serde(rename = "attributeNames")]
    pub r#attribute_names: Vec<String>,
    /// Type of augmented manifest.
    /// One of `PLAIN_TEXT_DOCUMENT` or `SEMI_STRUCTURED_DOCUMENT`.
    #[builder(into)]
    #[serde(rename = "documentType")]
    pub r#document_type: Option<String>,
    /// Location of augmented manifest file.
    #[builder(into)]
    #[serde(rename = "s3Uri")]
    pub r#s_3_uri: String,
    /// Location of source PDF files.
    #[builder(into)]
    #[serde(rename = "sourceDocumentsS3Uri")]
    pub r#source_documents_s_3_uri: Option<String>,
    /// Purpose of data in augmented manifest.
    /// One of `TRAIN` or `TEST`.
    #[builder(into)]
    #[serde(rename = "split")]
    pub r#split: Option<String>,
}
