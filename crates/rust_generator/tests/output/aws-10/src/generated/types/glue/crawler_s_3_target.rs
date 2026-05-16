#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct CrawlerS3Target {
    /// The name of a connection which allows crawler to access data in S3 within a VPC.
    #[builder(into)]
    #[serde(rename = "connectionName")]
    pub r#connection_name: Option<String>,
    /// The ARN of the dead-letter SQS queue.
    #[builder(into)]
    #[serde(rename = "dlqEventQueueArn")]
    pub r#dlq_event_queue_arn: Option<String>,
    /// The ARN of the SQS queue to receive S3 notifications from.
    #[builder(into)]
    #[serde(rename = "eventQueueArn")]
    pub r#event_queue_arn: Option<String>,
    /// A list of glob patterns used to exclude from the crawl.
    #[builder(into)]
    #[serde(rename = "exclusions")]
    pub r#exclusions: Option<Vec<String>>,
    /// The path to the Amazon S3 target.
    #[builder(into)]
    #[serde(rename = "path")]
    pub r#path: String,
    /// Sets the number of files in each leaf folder to be crawled when crawling sample files in a dataset. If not set, all the files are crawled. A valid value is an integer between 1 and 249.
    #[builder(into)]
    #[serde(rename = "sampleSize")]
    pub r#sample_size: Option<i32>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for CrawlerS3Target {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("connection_name".to_string(), self.r#connection_name.to_pulumi_value().await);
            map.insert("dlq_event_queue_arn".to_string(), self.r#dlq_event_queue_arn.to_pulumi_value().await);
            map.insert("event_queue_arn".to_string(), self.r#event_queue_arn.to_pulumi_value().await);
            map.insert("exclusions".to_string(), self.r#exclusions.to_pulumi_value().await);
            map.insert("path".to_string(), self.r#path.to_pulumi_value().await);
            map.insert("sample_size".to_string(), self.r#sample_size.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for CrawlerS3Target {
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
                    r#connection_name: {
                        let field_value = match fields_map.get("connection_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'connection_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#dlq_event_queue_arn: {
                        let field_value = match fields_map.get("dlq_event_queue_arn") {
                            Some(value) => value,
                            None => bail!("Missing field 'dlq_event_queue_arn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#event_queue_arn: {
                        let field_value = match fields_map.get("event_queue_arn") {
                            Some(value) => value,
                            None => bail!("Missing field 'event_queue_arn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#exclusions: {
                        let field_value = match fields_map.get("exclusions") {
                            Some(value) => value,
                            None => bail!("Missing field 'exclusions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<String>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#path: {
                        let field_value = match fields_map.get("path") {
                            Some(value) => value,
                            None => bail!("Missing field 'path' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#sample_size: {
                        let field_value = match fields_map.get("sample_size") {
                            Some(value) => value,
                            None => bail!("Missing field 'sample_size' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<i32> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
