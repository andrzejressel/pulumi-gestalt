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
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "address_prefix".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#address_prefix,
                )
                .await,
            );
            map.insert(
                "ip_allocation_method".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#ip_allocation_method,
                )
                .await,
            );
            map.insert(
                "ip_pools".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#ip_pools,
                )
                .await,
            );
            map.insert(
                "routes".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#routes,
                )
                .await,
            );
            map.insert(
                "vlan_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#vlan_id,
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
