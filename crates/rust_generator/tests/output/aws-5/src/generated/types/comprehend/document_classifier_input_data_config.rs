#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DocumentClassifierInputDataConfig {
    /// List of training datasets produced by Amazon SageMaker Ground Truth.
    /// Used if `data_format` is `AUGMENTED_MANIFEST`.
    /// See the `augmented_manifests` Configuration Block section below.
    #[builder(into)]
    #[serde(rename = "augmentedManifests")]
    pub r#augmented_manifests: Option<Vec<super::super::types::comprehend::DocumentClassifierInputDataConfigAugmentedManifest>>,
    /// The format for the training data.
    /// One of `COMPREHEND_CSV` or `AUGMENTED_MANIFEST`.
    #[builder(into)]
    #[serde(rename = "dataFormat")]
    pub r#data_format: Option<String>,
    /// Delimiter between labels when training a multi-label classifier.
    /// Valid values are `|`, `~`, `!`, `@`, `#`, `$`, `%`, `^`, `*`, `-`, `_`, `+`, `=`, `\`, `:`, `;`, `>`, `?`, `/`, `<space>`, and `<tab>`.
    /// Default is `|`.
    #[builder(into)]
    #[serde(rename = "labelDelimiter")]
    pub r#label_delimiter: Option<String>,
    /// Location of training documents.
    /// Used if `data_format` is `COMPREHEND_CSV`.
    #[builder(into)]
    #[serde(rename = "s3Uri")]
    pub r#s_3_uri: Option<String>,
    #[builder(into)]
    #[serde(rename = "testS3Uri")]
    pub r#test_s_3_uri: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for DocumentClassifierInputDataConfig {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "augmented_manifests".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#augmented_manifests,
                )
                .await,
            );
            map.insert(
                "data_format".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#data_format,
                )
                .await,
            );
            map.insert(
                "label_delimiter".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#label_delimiter,
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
                "test_s_3_uri".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#test_s_3_uri,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for DocumentClassifierInputDataConfig {
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
                    r#label_delimiter: {
                        let field_value = match fields_map.get("label_delimiter") {
                            Some(value) => value,
                            None => bail!("Missing field 'label_delimiter' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#test_s_3_uri: {
                        let field_value = match fields_map.get("test_s_3_uri") {
                            Some(value) => value,
                            None => bail!("Missing field 'test_s_3_uri' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
