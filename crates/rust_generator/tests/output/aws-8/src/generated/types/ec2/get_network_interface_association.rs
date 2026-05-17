#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetNetworkInterfaceAssociation {
    /// Allocation ID.
    #[builder(into)]
    #[serde(rename = "allocationId")]
    pub r#allocation_id: String,
    /// Association ID.
    #[builder(into)]
    #[serde(rename = "associationId")]
    pub r#association_id: String,
    /// Carrier IP address associated with the network interface. This attribute is only set when the network interface is in a subnet which is associated with a Wavelength Zone.
    #[builder(into)]
    #[serde(rename = "carrierIp")]
    pub r#carrier_ip: String,
    /// Customer-owned IP address.
    #[builder(into)]
    #[serde(rename = "customerOwnedIp")]
    pub r#customer_owned_ip: String,
    /// ID of the Elastic IP address owner.
    #[builder(into)]
    #[serde(rename = "ipOwnerId")]
    pub r#ip_owner_id: String,
    /// Public DNS name.
    #[builder(into)]
    #[serde(rename = "publicDnsName")]
    pub r#public_dns_name: String,
    /// Address of the Elastic IP address bound to the network interface.
    #[builder(into)]
    #[serde(rename = "publicIp")]
    pub r#public_ip: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetNetworkInterfaceAssociation {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "allocation_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#allocation_id,
                )
                .await,
            );
            map.insert(
                "association_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#association_id,
                )
                .await,
            );
            map.insert(
                "carrier_ip".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#carrier_ip,
                )
                .await,
            );
            map.insert(
                "customer_owned_ip".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#customer_owned_ip,
                )
                .await,
            );
            map.insert(
                "ip_owner_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#ip_owner_id,
                )
                .await,
            );
            map.insert(
                "public_dns_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#public_dns_name,
                )
                .await,
            );
            map.insert(
                "public_ip".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#public_ip,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetNetworkInterfaceAssociation {
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
                    r#allocation_id: {
                        let field_value = match fields_map.get("allocation_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'allocation_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#association_id: {
                        let field_value = match fields_map.get("association_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'association_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#carrier_ip: {
                        let field_value = match fields_map.get("carrier_ip") {
                            Some(value) => value,
                            None => bail!("Missing field 'carrier_ip' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#customer_owned_ip: {
                        let field_value = match fields_map.get("customer_owned_ip") {
                            Some(value) => value,
                            None => bail!("Missing field 'customer_owned_ip' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ip_owner_id: {
                        let field_value = match fields_map.get("ip_owner_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'ip_owner_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#public_dns_name: {
                        let field_value = match fields_map.get("public_dns_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'public_dns_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#public_ip: {
                        let field_value = match fields_map.get("public_ip") {
                            Some(value) => value,
                            None => bail!("Missing field 'public_ip' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
