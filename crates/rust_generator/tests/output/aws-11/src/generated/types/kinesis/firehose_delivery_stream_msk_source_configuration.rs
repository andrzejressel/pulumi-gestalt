#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FirehoseDeliveryStreamMskSourceConfiguration {
    /// The authentication configuration of the Amazon MSK cluster. See `authentication_configuration` block below for details.
    #[builder(into)]
    #[serde(rename = "authenticationConfiguration")]
    pub r#authentication_configuration: Box<super::super::types::kinesis::FirehoseDeliveryStreamMskSourceConfigurationAuthenticationConfiguration>,
    /// The ARN of the Amazon MSK cluster.
    #[builder(into)]
    #[serde(rename = "mskClusterArn")]
    pub r#msk_cluster_arn: String,
    /// The topic name within the Amazon MSK cluster.
    #[builder(into)]
    #[serde(rename = "topicName")]
    pub r#topic_name: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for FirehoseDeliveryStreamMskSourceConfiguration {
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
                "authentication_configuration".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#authentication_configuration,
                )
                .await,
            );
            map.insert(
                "msk_cluster_arn".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#msk_cluster_arn,
                )
                .await,
            );
            map.insert(
                "topic_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#topic_name,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for FirehoseDeliveryStreamMskSourceConfiguration {
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
                    r#authentication_configuration: {
                        let field_value = match fields_map.get("authentication_configuration") {
                            Some(value) => value,
                            None => bail!("Missing field 'authentication_configuration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#msk_cluster_arn: {
                        let field_value = match fields_map.get("msk_cluster_arn") {
                            Some(value) => value,
                            None => bail!("Missing field 'msk_cluster_arn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#topic_name: {
                        let field_value = match fields_map.get("topic_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'topic_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
