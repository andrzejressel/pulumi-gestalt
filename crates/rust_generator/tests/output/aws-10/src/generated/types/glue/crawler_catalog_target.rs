#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct CrawlerCatalogTarget {
    /// The name of the connection for an Amazon S3-backed Data Catalog table to be a target of the crawl when using a Catalog connection type paired with a `NETWORK` Connection type.
    #[builder(into)]
    #[serde(rename = "connectionName")]
    pub r#connection_name: Option<String>,
    /// The name of the Glue database to be synchronized.
    #[builder(into)]
    #[serde(rename = "databaseName")]
    pub r#database_name: String,
    /// A valid Amazon SQS ARN.
    /// 
    /// > **Note:** `deletion_behavior` of catalog target doesn't support `DEPRECATE_IN_DATABASE`.
    /// 
    /// > **Note:** `configuration` for catalog target crawlers will have `{ ... "Grouping": { "TableGroupingPolicy": "CombineCompatibleSchemas"} }` by default.
    #[builder(into)]
    #[serde(rename = "dlqEventQueueArn")]
    pub r#dlq_event_queue_arn: Option<String>,
    /// A valid Amazon SQS ARN.
    #[builder(into)]
    #[serde(rename = "eventQueueArn")]
    pub r#event_queue_arn: Option<String>,
    /// A list of catalog tables to be synchronized.
    #[builder(into)]
    #[serde(rename = "tables")]
    pub r#tables: Vec<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for CrawlerCatalogTarget {
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
                "connection_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#connection_name,
                )
                .await,
            );
            map.insert(
                "database_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#database_name,
                )
                .await,
            );
            map.insert(
                "dlq_event_queue_arn".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#dlq_event_queue_arn,
                )
                .await,
            );
            map.insert(
                "event_queue_arn".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#event_queue_arn,
                )
                .await,
            );
            map.insert(
                "tables".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#tables,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for CrawlerCatalogTarget {
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
                    r#connection_name: {
                        let field_value = match fields_map.get("connection_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'connection_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#database_name: {
                        let field_value = match fields_map.get("database_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'database_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#dlq_event_queue_arn: {
                        let field_value = match fields_map.get("dlq_event_queue_arn") {
                            Some(value) => value,
                            None => bail!("Missing field 'dlq_event_queue_arn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#event_queue_arn: {
                        let field_value = match fields_map.get("event_queue_arn") {
                            Some(value) => value,
                            None => bail!("Missing field 'event_queue_arn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#tables: {
                        let field_value = match fields_map.get("tables") {
                            Some(value) => value,
                            None => bail!("Missing field 'tables' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
