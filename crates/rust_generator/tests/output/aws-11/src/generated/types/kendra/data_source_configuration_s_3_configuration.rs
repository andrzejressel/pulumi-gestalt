#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DataSourceConfigurationS3Configuration {
    /// A block that provides the path to the S3 bucket that contains the user context filtering files for the data source. For the format of the file, see [Access control for S3 data sources](https://docs.aws.amazon.com/kendra/latest/dg/s3-acl.html). Detailed below.
    #[builder(into)]
    #[serde(rename = "accessControlListConfiguration")]
    pub r#access_control_list_configuration: Option<Box<super::super::types::kendra::DataSourceConfigurationS3ConfigurationAccessControlListConfiguration>>,
    /// The name of the bucket that contains the documents.
    #[builder(into)]
    #[serde(rename = "bucketName")]
    pub r#bucket_name: String,
    /// A block that defines the Document metadata files that contain information such as the document access control information, source URI, document author, and custom attributes. Each metadata file contains metadata about a single document. Detailed below.
    #[builder(into)]
    #[serde(rename = "documentsMetadataConfiguration")]
    pub r#documents_metadata_configuration: Option<Box<super::super::types::kendra::DataSourceConfigurationS3ConfigurationDocumentsMetadataConfiguration>>,
    /// A list of glob patterns for documents that should not be indexed. If a document that matches an inclusion prefix or inclusion pattern also matches an exclusion pattern, the document is not indexed. Refer to [Exclusion Patterns for more examples](https://docs.aws.amazon.com/kendra/latest/dg/API_S3DataSourceConfiguration.html#Kendra-Type-S3DataSourceConfiguration-ExclusionPatterns).
    #[builder(into)]
    #[serde(rename = "exclusionPatterns")]
    pub r#exclusion_patterns: Option<Vec<String>>,
    /// A list of glob patterns for documents that should be indexed. If a document that matches an inclusion pattern also matches an exclusion pattern, the document is not indexed. Refer to [Inclusion Patterns for more examples](https://docs.aws.amazon.com/kendra/latest/dg/API_S3DataSourceConfiguration.html#Kendra-Type-S3DataSourceConfiguration-InclusionPatterns).
    #[builder(into)]
    #[serde(rename = "inclusionPatterns")]
    pub r#inclusion_patterns: Option<Vec<String>>,
    /// A list of S3 prefixes for the documents that should be included in the index.
    #[builder(into)]
    #[serde(rename = "inclusionPrefixes")]
    pub r#inclusion_prefixes: Option<Vec<String>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for DataSourceConfigurationS3Configuration {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("access_control_list_configuration".to_string(), self.r#access_control_list_configuration.to_pulumi_value().await);
            map.insert("bucket_name".to_string(), self.r#bucket_name.to_pulumi_value().await);
            map.insert("documents_metadata_configuration".to_string(), self.r#documents_metadata_configuration.to_pulumi_value().await);
            map.insert("exclusion_patterns".to_string(), self.r#exclusion_patterns.to_pulumi_value().await);
            map.insert("inclusion_patterns".to_string(), self.r#inclusion_patterns.to_pulumi_value().await);
            map.insert("inclusion_prefixes".to_string(), self.r#inclusion_prefixes.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for DataSourceConfigurationS3Configuration {
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
                    r#access_control_list_configuration: {
                        let field_value = match fields_map.get("access_control_list_configuration") {
                            Some(value) => value,
                            None => bail!("Missing field 'access_control_list_configuration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::kendra::DataSourceConfigurationS3ConfigurationAccessControlListConfiguration>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#bucket_name: {
                        let field_value = match fields_map.get("bucket_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'bucket_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#documents_metadata_configuration: {
                        let field_value = match fields_map.get("documents_metadata_configuration") {
                            Some(value) => value,
                            None => bail!("Missing field 'documents_metadata_configuration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::kendra::DataSourceConfigurationS3ConfigurationDocumentsMetadataConfiguration>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#exclusion_patterns: {
                        let field_value = match fields_map.get("exclusion_patterns") {
                            Some(value) => value,
                            None => bail!("Missing field 'exclusion_patterns' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<String>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#inclusion_patterns: {
                        let field_value = match fields_map.get("inclusion_patterns") {
                            Some(value) => value,
                            None => bail!("Missing field 'inclusion_patterns' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<String>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#inclusion_prefixes: {
                        let field_value = match fields_map.get("inclusion_prefixes") {
                            Some(value) => value,
                            None => bail!("Missing field 'inclusion_prefixes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<String>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
