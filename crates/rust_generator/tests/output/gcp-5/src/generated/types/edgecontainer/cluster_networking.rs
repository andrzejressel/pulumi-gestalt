#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ClusterNetworking {
    /// All pods in the cluster are assigned an RFC1918 IPv4 address from these
    /// blocks. Only a single block is supported. This field cannot be changed
    /// after creation.
    #[builder(into)]
    pub r#cluster_ipv_4_cidr_blocks: Vec<String>,
    /// If specified, dual stack mode is enabled and all pods in the cluster are
    /// assigned an IPv6 address from these blocks alongside from an IPv4
    /// address. Only a single block is supported. This field cannot be changed
    /// after creation.
    #[builder(into)]
    pub r#cluster_ipv_6_cidr_blocks: Option<Vec<String>>,
    /// (Output)
    /// IP addressing type of this cluster i.e. SINGLESTACK_V4 vs DUALSTACK_V4_V6.
    #[builder(into)]
    pub r#network_type: Option<String>,
    /// All services in the cluster are assigned an RFC1918 IPv4 address from these
    /// blocks. Only a single block is supported. This field cannot be changed
    /// after creation.
    #[builder(into)]
    pub r#services_ipv_4_cidr_blocks: Vec<String>,
    /// If specified, dual stack mode is enabled and all services in the cluster are
    /// assigned an IPv6 address from these blocks alongside from an IPv4
    /// address. Only a single block is supported. This field cannot be changed
    /// after creation.
    #[builder(into)]
    pub r#services_ipv_6_cidr_blocks: Option<Vec<String>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ClusterNetworking {
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
                    "clusterIpv4CidrBlocks",
                    &self.r#cluster_ipv_4_cidr_blocks,
                ),
                to_pulumi_object_field(
                    "clusterIpv6CidrBlocks",
                    &self.r#cluster_ipv_6_cidr_blocks,
                ),
                to_pulumi_object_field(
                    "networkType",
                    &self.r#network_type,
                ),
                to_pulumi_object_field(
                    "servicesIpv4CidrBlocks",
                    &self.r#services_ipv_4_cidr_blocks,
                ),
                to_pulumi_object_field(
                    "servicesIpv6CidrBlocks",
                    &self.r#services_ipv_6_cidr_blocks,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ClusterNetworking {
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
                    r#cluster_ipv_4_cidr_blocks: {
                        let field_value = match fields_map.get("clusterIpv4CidrBlocks") {
                            Some(value) => value,
                            None => bail!("Missing field 'clusterIpv4CidrBlocks' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cluster_ipv_6_cidr_blocks: {
                        let field_value = match fields_map.get("clusterIpv6CidrBlocks") {
                            Some(value) => value,
                            None => bail!("Missing field 'clusterIpv6CidrBlocks' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#network_type: {
                        let field_value = match fields_map.get("networkType") {
                            Some(value) => value,
                            None => bail!("Missing field 'networkType' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#services_ipv_4_cidr_blocks: {
                        let field_value = match fields_map.get("servicesIpv4CidrBlocks") {
                            Some(value) => value,
                            None => bail!("Missing field 'servicesIpv4CidrBlocks' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#services_ipv_6_cidr_blocks: {
                        let field_value = match fields_map.get("servicesIpv6CidrBlocks") {
                            Some(value) => value,
                            None => bail!("Missing field 'servicesIpv6CidrBlocks' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
