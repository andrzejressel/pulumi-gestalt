#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DataQualityJobDefinitionNetworkConfig {
    /// Whether to encrypt all communications between the instances used for the monitoring jobs. Choose `true` to encrypt communications. Encryption provides greater security for distributed jobs, but the processing might take longer.
    #[builder(into)]
    #[serde(rename = "enableInterContainerTrafficEncryption")]
    pub r#enable_inter_container_traffic_encryption: Option<bool>,
    /// Whether to allow inbound and outbound network calls to and from the containers used for the monitoring job.
    #[builder(into)]
    #[serde(rename = "enableNetworkIsolation")]
    pub r#enable_network_isolation: Option<bool>,
    /// Specifies a VPC that your training jobs and hosted models have access to. Control access to and from your training and model containers by configuring the VPC. Fields are documented below.
    #[builder(into)]
    #[serde(rename = "vpcConfig")]
    pub r#vpc_config: Option<Box<super::super::types::sagemaker::DataQualityJobDefinitionNetworkConfigVpcConfig>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for DataQualityJobDefinitionNetworkConfig {
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
                "enable_inter_container_traffic_encryption".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#enable_inter_container_traffic_encryption,
                )
                .await,
            );
            map.insert(
                "enable_network_isolation".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#enable_network_isolation,
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
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for DataQualityJobDefinitionNetworkConfig {
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
                    r#enable_inter_container_traffic_encryption: {
                        let field_value = match fields_map.get("enable_inter_container_traffic_encryption") {
                            Some(value) => value,
                            None => bail!("Missing field 'enable_inter_container_traffic_encryption' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#enable_network_isolation: {
                        let field_value = match fields_map.get("enable_network_isolation") {
                            Some(value) => value,
                            None => bail!("Missing field 'enable_network_isolation' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
