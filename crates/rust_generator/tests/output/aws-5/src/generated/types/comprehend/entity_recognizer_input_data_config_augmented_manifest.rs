#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for EntityRecognizerInputDataConfigAugmentedManifest {
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
                "annotation_data_s_3_uri".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#annotation_data_s_3_uri,
                )
                .await,
            );
            map.insert(
                "attribute_names".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#attribute_names,
                )
                .await,
            );
            map.insert(
                "document_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#document_type,
                )
                .await,
            );
            map.insert(
                "s_3_uri".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#s_3_uri,
                )
                .await,
            );
            map.insert(
                "source_documents_s_3_uri".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#source_documents_s_3_uri,
                )
                .await,
            );
            map.insert(
                "split".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#split,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for EntityRecognizerInputDataConfigAugmentedManifest {
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
                    r#annotation_data_s_3_uri: {
                        let field_value = match fields_map.get("annotation_data_s_3_uri") {
                            Some(value) => value,
                            None => bail!("Missing field 'annotation_data_s_3_uri' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#attribute_names: {
                        let field_value = match fields_map.get("attribute_names") {
                            Some(value) => value,
                            None => bail!("Missing field 'attribute_names' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#document_type: {
                        let field_value = match fields_map.get("document_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'document_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#s_3_uri: {
                        let field_value = match fields_map.get("s_3_uri") {
                            Some(value) => value,
                            None => bail!("Missing field 's_3_uri' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#source_documents_s_3_uri: {
                        let field_value = match fields_map.get("source_documents_s_3_uri") {
                            Some(value) => value,
                            None => bail!("Missing field 'source_documents_s_3_uri' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#split: {
                        let field_value = match fields_map.get("split") {
                            Some(value) => value,
                            None => bail!("Missing field 'split' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
