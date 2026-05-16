#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ConnectorKafkaClusterApacheKafkaCluster {
    /// The bootstrap servers of the cluster.
    #[builder(into)]
    #[serde(rename = "bootstrapServers")]
    pub r#bootstrap_servers: String,
    /// Details of an Amazon VPC which has network connectivity to the Apache Kafka cluster. See `vpc` Block for details.
    #[builder(into)]
    #[serde(rename = "vpc")]
    pub r#vpc: Box<super::super::types::mskconnect::ConnectorKafkaClusterApacheKafkaClusterVpc>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ConnectorKafkaClusterApacheKafkaCluster {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("bootstrap_servers".to_string(), self.r#bootstrap_servers.to_pulumi_value().await);
            map.insert("vpc".to_string(), self.r#vpc.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ConnectorKafkaClusterApacheKafkaCluster {
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
                    r#bootstrap_servers: {
                        let field_value = match fields_map.get("bootstrap_servers") {
                            Some(value) => value,
                            None => bail!("Missing field 'bootstrap_servers' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#vpc: {
                        let field_value = match fields_map.get("vpc") {
                            Some(value) => value,
                            None => bail!("Missing field 'vpc' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Box<super::super::types::mskconnect::ConnectorKafkaClusterApacheKafkaClusterVpc> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
