#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ReplicatorKafkaCluster {
    /// Details of an Amazon MSK cluster.
    #[builder(into)]
    #[serde(rename = "amazonMskCluster")]
    pub r#amazon_msk_cluster: Box<super::super::types::msk::ReplicatorKafkaClusterAmazonMskCluster>,
    /// Details of an Amazon VPC which has network connectivity to the Apache Kafka cluster.
    #[builder(into)]
    #[serde(rename = "vpcConfig")]
    pub r#vpc_config: Box<super::super::types::msk::ReplicatorKafkaClusterVpcConfig>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ReplicatorKafkaCluster {
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
                "amazon_msk_cluster".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#amazon_msk_cluster,
                )
                .await,
            );
            map.insert(
                "vpc_config".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#vpc_config,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ReplicatorKafkaCluster {
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
                    r#amazon_msk_cluster: {
                        let field_value = match fields_map.get("amazon_msk_cluster") {
                            Some(value) => value,
                            None => bail!("Missing field 'amazon_msk_cluster' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#vpc_config: {
                        let field_value = match fields_map.get("vpc_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'vpc_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
