#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DataSourceCustomDocumentEnrichmentConfiguration {
    /// Configuration information to alter document attributes or metadata fields and content when ingesting documents into Amazon Kendra. Minimum number of `0` items. Maximum number of `100` items. Detailed below.
    #[builder(into)]
    #[serde(rename = "inlineConfigurations")]
    pub r#inline_configurations: Option<Vec<super::super::types::kendra::DataSourceCustomDocumentEnrichmentConfigurationInlineConfiguration>>,
    /// A block that specifies the configuration information for invoking a Lambda function in AWS Lambda on the structured documents with their metadata and text extracted. You can use a Lambda function to apply advanced logic for creating, modifying, or deleting document metadata and content. For more information, see [Advanced data manipulation](https://docs.aws.amazon.com/kendra/latest/dg/custom-document-enrichment.html#advanced-data-manipulation). Detailed below.
    #[builder(into)]
    #[serde(rename = "postExtractionHookConfiguration")]
    pub r#post_extraction_hook_configuration: Option<Box<super::super::types::kendra::DataSourceCustomDocumentEnrichmentConfigurationPostExtractionHookConfiguration>>,
    /// Configuration information for invoking a Lambda function in AWS Lambda on the original or raw documents before extracting their metadata and text. You can use a Lambda function to apply advanced logic for creating, modifying, or deleting document metadata and content. For more information, see [Advanced data manipulation](https://docs.aws.amazon.com/kendra/latest/dg/custom-document-enrichment.html#advanced-data-manipulation). Detailed below.
    #[builder(into)]
    #[serde(rename = "preExtractionHookConfiguration")]
    pub r#pre_extraction_hook_configuration: Option<Box<super::super::types::kendra::DataSourceCustomDocumentEnrichmentConfigurationPreExtractionHookConfiguration>>,
    /// The Amazon Resource Name (ARN) of a role with permission to run `pre_extraction_hook_configuration` and `post_extraction_hook_configuration` for altering document metadata and content during the document ingestion process. For more information, see [IAM roles for Amazon Kendra](https://docs.aws.amazon.com/kendra/latest/dg/iam-roles.html).
    #[builder(into)]
    #[serde(rename = "roleArn")]
    pub r#role_arn: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for DataSourceCustomDocumentEnrichmentConfiguration {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("inline_configurations".to_string(), self.r#inline_configurations.to_pulumi_value().await);
            map.insert("post_extraction_hook_configuration".to_string(), self.r#post_extraction_hook_configuration.to_pulumi_value().await);
            map.insert("pre_extraction_hook_configuration".to_string(), self.r#pre_extraction_hook_configuration.to_pulumi_value().await);
            map.insert("role_arn".to_string(), self.r#role_arn.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for DataSourceCustomDocumentEnrichmentConfiguration {
    fn from_pulumi_value(
        value: &pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    ) -> pulumi_gestalt_rust::__private::rootcause::Result<Self> {
        use std::collections::BTreeMap;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValueContent;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue;
        use pulumi_gestalt_rust::__private::rootcause::bail;

        match value.content {
            PulumiValueContent::Object(ref obj) => {
                let fields_map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> =
                    obj.iter().cloned().collect();

                Ok(Self {
                    r#inline_configurations: {
                        let field_value = match fields_map.get("inline_configurations") {
                            Some(value) => value,
                            None => bail!("Missing field 'inline_configurations' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<super::super::types::kendra::DataSourceCustomDocumentEnrichmentConfigurationInlineConfiguration>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#post_extraction_hook_configuration: {
                        let field_value = match fields_map.get("post_extraction_hook_configuration") {
                            Some(value) => value,
                            None => bail!("Missing field 'post_extraction_hook_configuration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::kendra::DataSourceCustomDocumentEnrichmentConfigurationPostExtractionHookConfiguration>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#pre_extraction_hook_configuration: {
                        let field_value = match fields_map.get("pre_extraction_hook_configuration") {
                            Some(value) => value,
                            None => bail!("Missing field 'pre_extraction_hook_configuration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::kendra::DataSourceCustomDocumentEnrichmentConfigurationPreExtractionHookConfiguration>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#role_arn: {
                        let field_value = match fields_map.get("role_arn") {
                            Some(value) => value,
                            None => bail!("Missing field 'role_arn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
