#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct HciLogicalNetworkSubnet {
    /// The address prefix in CIDR notation. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "addressPrefix")]
    pub r#address_prefix: Option<String>,
    /// The IP address allocation method for the subnet. Possible values are `Dynamic` and `Static`. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "ipAllocationMethod")]
    pub r#ip_allocation_method: String,
    /// One or more `ip_pool` block as defined above. Changing this forces a new resource to be created.
    /// 
    /// > **Note:** If `ip_pool` is not specified, it will be assigned by the server. If you experience a diff you may need to add this to `ignore_changes`.
    #[builder(into)]
    #[serde(rename = "ipPools")]
    pub r#ip_pools: Option<Vec<super::super::types::stack::HciLogicalNetworkSubnetIpPool>>,
    /// A `route` block as defined above. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "routes")]
    pub r#routes: Option<Vec<super::super::types::stack::HciLogicalNetworkSubnetRoute>>,
    /// The VLAN ID for the Logical Network. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "vlanId")]
    pub r#vlan_id: Option<i32>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for HciLogicalNetworkSubnet {
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
                    "address_prefix",
                    &self.r#address_prefix,
                ),
                to_pulumi_object_field(
                    "ip_allocation_method",
                    &self.r#ip_allocation_method,
                ),
                to_pulumi_object_field(
                    "ip_pools",
                    &self.r#ip_pools,
                ),
                to_pulumi_object_field(
                    "routes",
                    &self.r#routes,
                ),
                to_pulumi_object_field(
                    "vlan_id",
                    &self.r#vlan_id,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for HciLogicalNetworkSubnet {
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
                    r#address_prefix: {
                        let field_value = match fields_map.get("address_prefix") {
                            Some(value) => value,
                            None => bail!("Missing field 'address_prefix' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ip_allocation_method: {
                        let field_value = match fields_map.get("ip_allocation_method") {
                            Some(value) => value,
                            None => bail!("Missing field 'ip_allocation_method' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ip_pools: {
                        let field_value = match fields_map.get("ip_pools") {
                            Some(value) => value,
                            None => bail!("Missing field 'ip_pools' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#routes: {
                        let field_value = match fields_map.get("routes") {
                            Some(value) => value,
                            None => bail!("Missing field 'routes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#vlan_id: {
                        let field_value = match fields_map.get("vlan_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'vlan_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
