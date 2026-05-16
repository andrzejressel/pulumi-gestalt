#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct IngestionDestinationDestinationConfigurationAuditLogDestination {
    /// Contains information about an Amazon Data Firehose delivery stream.
    #[builder(into)]
    #[serde(rename = "firehoseStream")]
    pub r#firehose_stream: Option<Box<super::super::types::appfabric::IngestionDestinationDestinationConfigurationAuditLogDestinationFirehoseStream>>,
    /// Contains information about an Amazon S3 bucket.
    #[builder(into)]
    #[serde(rename = "s3Bucket")]
    pub r#s_3_bucket: Option<Box<super::super::types::appfabric::IngestionDestinationDestinationConfigurationAuditLogDestinationS3Bucket>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for IngestionDestinationDestinationConfigurationAuditLogDestination {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("firehose_stream".to_string(), self.r#firehose_stream.to_pulumi_value().await);
            map.insert("s_3_bucket".to_string(), self.r#s_3_bucket.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for IngestionDestinationDestinationConfigurationAuditLogDestination {
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
                    r#firehose_stream: {
                        let field_value = match fields_map.get("firehose_stream") {
                            Some(value) => value,
                            None => bail!("Missing field 'firehose_stream' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::appfabric::IngestionDestinationDestinationConfigurationAuditLogDestinationFirehoseStream>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#s_3_bucket: {
                        let field_value = match fields_map.get("s_3_bucket") {
                            Some(value) => value,
                            None => bail!("Missing field 's_3_bucket' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::appfabric::IngestionDestinationDestinationConfigurationAuditLogDestinationS3Bucket>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
