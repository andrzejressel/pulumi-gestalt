#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetTopicIngestionDataSourceSettingAwsKinese {
    /// AWS role ARN to be used for Federated Identity authentication with
    /// Kinesis. Check the Pub/Sub docs for how to set up this role and the
    /// required permissions that need to be attached to it.
    #[builder(into)]
    #[serde(rename = "awsRoleArn")]
    pub r#aws_role_arn: String,
    /// The Kinesis consumer ARN to used for ingestion in
    /// Enhanced Fan-Out mode. The consumer must be already
    /// created and ready to be used.
    #[builder(into)]
    #[serde(rename = "consumerArn")]
    pub r#consumer_arn: String,
    /// The GCP service account to be used for Federated Identity authentication
    /// with Kinesis (via a 'AssumeRoleWithWebIdentity' call for the provided
    /// role). The 'awsRoleArn' must be set up with 'accounts.google.com:sub'
    /// equals to this service account number.
    #[builder(into)]
    #[serde(rename = "gcpServiceAccount")]
    pub r#gcp_service_account: String,
    /// The Kinesis stream ARN to ingest data from.
    #[builder(into)]
    #[serde(rename = "streamArn")]
    pub r#stream_arn: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetTopicIngestionDataSourceSettingAwsKinese {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "aws_role_arn",
                    &self.r#aws_role_arn,
                ),
                to_pulumi_object_field(
                    "consumer_arn",
                    &self.r#consumer_arn,
                ),
                to_pulumi_object_field(
                    "gcp_service_account",
                    &self.r#gcp_service_account,
                ),
                to_pulumi_object_field(
                    "stream_arn",
                    &self.r#stream_arn,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetTopicIngestionDataSourceSettingAwsKinese {
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
                    r#aws_role_arn: {
                        let field_value = match fields_map.get("aws_role_arn") {
                            Some(value) => value,
                            None => bail!("Missing field 'aws_role_arn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#consumer_arn: {
                        let field_value = match fields_map.get("consumer_arn") {
                            Some(value) => value,
                            None => bail!("Missing field 'consumer_arn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#gcp_service_account: {
                        let field_value = match fields_map.get("gcp_service_account") {
                            Some(value) => value,
                            None => bail!("Missing field 'gcp_service_account' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#stream_arn: {
                        let field_value = match fields_map.get("stream_arn") {
                            Some(value) => value,
                            None => bail!("Missing field 'stream_arn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
