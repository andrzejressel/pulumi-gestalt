#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PoolNetworkConfiguration {
    /// Whether to enable accelerated networking. Possible values are `true` and `false`. Defaults to `false`. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "acceleratedNetworkingEnabled")]
    pub r#accelerated_networking_enabled: Option<bool>,
    /// The scope of dynamic vnet assignment. Allowed values: `none`, `job`. Changing this forces a new resource to be created. Defaults to `none`.
    #[builder(into)]
    #[serde(rename = "dynamicVnetAssignmentScope")]
    pub r#dynamic_vnet_assignment_scope: Option<String>,
    /// A list of `endpoint_configuration` blocks that can be used to address specific ports on an individual compute node externally as defined below. Set as documented in the inbound_nat_pools block below. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "endpointConfigurations")]
    pub r#endpoint_configurations: Option<Vec<super::super::types::batch::PoolNetworkConfigurationEndpointConfiguration>>,
    /// Type of public IP address provisioning. Supported values are `BatchManaged`, `UserManaged` and `NoPublicIPAddresses`.
    #[builder(into)]
    #[serde(rename = "publicAddressProvisioningType")]
    pub r#public_address_provisioning_type: Option<String>,
    /// A list of public IP ids that will be allocated to nodes. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "publicIps")]
    pub r#public_ips: Option<Vec<String>>,
    /// The ARM resource identifier of the virtual network subnet which the compute nodes of the pool will join. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "subnetId")]
    pub r#subnet_id: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for PoolNetworkConfiguration {
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
                    "acceleratedNetworkingEnabled",
                    &self.r#accelerated_networking_enabled,
                ),
                to_pulumi_object_field(
                    "dynamicVnetAssignmentScope",
                    &self.r#dynamic_vnet_assignment_scope,
                ),
                to_pulumi_object_field(
                    "endpointConfigurations",
                    &self.r#endpoint_configurations,
                ),
                to_pulumi_object_field(
                    "publicAddressProvisioningType",
                    &self.r#public_address_provisioning_type,
                ),
                to_pulumi_object_field(
                    "publicIps",
                    &self.r#public_ips,
                ),
                to_pulumi_object_field(
                    "subnetId",
                    &self.r#subnet_id,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for PoolNetworkConfiguration {
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
                    r#accelerated_networking_enabled: {
                        let field_value = match fields_map.get("acceleratedNetworkingEnabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'acceleratedNetworkingEnabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#dynamic_vnet_assignment_scope: {
                        let field_value = match fields_map.get("dynamicVnetAssignmentScope") {
                            Some(value) => value,
                            None => bail!("Missing field 'dynamicVnetAssignmentScope' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#endpoint_configurations: {
                        let field_value = match fields_map.get("endpointConfigurations") {
                            Some(value) => value,
                            None => bail!("Missing field 'endpointConfigurations' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#public_address_provisioning_type: {
                        let field_value = match fields_map.get("publicAddressProvisioningType") {
                            Some(value) => value,
                            None => bail!("Missing field 'publicAddressProvisioningType' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#public_ips: {
                        let field_value = match fields_map.get("publicIps") {
                            Some(value) => value,
                            None => bail!("Missing field 'publicIps' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#subnet_id: {
                        let field_value = match fields_map.get("subnetId") {
                            Some(value) => value,
                            None => bail!("Missing field 'subnetId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
