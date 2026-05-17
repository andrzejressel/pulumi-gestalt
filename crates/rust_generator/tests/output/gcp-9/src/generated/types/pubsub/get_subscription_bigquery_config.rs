#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetSubscriptionBigqueryConfig {
    /// When true and use_topic_schema or use_table_schema is true, any fields that are a part of the topic schema or message schema that
    /// are not part of the BigQuery table schema are dropped when writing to BigQuery. Otherwise, the schemas must be kept in sync
    /// and any messages with extra fields are not written and remain in the subscription's backlog.
    #[builder(into)]
    #[serde(rename = "dropUnknownFields")]
    pub r#drop_unknown_fields: bool,
    /// The service account to use to write to BigQuery. If not specified, the Pub/Sub
    /// [service agent](https://cloud.google.com/iam/docs/service-agents),
    /// service-{project_number}@gcp-sa-pubsub.iam.gserviceaccount.com, is used.
    #[builder(into)]
    #[serde(rename = "serviceAccountEmail")]
    pub r#service_account_email: String,
    /// The name of the table to which to write data, of the form {projectId}.{datasetId}.{tableId}
    #[builder(into)]
    #[serde(rename = "table")]
    pub r#table: String,
    /// When true, use the BigQuery table's schema as the columns to write to in BigQuery. Messages
    /// must be published in JSON format. Only one of use_topic_schema and use_table_schema can be set.
    #[builder(into)]
    #[serde(rename = "useTableSchema")]
    pub r#use_table_schema: bool,
    /// When true, use the topic's schema as the columns to write to in BigQuery, if it exists.
    /// Only one of use_topic_schema and use_table_schema can be set.
    #[builder(into)]
    #[serde(rename = "useTopicSchema")]
    pub r#use_topic_schema: bool,
    /// When true, write the subscription name, messageId, publishTime, attributes, and orderingKey to additional columns in the table.
    /// The subscription name, messageId, and publishTime fields are put in their own columns while all other message properties (other than data) are written to a JSON object in the attributes column.
    #[builder(into)]
    #[serde(rename = "writeMetadata")]
    pub r#write_metadata: bool,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetSubscriptionBigqueryConfig {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "drop_unknown_fields".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#drop_unknown_fields,
                )
                .await,
            );
            map.insert(
                "service_account_email".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#service_account_email,
                )
                .await,
            );
            map.insert(
                "table".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#table,
                )
                .await,
            );
            map.insert(
                "use_table_schema".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#use_table_schema,
                )
                .await,
            );
            map.insert(
                "use_topic_schema".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#use_topic_schema,
                )
                .await,
            );
            map.insert(
                "write_metadata".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#write_metadata,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetSubscriptionBigqueryConfig {
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
                    r#drop_unknown_fields: {
                        let field_value = match fields_map.get("drop_unknown_fields") {
                            Some(value) => value,
                            None => bail!("Missing field 'drop_unknown_fields' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#service_account_email: {
                        let field_value = match fields_map.get("service_account_email") {
                            Some(value) => value,
                            None => bail!("Missing field 'service_account_email' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#table: {
                        let field_value = match fields_map.get("table") {
                            Some(value) => value,
                            None => bail!("Missing field 'table' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#use_table_schema: {
                        let field_value = match fields_map.get("use_table_schema") {
                            Some(value) => value,
                            None => bail!("Missing field 'use_table_schema' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#use_topic_schema: {
                        let field_value = match fields_map.get("use_topic_schema") {
                            Some(value) => value,
                            None => bail!("Missing field 'use_topic_schema' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#write_metadata: {
                        let field_value = match fields_map.get("write_metadata") {
                            Some(value) => value,
                            None => bail!("Missing field 'write_metadata' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
