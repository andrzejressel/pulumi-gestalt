#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct NetworkConnectionMonitorEndpoint {
    /// The IP address or domain name of the Network Connection Monitor endpoint.
    #[builder(into)]
    #[serde(rename = "address")]
    pub r#address: Option<String>,
    /// The test coverage for the Network Connection Monitor endpoint. Possible values are `AboveAverage`, `Average`, `BelowAverage`, `Default`, `Full` and `Low`.
    #[builder(into)]
    #[serde(rename = "coverageLevel")]
    pub r#coverage_level: Option<String>,
    /// A list of IPv4/IPv6 subnet masks or IPv4/IPv6 IP addresses to be excluded to the Network Connection Monitor endpoint.
    #[builder(into)]
    #[serde(rename = "excludedIpAddresses")]
    pub r#excluded_ip_addresses: Option<Vec<String>>,
    /// A `filter` block as defined below.
    #[builder(into)]
    #[serde(rename = "filter")]
    pub r#filter: Option<Box<super::super::types::network::NetworkConnectionMonitorEndpointFilter>>,
    /// A list of IPv4/IPv6 subnet masks or IPv4/IPv6 IP addresses to be included to the Network Connection Monitor endpoint.
    #[builder(into)]
    #[serde(rename = "includedIpAddresses")]
    pub r#included_ip_addresses: Option<Vec<String>>,
    /// The name of the endpoint for the Network Connection Monitor .
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// The resource ID which is used as the endpoint by the Network Connection Monitor.
    #[builder(into)]
    #[serde(rename = "targetResourceId")]
    pub r#target_resource_id: Option<String>,
    /// The endpoint type of the Network Connection Monitor. Possible values are `AzureArcVM`, `AzureSubnet`, `AzureVM`, `AzureVNet`, `ExternalAddress`, `MMAWorkspaceMachine` and `MMAWorkspaceNetwork`.
    #[builder(into)]
    #[serde(rename = "targetResourceType")]
    pub r#target_resource_type: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for NetworkConnectionMonitorEndpoint {
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
                "address".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#address,
                )
                .await,
            );
            map.insert(
                "coverage_level".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#coverage_level,
                )
                .await,
            );
            map.insert(
                "excluded_ip_addresses".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#excluded_ip_addresses,
                )
                .await,
            );
            map.insert(
                "filter".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#filter,
                )
                .await,
            );
            map.insert(
                "included_ip_addresses".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#included_ip_addresses,
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
                "target_resource_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#target_resource_id,
                )
                .await,
            );
            map.insert(
                "target_resource_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#target_resource_type,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for NetworkConnectionMonitorEndpoint {
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
                    r#address: {
                        let field_value = match fields_map.get("address") {
                            Some(value) => value,
                            None => bail!("Missing field 'address' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#coverage_level: {
                        let field_value = match fields_map.get("coverage_level") {
                            Some(value) => value,
                            None => bail!("Missing field 'coverage_level' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#excluded_ip_addresses: {
                        let field_value = match fields_map.get("excluded_ip_addresses") {
                            Some(value) => value,
                            None => bail!("Missing field 'excluded_ip_addresses' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#filter: {
                        let field_value = match fields_map.get("filter") {
                            Some(value) => value,
                            None => bail!("Missing field 'filter' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#included_ip_addresses: {
                        let field_value = match fields_map.get("included_ip_addresses") {
                            Some(value) => value,
                            None => bail!("Missing field 'included_ip_addresses' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#target_resource_id: {
                        let field_value = match fields_map.get("target_resource_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'target_resource_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#target_resource_type: {
                        let field_value = match fields_map.get("target_resource_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'target_resource_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
