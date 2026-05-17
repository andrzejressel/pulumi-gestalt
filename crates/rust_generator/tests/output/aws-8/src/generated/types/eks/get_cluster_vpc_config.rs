#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetClusterVpcConfig {
    /// The cluster security group that was created by Amazon EKS for the cluster.
    #[builder(into)]
    #[serde(rename = "clusterSecurityGroupId")]
    pub r#cluster_security_group_id: String,
    /// Indicates whether or not the Amazon EKS private API server endpoint is enabled.
    #[builder(into)]
    #[serde(rename = "endpointPrivateAccess")]
    pub r#endpoint_private_access: bool,
    /// Indicates whether or not the Amazon EKS public API server endpoint is enabled.
    #[builder(into)]
    #[serde(rename = "endpointPublicAccess")]
    pub r#endpoint_public_access: bool,
    /// List of CIDR blocks. Indicates which CIDR blocks can access the Amazon EKS public API server endpoint.
    #[builder(into)]
    #[serde(rename = "publicAccessCidrs")]
    pub r#public_access_cidrs: Vec<String>,
    /// List of security group IDs
    #[builder(into)]
    #[serde(rename = "securityGroupIds")]
    pub r#security_group_ids: Vec<String>,
    /// List of subnet IDs
    #[builder(into)]
    #[serde(rename = "subnetIds")]
    pub r#subnet_ids: Vec<String>,
    /// The VPC associated with your cluster.
    #[builder(into)]
    #[serde(rename = "vpcId")]
    pub r#vpc_id: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetClusterVpcConfig {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "cluster_security_group_id",
                    &self.r#cluster_security_group_id,
                ),
                to_pulumi_object_field(
                    "endpoint_private_access",
                    &self.r#endpoint_private_access,
                ),
                to_pulumi_object_field(
                    "endpoint_public_access",
                    &self.r#endpoint_public_access,
                ),
                to_pulumi_object_field(
                    "public_access_cidrs",
                    &self.r#public_access_cidrs,
                ),
                to_pulumi_object_field(
                    "security_group_ids",
                    &self.r#security_group_ids,
                ),
                to_pulumi_object_field(
                    "subnet_ids",
                    &self.r#subnet_ids,
                ),
                to_pulumi_object_field(
                    "vpc_id",
                    &self.r#vpc_id,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetClusterVpcConfig {
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
