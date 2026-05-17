#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetVirtualNetworkSubnetOverride {
    /// The name of the subnet.
    #[builder(into)]
    #[serde(rename = "labSubnetName")]
    pub r#lab_subnet_name: String,
    /// The resource identifier for the subnet.
    #[builder(into)]
    #[serde(rename = "resourceId")]
    pub r#resource_id: String,
    /// Indicates if the subnet can be used for VM creation.  Possible values are `Allow`, `Default` and `Deny`.
    #[builder(into)]
    #[serde(rename = "useInVmCreationPermission")]
    pub r#use_in_vm_creation_permission: String,
    #[builder(into)]
    #[serde(rename = "usePublicIpAddressPermission")]
    pub r#use_public_ip_address_permission: String,
    /// The virtual network pool associated with this subnet.
    #[builder(into)]
    #[serde(rename = "virtualNetworkPoolName")]
    pub r#virtual_network_pool_name: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetVirtualNetworkSubnetOverride {
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
                "lab_subnet_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#lab_subnet_name,
                )
                .await,
            );
            map.insert(
                "resource_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#resource_id,
                )
                .await,
            );
            map.insert(
                "use_in_vm_creation_permission".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#use_in_vm_creation_permission,
                )
                .await,
            );
            map.insert(
                "use_public_ip_address_permission".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#use_public_ip_address_permission,
                )
                .await,
            );
            map.insert(
                "virtual_network_pool_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#virtual_network_pool_name,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetVirtualNetworkSubnetOverride {
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
                    r#lab_subnet_name: {
                        let field_value = match fields_map.get("lab_subnet_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'lab_subnet_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#resource_id: {
                        let field_value = match fields_map.get("resource_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'resource_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#use_in_vm_creation_permission: {
                        let field_value = match fields_map.get("use_in_vm_creation_permission") {
                            Some(value) => value,
                            None => bail!("Missing field 'use_in_vm_creation_permission' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#use_public_ip_address_permission: {
                        let field_value = match fields_map.get("use_public_ip_address_permission") {
                            Some(value) => value,
                            None => bail!("Missing field 'use_public_ip_address_permission' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#virtual_network_pool_name: {
                        let field_value = match fields_map.get("virtual_network_pool_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'virtual_network_pool_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
