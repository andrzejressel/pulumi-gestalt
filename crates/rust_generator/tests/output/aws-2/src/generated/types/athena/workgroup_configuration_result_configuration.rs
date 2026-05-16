#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct WorkgroupConfigurationResultConfiguration {
    /// That an Amazon S3 canned ACL should be set to control ownership of stored query results. See ACL Configuration below.
    #[builder(into)]
    #[serde(rename = "aclConfiguration")]
    pub r#acl_configuration: Option<Box<super::super::types::athena::WorkgroupConfigurationResultConfigurationAclConfiguration>>,
    /// Configuration block with encryption settings. See Encryption Configuration below.
    #[builder(into)]
    #[serde(rename = "encryptionConfiguration")]
    pub r#encryption_configuration: Option<Box<super::super::types::athena::WorkgroupConfigurationResultConfigurationEncryptionConfiguration>>,
    /// AWS account ID that you expect to be the owner of the Amazon S3 bucket.
    #[builder(into)]
    #[serde(rename = "expectedBucketOwner")]
    pub r#expected_bucket_owner: Option<String>,
    /// Location in Amazon S3 where your query results are stored, such as `s3://path/to/query/bucket/`. For more information, see [Queries and Query Result Files](https://docs.aws.amazon.com/athena/latest/ug/querying.html).
    #[builder(into)]
    #[serde(rename = "outputLocation")]
    pub r#output_location: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for WorkgroupConfigurationResultConfiguration {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("acl_configuration".to_string(), self.r#acl_configuration.to_pulumi_value().await);
            map.insert("encryption_configuration".to_string(), self.r#encryption_configuration.to_pulumi_value().await);
            map.insert("expected_bucket_owner".to_string(), self.r#expected_bucket_owner.to_pulumi_value().await);
            map.insert("output_location".to_string(), self.r#output_location.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for WorkgroupConfigurationResultConfiguration {
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
                    r#acl_configuration: {
                        let field_value = match fields_map.get("acl_configuration") {
                            Some(value) => value,
                            None => bail!("Missing field 'acl_configuration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::athena::WorkgroupConfigurationResultConfigurationAclConfiguration>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#encryption_configuration: {
                        let field_value = match fields_map.get("encryption_configuration") {
                            Some(value) => value,
                            None => bail!("Missing field 'encryption_configuration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::athena::WorkgroupConfigurationResultConfigurationEncryptionConfiguration>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#expected_bucket_owner: {
                        let field_value = match fields_map.get("expected_bucket_owner") {
                            Some(value) => value,
                            None => bail!("Missing field 'expected_bucket_owner' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#output_location: {
                        let field_value = match fields_map.get("output_location") {
                            Some(value) => value,
                            None => bail!("Missing field 'output_location' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
