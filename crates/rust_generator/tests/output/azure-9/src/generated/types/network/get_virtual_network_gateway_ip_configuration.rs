#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetVirtualNetworkGatewayIpConfiguration {
    /// The resource ID of the IP configuration.
    #[builder(into)]
    pub r#id: String,
    /// Specifies the name of the Virtual Network Gateway.
    #[builder(into)]
    pub r#name: String,
    /// The Private IP Address associated with the Virtual Network Gateway.
    #[builder(into)]
    pub r#private_ip_address: String,
    /// Defines how the private IP address
    /// of the gateways virtual interface is assigned.
    #[builder(into)]
    pub r#private_ip_address_allocation: String,
    /// The ID of the Public IP Address associated
    /// with the Virtual Network Gateway.
    #[builder(into)]
    pub r#public_ip_address_id: String,
    /// The ID of the gateway subnet of a virtual network in
    /// which the virtual network gateway will be created. It is mandatory that
    /// the associated subnet is named `GatewaySubnet`. Therefore, each virtual
    /// network can contain at most a single Virtual Network Gateway.
    #[builder(into)]
    pub r#subnet_id: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetVirtualNetworkGatewayIpConfiguration {
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
                    "id",
                    &self.r#id,
                ),
                to_pulumi_object_field(
                    "name",
                    &self.r#name,
                ),
                to_pulumi_object_field(
                    "privateIpAddress",
                    &self.r#private_ip_address,
                ),
                to_pulumi_object_field(
                    "privateIpAddressAllocation",
                    &self.r#private_ip_address_allocation,
                ),
                to_pulumi_object_field(
                    "publicIpAddressId",
                    &self.r#public_ip_address_id,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetVirtualNetworkGatewayIpConfiguration {
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
                    r#id: {
                        let field_value = match fields_map.get("id") {
                            Some(value) => value,
                            None => bail!("Missing field 'id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#name: {
                        let field_value = match fields_map.get("name") {
                            Some(value) => value,
                            None => bail!("Missing field 'name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#private_ip_address: {
                        let field_value = match fields_map.get("privateIpAddress") {
                            Some(value) => value,
                            None => bail!("Missing field 'privateIpAddress' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#private_ip_address_allocation: {
                        let field_value = match fields_map.get("privateIpAddressAllocation") {
                            Some(value) => value,
                            None => bail!("Missing field 'privateIpAddressAllocation' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#public_ip_address_id: {
                        let field_value = match fields_map.get("publicIpAddressId") {
                            Some(value) => value,
                            None => bail!("Missing field 'publicIpAddressId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
