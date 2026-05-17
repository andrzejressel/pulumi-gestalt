#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ClusterVpcConfig {
    /// Cluster security group that was created by Amazon EKS for the cluster. Managed node groups use this security group for control-plane-to-data-plane communication.
    #[builder(into)]
    #[serde(rename = "clusterSecurityGroupId")]
    pub r#cluster_security_group_id: Option<String>,
    /// Whether the Amazon EKS private API server endpoint is enabled. Default is `false`.
    #[builder(into)]
    #[serde(rename = "endpointPrivateAccess")]
    pub r#endpoint_private_access: Option<bool>,
    /// Whether the Amazon EKS public API server endpoint is enabled. Default is `true`.
    #[builder(into)]
    #[serde(rename = "endpointPublicAccess")]
    pub r#endpoint_public_access: Option<bool>,
    /// List of CIDR blocks. Indicates which CIDR blocks can access the Amazon EKS public API server endpoint when enabled. EKS defaults this to a list with `0.0.0.0/0`. The provider will only perform drift detection of its value when present in a configuration.
    #[builder(into)]
    #[serde(rename = "publicAccessCidrs")]
    pub r#public_access_cidrs: Option<Vec<String>>,
    /// List of security group IDs for the cross-account elastic network interfaces that Amazon EKS creates to use to allow communication between your worker nodes and the Kubernetes control plane.
    #[builder(into)]
    #[serde(rename = "securityGroupIds")]
    pub r#security_group_ids: Option<Vec<String>>,
    /// List of subnet IDs. Must be in at least two different availability zones. Amazon EKS creates cross-account elastic network interfaces in these subnets to allow communication between your worker nodes and the Kubernetes control plane.
    #[builder(into)]
    #[serde(rename = "subnetIds")]
    pub r#subnet_ids: Vec<String>,
    /// ID of the VPC associated with your cluster.
    #[builder(into)]
    #[serde(rename = "vpcId")]
    pub r#vpc_id: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ClusterVpcConfig {
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
                "cluster_security_group_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#cluster_security_group_id,
                )
                .await,
            );
            map.insert(
                "endpoint_private_access".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#endpoint_private_access,
                )
                .await,
            );
            map.insert(
                "endpoint_public_access".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#endpoint_public_access,
                )
                .await,
            );
            map.insert(
                "public_access_cidrs".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#public_access_cidrs,
                )
                .await,
            );
            map.insert(
                "security_group_ids".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#security_group_ids,
                )
                .await,
            );
            map.insert(
                "subnet_ids".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#subnet_ids,
                )
                .await,
            );
            map.insert(
                "vpc_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#vpc_id,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ClusterVpcConfig {
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
                    r#cluster_security_group_id: {
                        let field_value = match fields_map.get("cluster_security_group_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'cluster_security_group_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#endpoint_private_access: {
                        let field_value = match fields_map.get("endpoint_private_access") {
                            Some(value) => value,
                            None => bail!("Missing field 'endpoint_private_access' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#endpoint_public_access: {
                        let field_value = match fields_map.get("endpoint_public_access") {
                            Some(value) => value,
                            None => bail!("Missing field 'endpoint_public_access' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#public_access_cidrs: {
                        let field_value = match fields_map.get("public_access_cidrs") {
                            Some(value) => value,
                            None => bail!("Missing field 'public_access_cidrs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#security_group_ids: {
                        let field_value = match fields_map.get("security_group_ids") {
                            Some(value) => value,
                            None => bail!("Missing field 'security_group_ids' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#subnet_ids: {
                        let field_value = match fields_map.get("subnet_ids") {
                            Some(value) => value,
                            None => bail!("Missing field 'subnet_ids' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#vpc_id: {
                        let field_value = match fields_map.get("vpc_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'vpc_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
