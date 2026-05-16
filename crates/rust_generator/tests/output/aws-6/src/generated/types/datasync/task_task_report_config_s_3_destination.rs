#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct TaskTaskReportConfigS3Destination {
    /// Specifies the Amazon Resource Name (ARN) of the IAM policy that allows DataSync to upload a task report to your S3 bucket.
    #[builder(into)]
    #[serde(rename = "bucketAccessRoleArn")]
    pub r#bucket_access_role_arn: String,
    /// Specifies the ARN of the S3 bucket where DataSync uploads your report.
    #[builder(into)]
    #[serde(rename = "s3BucketArn")]
    pub r#s_3_bucket_arn: String,
    /// Specifies a bucket prefix for your report.
    #[builder(into)]
    #[serde(rename = "subdirectory")]
    pub r#subdirectory: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for TaskTaskReportConfigS3Destination {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("bucket_access_role_arn".to_string(), self.r#bucket_access_role_arn.to_pulumi_value().await);
            map.insert("s_3_bucket_arn".to_string(), self.r#s_3_bucket_arn.to_pulumi_value().await);
            map.insert("subdirectory".to_string(), self.r#subdirectory.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for TaskTaskReportConfigS3Destination {
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
                    r#bucket_access_role_arn: {
                        let field_value = match fields_map.get("bucket_access_role_arn") {
                            Some(value) => value,
                            None => bail!("Missing field 'bucket_access_role_arn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#s_3_bucket_arn: {
                        let field_value = match fields_map.get("s_3_bucket_arn") {
                            Some(value) => value,
                            None => bail!("Missing field 's_3_bucket_arn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#subdirectory: {
                        let field_value = match fields_map.get("subdirectory") {
                            Some(value) => value,
                            None => bail!("Missing field 'subdirectory' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
