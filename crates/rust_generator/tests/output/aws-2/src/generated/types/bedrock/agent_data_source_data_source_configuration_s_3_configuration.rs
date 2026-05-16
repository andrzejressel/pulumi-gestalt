#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AgentDataSourceDataSourceConfigurationS3Configuration {
    /// ARN of the bucket that contains the data source.
    #[builder(into)]
    #[serde(rename = "bucketArn")]
    pub r#bucket_arn: String,
    /// Bucket account owner ID for the S3 bucket.
    #[builder(into)]
    #[serde(rename = "bucketOwnerAccountId")]
    pub r#bucket_owner_account_id: Option<String>,
    /// List of S3 prefixes that define the object containing the data sources. For more information, see [Organizing objects using prefixes](https://docs.aws.amazon.com/AmazonS3/latest/userguide/using-prefixes.html).
    #[builder(into)]
    #[serde(rename = "inclusionPrefixes")]
    pub r#inclusion_prefixes: Option<Vec<String>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for AgentDataSourceDataSourceConfigurationS3Configuration {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("bucket_arn".to_string(), self.r#bucket_arn.to_pulumi_value().await);
            map.insert("bucket_owner_account_id".to_string(), self.r#bucket_owner_account_id.to_pulumi_value().await);
            map.insert("inclusion_prefixes".to_string(), self.r#inclusion_prefixes.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for AgentDataSourceDataSourceConfigurationS3Configuration {
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
                    r#bucket_arn: {
                        let field_value = match fields_map.get("bucket_arn") {
                            Some(value) => value,
                            None => bail!("Missing field 'bucket_arn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#bucket_owner_account_id: {
                        let field_value = match fields_map.get("bucket_owner_account_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'bucket_owner_account_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
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
