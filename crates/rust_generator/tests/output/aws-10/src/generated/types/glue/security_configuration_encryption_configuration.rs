#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct SecurityConfigurationEncryptionConfiguration {
    #[builder(into)]
    #[serde(rename = "cloudwatchEncryption")]
    pub r#cloudwatch_encryption: Box<super::super::types::glue::SecurityConfigurationEncryptionConfigurationCloudwatchEncryption>,
    #[builder(into)]
    #[serde(rename = "jobBookmarksEncryption")]
    pub r#job_bookmarks_encryption: Box<super::super::types::glue::SecurityConfigurationEncryptionConfigurationJobBookmarksEncryption>,
    /// A `s3_encryption ` block as described below, which contains encryption configuration for S3 data.
    #[builder(into)]
    #[serde(rename = "s3Encryption")]
    pub r#s_3_encryption: Box<super::super::types::glue::SecurityConfigurationEncryptionConfigurationS3Encryption>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for SecurityConfigurationEncryptionConfiguration {
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
                "cloudwatch_encryption".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#cloudwatch_encryption,
                )
                .await,
            );
            map.insert(
                "job_bookmarks_encryption".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#job_bookmarks_encryption,
                )
                .await,
            );
            map.insert(
                "s_3_encryption".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#s_3_encryption,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for SecurityConfigurationEncryptionConfiguration {
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
                    r#cloudwatch_encryption: {
                        let field_value = match fields_map.get("cloudwatch_encryption") {
                            Some(value) => value,
                            None => bail!("Missing field 'cloudwatch_encryption' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#job_bookmarks_encryption: {
                        let field_value = match fields_map.get("job_bookmarks_encryption") {
                            Some(value) => value,
                            None => bail!("Missing field 'job_bookmarks_encryption' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#s_3_encryption: {
                        let field_value = match fields_map.get("s_3_encryption") {
                            Some(value) => value,
                            None => bail!("Missing field 's_3_encryption' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
