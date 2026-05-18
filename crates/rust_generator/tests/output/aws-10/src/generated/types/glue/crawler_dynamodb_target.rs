#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct CrawlerDynamodbTarget {
    /// The name of the DynamoDB table to crawl.
    #[builder(into)]
    #[serde(rename = "path")]
    pub r#path: String,
    /// Indicates whether to scan all the records, or to sample rows from the table. Scanning all the records can take a long time when the table is not a high throughput table.  defaults to `true`.
    #[builder(into)]
    #[serde(rename = "scanAll")]
    pub r#scan_all: Option<bool>,
    /// The percentage of the configured read capacity units to use by the AWS Glue crawler. The valid values are null or a value between 0.1 to 1.5.
    #[builder(into)]
    #[serde(rename = "scanRate")]
    pub r#scan_rate: Option<f64>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for CrawlerDynamodbTarget {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > + Send {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "path",
                    &self.r#path,
                ),
                to_pulumi_object_field(
                    "scan_all",
                    &self.r#scan_all,
                ),
                to_pulumi_object_field(
                    "scan_rate",
                    &self.r#scan_rate,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for CrawlerDynamodbTarget {
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
                    r#path: {
                        let field_value = match fields_map.get("path") {
                            Some(value) => value,
                            None => bail!("Missing field 'path' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#scan_all: {
                        let field_value = match fields_map.get("scan_all") {
                            Some(value) => value,
                            None => bail!("Missing field 'scan_all' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#scan_rate: {
                        let field_value = match fields_map.get("scan_rate") {
                            Some(value) => value,
                            None => bail!("Missing field 'scan_rate' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
