#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetSubnetworksSubnetwork {
    /// Description of the subnetwork.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: String,
    /// The IP address range represented as a CIDR block.
    #[builder(into)]
    #[serde(rename = "ipCidrRange")]
    pub r#ip_cidr_range: String,
    /// The name of the subnetwork.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// The self link of the parent network.
    #[builder(into)]
    #[serde(rename = "network")]
    pub r#network: String,
    #[builder(into)]
    #[serde(rename = "networkSelfLink")]
    pub r#network_self_link: String,
    /// Whether the VMs in the subnet can access Google services without assigned external IP addresses.
    #[builder(into)]
    #[serde(rename = "privateIpGoogleAccess")]
    pub r#private_ip_google_access: bool,
    /// The self link of the subnetwork.
    #[builder(into)]
    #[serde(rename = "selfLink")]
    pub r#self_link: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetSubnetworksSubnetwork {
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
                "description".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#description,
                )
                .await,
            );
            map.insert(
                "ip_cidr_range".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#ip_cidr_range,
                )
                .await,
            );
            map.insert(
                "name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#name,
                )
                .await,
            );
            map.insert(
                "network".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#network,
                )
                .await,
            );
            map.insert(
                "network_self_link".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#network_self_link,
                )
                .await,
            );
            map.insert(
                "private_ip_google_access".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#private_ip_google_access,
                )
                .await,
            );
            map.insert(
                "self_link".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#self_link,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetSubnetworksSubnetwork {
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
                    r#description: {
                        let field_value = match fields_map.get("description") {
                            Some(value) => value,
                            None => bail!("Missing field 'description' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ip_cidr_range: {
                        let field_value = match fields_map.get("ip_cidr_range") {
                            Some(value) => value,
                            None => bail!("Missing field 'ip_cidr_range' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#network: {
                        let field_value = match fields_map.get("network") {
                            Some(value) => value,
                            None => bail!("Missing field 'network' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#network_self_link: {
                        let field_value = match fields_map.get("network_self_link") {
                            Some(value) => value,
                            None => bail!("Missing field 'network_self_link' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#private_ip_google_access: {
                        let field_value = match fields_map.get("private_ip_google_access") {
                            Some(value) => value,
                            None => bail!("Missing field 'private_ip_google_access' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#self_link: {
                        let field_value = match fields_map.get("self_link") {
                            Some(value) => value,
                            None => bail!("Missing field 'self_link' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
