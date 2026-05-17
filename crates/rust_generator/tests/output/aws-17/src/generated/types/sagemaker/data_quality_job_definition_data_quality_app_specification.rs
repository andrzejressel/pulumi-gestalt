#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DataQualityJobDefinitionDataQualityAppSpecification {
    /// Sets the environment variables in the container that the monitoring job runs. A list of key value pairs.
    #[builder(into)]
    #[serde(rename = "environment")]
    pub r#environment: Option<std::collections::HashMap<String, String>>,
    /// The container image that the data quality monitoring job runs.
    #[builder(into)]
    #[serde(rename = "imageUri")]
    pub r#image_uri: String,
    /// An Amazon S3 URI to a script that is called after analysis has been performed. Applicable only for the built-in (first party) containers.
    #[builder(into)]
    #[serde(rename = "postAnalyticsProcessorSourceUri")]
    pub r#post_analytics_processor_source_uri: Option<String>,
    /// An Amazon S3 URI to a script that is called per row prior to running analysis. It can base64 decode the payload and convert it into a flatted json so that the built-in container can use the converted data. Applicable only for the built-in (first party) containers.
    #[builder(into)]
    #[serde(rename = "recordPreprocessorSourceUri")]
    pub r#record_preprocessor_source_uri: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for DataQualityJobDefinitionDataQualityAppSpecification {
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
                "environment".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#environment,
                )
                .await,
            );
            map.insert(
                "image_uri".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#image_uri,
                )
                .await,
            );
            map.insert(
                "post_analytics_processor_source_uri".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#post_analytics_processor_source_uri,
                )
                .await,
            );
            map.insert(
                "record_preprocessor_source_uri".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#record_preprocessor_source_uri,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for DataQualityJobDefinitionDataQualityAppSpecification {
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
                    r#environment: {
                        let field_value = match fields_map.get("environment") {
                            Some(value) => value,
                            None => bail!("Missing field 'environment' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#image_uri: {
                        let field_value = match fields_map.get("image_uri") {
                            Some(value) => value,
                            None => bail!("Missing field 'image_uri' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#post_analytics_processor_source_uri: {
                        let field_value = match fields_map.get("post_analytics_processor_source_uri") {
                            Some(value) => value,
                            None => bail!("Missing field 'post_analytics_processor_source_uri' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#record_preprocessor_source_uri: {
                        let field_value = match fields_map.get("record_preprocessor_source_uri") {
                            Some(value) => value,
                            None => bail!("Missing field 'record_preprocessor_source_uri' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
