#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct EnvironmentConfigPrivateEnvironmentConfig {
    /// When specified, the environment will use Private Service Connect instead of VPC peerings to connect to Cloud SQL in the Tenant Project, and the PSC endpoint in the Customer Project will use an IP address from this subnetwork. This field is supported for Cloud Composer environments in versions composer-2.*.*-airflow-*.*.* and newer.
    #[builder(into)]
    pub r#cloud_composer_connection_subnetwork: Option<String>,
    /// The CIDR block from which IP range for Cloud Composer Network in tenant project will be reserved. Needs to be disjoint from private_cluster_config.master_ipv4_cidr_block and cloud_sql_ipv4_cidr_block. This field is supported for Cloud Composer environments in versions composer-2.*.*-airflow-*.*.* and newer.
    #[builder(into)]
    pub r#cloud_composer_network_ipv_4_cidr_block: Option<String>,
    /// The CIDR block from which IP range in tenant project will be reserved for Cloud SQL. Needs to be disjoint from web_server_ipv4_cidr_block.
    #[builder(into)]
    pub r#cloud_sql_ipv_4_cidr_block: Option<String>,
    /// Mode of internal communication within the Composer environment. Must be one of "VPC_PEERING" or "PRIVATE_SERVICE_CONNECT".
    #[builder(into)]
    pub r#connection_type: Option<String>,
    /// If true, access to the public endpoint of the GKE cluster is denied. If this field is set to true, ip_allocation_policy.use_ip_aliases must be set to true for Cloud Composer environments in versions composer-1.*.*-airflow-*.*.*.
    #[builder(into)]
    pub r#enable_private_endpoint: Option<bool>,
    /// When enabled, IPs from public (non-RFC1918) ranges can be used for ip_allocation_policy.cluster_ipv4_cidr_block and ip_allocation_policy.service_ipv4_cidr_block.
    #[builder(into)]
    pub r#enable_privately_used_public_ips: Option<bool>,
    /// The IP range in CIDR notation to use for the hosted master network. This range is used for assigning internal IP addresses to the cluster master or set of masters and to the internal load balancer virtual IP. This range must not overlap with any other ranges in use within the cluster's network. If left blank, the default value of '172.16.0.0/28' is used.
    #[builder(into)]
    pub r#master_ipv_4_cidr_block: Option<String>,
    /// The CIDR block from which IP range for web server will be reserved. Needs to be disjoint from master_ipv4_cidr_block and cloud_sql_ipv4_cidr_block. This field is supported for Cloud Composer environments in versions composer-1.*.*-airflow-*.*.*.
    #[builder(into)]
    pub r#web_server_ipv_4_cidr_block: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for EnvironmentConfigPrivateEnvironmentConfig {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > + Send {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "cloudComposerConnectionSubnetwork",
                    &self.r#cloud_composer_connection_subnetwork,
                ),
                to_pulumi_object_field(
                    "cloudComposerNetworkIpv4CidrBlock",
                    &self.r#cloud_composer_network_ipv_4_cidr_block,
                ),
                to_pulumi_object_field(
                    "cloudSqlIpv4CidrBlock",
                    &self.r#cloud_sql_ipv_4_cidr_block,
                ),
                to_pulumi_object_field(
                    "connectionType",
                    &self.r#connection_type,
                ),
                to_pulumi_object_field(
                    "enablePrivateEndpoint",
                    &self.r#enable_private_endpoint,
                ),
                to_pulumi_object_field(
                    "enablePrivatelyUsedPublicIps",
                    &self.r#enable_privately_used_public_ips,
                ),
                to_pulumi_object_field(
                    "masterIpv4CidrBlock",
                    &self.r#master_ipv_4_cidr_block,
                ),
                to_pulumi_object_field(
                    "webServerIpv4CidrBlock",
                    &self.r#web_server_ipv_4_cidr_block,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for EnvironmentConfigPrivateEnvironmentConfig {
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
                    r#cloud_composer_connection_subnetwork: {
                        let field_value = match fields_map.get("cloudComposerConnectionSubnetwork") {
                            Some(value) => value,
                            None => bail!("Missing field 'cloudComposerConnectionSubnetwork' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cloud_composer_network_ipv_4_cidr_block: {
                        let field_value = match fields_map.get("cloudComposerNetworkIpv4CidrBlock") {
                            Some(value) => value,
                            None => bail!("Missing field 'cloudComposerNetworkIpv4CidrBlock' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cloud_sql_ipv_4_cidr_block: {
                        let field_value = match fields_map.get("cloudSqlIpv4CidrBlock") {
                            Some(value) => value,
                            None => bail!("Missing field 'cloudSqlIpv4CidrBlock' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#connection_type: {
                        let field_value = match fields_map.get("connectionType") {
                            Some(value) => value,
                            None => bail!("Missing field 'connectionType' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#enable_private_endpoint: {
                        let field_value = match fields_map.get("enablePrivateEndpoint") {
                            Some(value) => value,
                            None => bail!("Missing field 'enablePrivateEndpoint' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#enable_privately_used_public_ips: {
                        let field_value = match fields_map.get("enablePrivatelyUsedPublicIps") {
                            Some(value) => value,
                            None => bail!("Missing field 'enablePrivatelyUsedPublicIps' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#master_ipv_4_cidr_block: {
                        let field_value = match fields_map.get("masterIpv4CidrBlock") {
                            Some(value) => value,
                            None => bail!("Missing field 'masterIpv4CidrBlock' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#web_server_ipv_4_cidr_block: {
                        let field_value = match fields_map.get("webServerIpv4CidrBlock") {
                            Some(value) => value,
                            None => bail!("Missing field 'webServerIpv4CidrBlock' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
