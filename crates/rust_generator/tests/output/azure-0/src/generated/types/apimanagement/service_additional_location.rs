#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ServiceAdditionalLocation {
    /// The number of compute units in this region. Defaults to the capacity of the main region.
    #[builder(into)]
    #[serde(rename = "capacity")]
    pub r#capacity: Option<i32>,
    /// Only valid for an Api Management service deployed in multiple locations. This can be used to disable the gateway in this additional location.
    #[builder(into)]
    #[serde(rename = "gatewayDisabled")]
    pub r#gateway_disabled: Option<bool>,
    /// The URL of the Regional Gateway for the API Management Service in the specified region.
    #[builder(into)]
    #[serde(rename = "gatewayRegionalUrl")]
    pub r#gateway_regional_url: Option<String>,
    /// The name of the Azure Region in which the API Management Service should be expanded to.
    #[builder(into)]
    #[serde(rename = "location")]
    pub r#location: String,
    /// The Private IP addresses of the API Management Service. Available only when the API Manager instance is using Virtual Network mode.
    #[builder(into)]
    #[serde(rename = "privateIpAddresses")]
    pub r#private_ip_addresses: Option<Vec<String>>,
    /// ID of a standard SKU IPv4 Public IP.
    /// 
    /// > **NOTE:** Availability zones and custom public IPs are only supported in the Premium tier.
    #[builder(into)]
    #[serde(rename = "publicIpAddressId")]
    pub r#public_ip_address_id: Option<String>,
    /// Public Static Load Balanced IP addresses of the API Management service in the additional location. Available only for Basic, Standard and Premium SKU.
    #[builder(into)]
    #[serde(rename = "publicIpAddresses")]
    pub r#public_ip_addresses: Option<Vec<String>>,
    /// A `virtual_network_configuration` block as defined below. Required when `virtual_network_type` is `External` or `Internal`.
    #[builder(into)]
    #[serde(rename = "virtualNetworkConfiguration")]
    pub r#virtual_network_configuration: Option<Box<super::super::types::apimanagement::ServiceAdditionalLocationVirtualNetworkConfiguration>>,
    /// A list of availability zones.
    #[builder(into)]
    #[serde(rename = "zones")]
    pub r#zones: Option<Vec<String>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ServiceAdditionalLocation {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_concurrent, to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "capacity",
                    &self.r#capacity,
                ),
                to_pulumi_object_field(
                    "gateway_disabled",
                    &self.r#gateway_disabled,
                ),
                to_pulumi_object_field(
                    "gateway_regional_url",
                    &self.r#gateway_regional_url,
                ),
                to_pulumi_object_field(
                    "location",
                    &self.r#location,
                ),
                to_pulumi_object_field(
                    "private_ip_addresses",
                    &self.r#private_ip_addresses,
                ),
                to_pulumi_object_field(
                    "public_ip_address_id",
                    &self.r#public_ip_address_id,
                ),
                to_pulumi_object_field(
                    "public_ip_addresses",
                    &self.r#public_ip_addresses,
                ),
                to_pulumi_object_field(
                    "virtual_network_configuration",
                    &self.r#virtual_network_configuration,
                ),
                to_pulumi_object_field(
                    "zones",
                    &self.r#zones,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ServiceAdditionalLocation {
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
                    r#capacity: {
                        let field_value = match fields_map.get("capacity") {
                            Some(value) => value,
                            None => bail!("Missing field 'capacity' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#gateway_disabled: {
                        let field_value = match fields_map.get("gateway_disabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'gateway_disabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#gateway_regional_url: {
                        let field_value = match fields_map.get("gateway_regional_url") {
                            Some(value) => value,
                            None => bail!("Missing field 'gateway_regional_url' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#location: {
                        let field_value = match fields_map.get("location") {
                            Some(value) => value,
                            None => bail!("Missing field 'location' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#private_ip_addresses: {
                        let field_value = match fields_map.get("private_ip_addresses") {
                            Some(value) => value,
                            None => bail!("Missing field 'private_ip_addresses' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#public_ip_address_id: {
                        let field_value = match fields_map.get("public_ip_address_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'public_ip_address_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#public_ip_addresses: {
                        let field_value = match fields_map.get("public_ip_addresses") {
                            Some(value) => value,
                            None => bail!("Missing field 'public_ip_addresses' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#virtual_network_configuration: {
                        let field_value = match fields_map.get("virtual_network_configuration") {
                            Some(value) => value,
                            None => bail!("Missing field 'virtual_network_configuration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#zones: {
                        let field_value = match fields_map.get("zones") {
                            Some(value) => value,
                            None => bail!("Missing field 'zones' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
