#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct WorkspaceCustomParameters {
    /// The ID of a Azure Machine Learning workspace to link with Databricks workspace. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "machineLearningWorkspaceId")]
    pub r#machine_learning_workspace_id: Option<String>,
    /// Name of the NAT gateway for Secure Cluster Connectivity (No Public IP) workspace subnets (only for workspace with managed virtual network). Defaults to `nat-gateway`. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "natGatewayName")]
    pub r#nat_gateway_name: Option<String>,
    /// Are public IP Addresses not allowed? Possible values are `true` or `false`. Defaults to `true`.
    /// 
    /// > **Note:** Updating `no_public_ip` parameter is only allowed if the value is changing from `false` to `true` and only for VNet-injected workspaces.
    /// 
    /// > **Note:** In `v3.104.0` and higher of the provider the `no_public_ip` parameter will now default to `true` instead of `false`.
    #[builder(into)]
    #[serde(rename = "noPublicIp")]
    pub r#no_public_ip: Option<bool>,
    /// The name of the Private Subnet within the Virtual Network. Required if `virtual_network_id` is set. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "privateSubnetName")]
    pub r#private_subnet_name: Option<String>,
    /// The resource ID of the `azure.network.SubnetNetworkSecurityGroupAssociation` resource which is referred to by the `private_subnet_name` field. This is the same as the ID of the subnet referred to by the `private_subnet_name` field. Required if `virtual_network_id` is set.
    #[builder(into)]
    #[serde(rename = "privateSubnetNetworkSecurityGroupAssociationId")]
    pub r#private_subnet_network_security_group_association_id: Option<String>,
    /// Name of the Public IP for No Public IP workspace with managed virtual network. Defaults to `nat-gw-public-ip`. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "publicIpName")]
    pub r#public_ip_name: Option<String>,
    /// The name of the Public Subnet within the Virtual Network. Required if `virtual_network_id` is set. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "publicSubnetName")]
    pub r#public_subnet_name: Option<String>,
    /// The resource ID of the `azure.network.SubnetNetworkSecurityGroupAssociation` resource which is referred to by the `public_subnet_name` field. This is the same as the ID of the subnet referred to by the `public_subnet_name` field. Required if `virtual_network_id` is set.
    #[builder(into)]
    #[serde(rename = "publicSubnetNetworkSecurityGroupAssociationId")]
    pub r#public_subnet_network_security_group_association_id: Option<String>,
    /// Default Databricks File Storage account name. Defaults to a randomized name(e.g. `dbstoragel6mfeghoe5kxu`). Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "storageAccountName")]
    pub r#storage_account_name: Option<String>,
    /// Storage account SKU name. Possible values include `Standard_LRS`, `Standard_GRS`, `Standard_RAGRS`, `Standard_GZRS`, `Standard_RAGZRS`, `Standard_ZRS`, `Premium_LRS` or `Premium_ZRS`. Defaults to `Standard_GRS`.
    #[builder(into)]
    #[serde(rename = "storageAccountSkuName")]
    pub r#storage_account_sku_name: Option<String>,
    /// The ID of a Virtual Network where this Databricks Cluster should be created. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "virtualNetworkId")]
    pub r#virtual_network_id: Option<String>,
    /// Address prefix for Managed virtual network. Defaults to `10.139`. Changing this forces a new resource to be created.
    /// 
    /// > **Note:** Databricks requires that a network security group is associated with the `public` and `private` subnets when a `virtual_network_id` has been defined. Both `public` and `private` subnets must be delegated to `Microsoft.Databricks/workspaces`. For more information about subnet delegation see the [product documentation](https://docs.microsoft.com/azure/virtual-network/subnet-delegation-overview).
    #[builder(into)]
    #[serde(rename = "vnetAddressPrefix")]
    pub r#vnet_address_prefix: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for WorkspaceCustomParameters {
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
                    "machine_learning_workspace_id",
                    &self.r#machine_learning_workspace_id,
                ),
                to_pulumi_object_field(
                    "nat_gateway_name",
                    &self.r#nat_gateway_name,
                ),
                to_pulumi_object_field(
                    "no_public_ip",
                    &self.r#no_public_ip,
                ),
                to_pulumi_object_field(
                    "private_subnet_name",
                    &self.r#private_subnet_name,
                ),
                to_pulumi_object_field(
                    "private_subnet_network_security_group_association_id",
                    &self.r#private_subnet_network_security_group_association_id,
                ),
                to_pulumi_object_field(
                    "public_ip_name",
                    &self.r#public_ip_name,
                ),
                to_pulumi_object_field(
                    "public_subnet_name",
                    &self.r#public_subnet_name,
                ),
                to_pulumi_object_field(
                    "public_subnet_network_security_group_association_id",
                    &self.r#public_subnet_network_security_group_association_id,
                ),
                to_pulumi_object_field(
                    "storage_account_name",
                    &self.r#storage_account_name,
                ),
                to_pulumi_object_field(
                    "storage_account_sku_name",
                    &self.r#storage_account_sku_name,
                ),
                to_pulumi_object_field(
                    "virtual_network_id",
                    &self.r#virtual_network_id,
                ),
                to_pulumi_object_field(
                    "vnet_address_prefix",
                    &self.r#vnet_address_prefix,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for WorkspaceCustomParameters {
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
                    r#machine_learning_workspace_id: {
                        let field_value = match fields_map.get("machine_learning_workspace_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'machine_learning_workspace_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#nat_gateway_name: {
                        let field_value = match fields_map.get("nat_gateway_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'nat_gateway_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#no_public_ip: {
                        let field_value = match fields_map.get("no_public_ip") {
                            Some(value) => value,
                            None => bail!("Missing field 'no_public_ip' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#private_subnet_name: {
                        let field_value = match fields_map.get("private_subnet_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'private_subnet_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#private_subnet_network_security_group_association_id: {
                        let field_value = match fields_map.get("private_subnet_network_security_group_association_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'private_subnet_network_security_group_association_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#public_ip_name: {
                        let field_value = match fields_map.get("public_ip_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'public_ip_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#public_subnet_name: {
                        let field_value = match fields_map.get("public_subnet_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'public_subnet_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#public_subnet_network_security_group_association_id: {
                        let field_value = match fields_map.get("public_subnet_network_security_group_association_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'public_subnet_network_security_group_association_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#storage_account_name: {
                        let field_value = match fields_map.get("storage_account_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'storage_account_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#storage_account_sku_name: {
                        let field_value = match fields_map.get("storage_account_sku_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'storage_account_sku_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#virtual_network_id: {
                        let field_value = match fields_map.get("virtual_network_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'virtual_network_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#vnet_address_prefix: {
                        let field_value = match fields_map.get("vnet_address_prefix") {
                            Some(value) => value,
                            None => bail!("Missing field 'vnet_address_prefix' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
