#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ClusterBrokerNodeGroupInfo {
    /// The distribution of broker nodes across availability zones ([documentation](https://docs.aws.amazon.com/msk/1.0/apireference/clusters.html#clusters-model-brokerazdistribution)). Currently the only valid value is `DEFAULT`.
    #[builder(into)]
    #[serde(rename = "azDistribution")]
    pub r#az_distribution: Option<String>,
    /// A list of subnets to connect to in client VPC ([documentation](https://docs.aws.amazon.com/msk/1.0/apireference/clusters.html#clusters-prop-brokernodegroupinfo-clientsubnets)).
    #[builder(into)]
    #[serde(rename = "clientSubnets")]
    pub r#client_subnets: Vec<String>,
    /// Information about the cluster access configuration. See below. For security reasons, you can't turn on public access while creating an MSK cluster. However, you can update an existing cluster to make it publicly accessible. You can also create a new cluster and then update it to make it publicly accessible ([documentation](https://docs.aws.amazon.com/msk/latest/developerguide/public-access.html)).
    #[builder(into)]
    #[serde(rename = "connectivityInfo")]
    pub r#connectivity_info: Option<Box<super::super::types::msk::ClusterBrokerNodeGroupInfoConnectivityInfo>>,
    /// Specify the instance type to use for the kafka brokersE.g., kafka.m5.large. ([Pricing info](https://aws.amazon.com/msk/pricing/))
    #[builder(into)]
    #[serde(rename = "instanceType")]
    pub r#instance_type: String,
    /// A list of the security groups to associate with the elastic network interfaces to control who can communicate with the cluster.
    #[builder(into)]
    #[serde(rename = "securityGroups")]
    pub r#security_groups: Vec<String>,
    /// A block that contains information about storage volumes attached to MSK broker nodes. See below.
    #[builder(into)]
    #[serde(rename = "storageInfo")]
    pub r#storage_info: Option<Box<super::super::types::msk::ClusterBrokerNodeGroupInfoStorageInfo>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ClusterBrokerNodeGroupInfo {
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
                "az_distribution".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#az_distribution,
                )
                .await,
            );
            map.insert(
                "client_subnets".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#client_subnets,
                )
                .await,
            );
            map.insert(
                "connectivity_info".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#connectivity_info,
                )
                .await,
            );
            map.insert(
                "instance_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#instance_type,
                )
                .await,
            );
            map.insert(
                "security_groups".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#security_groups,
                )
                .await,
            );
            map.insert(
                "storage_info".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#storage_info,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ClusterBrokerNodeGroupInfo {
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
                    r#az_distribution: {
                        let field_value = match fields_map.get("az_distribution") {
                            Some(value) => value,
                            None => bail!("Missing field 'az_distribution' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#client_subnets: {
                        let field_value = match fields_map.get("client_subnets") {
                            Some(value) => value,
                            None => bail!("Missing field 'client_subnets' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#connectivity_info: {
                        let field_value = match fields_map.get("connectivity_info") {
                            Some(value) => value,
                            None => bail!("Missing field 'connectivity_info' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#instance_type: {
                        let field_value = match fields_map.get("instance_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'instance_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#security_groups: {
                        let field_value = match fields_map.get("security_groups") {
                            Some(value) => value,
                            None => bail!("Missing field 'security_groups' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#storage_info: {
                        let field_value = match fields_map.get("storage_info") {
                            Some(value) => value,
                            None => bail!("Missing field 'storage_info' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
