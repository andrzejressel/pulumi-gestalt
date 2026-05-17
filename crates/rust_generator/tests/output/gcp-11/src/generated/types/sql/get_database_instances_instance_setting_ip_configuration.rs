#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetDatabaseInstancesInstanceSettingIpConfiguration {
    /// The name of the allocated ip range for the private ip CloudSQL instance. For example: "google-managed-services-default". If set, the instance ip will be created in the allocated range. The range name must comply with RFC 1035. Specifically, the name must be 1-63 characters long and match the regular expression a-z?.
    #[builder(into)]
    #[serde(rename = "allocatedIpRange")]
    pub r#allocated_ip_range: String,
    #[builder(into)]
    #[serde(rename = "authorizedNetworks")]
    pub r#authorized_networks: Vec<super::super::types::sql::GetDatabaseInstancesInstanceSettingIpConfigurationAuthorizedNetwork>,
    /// Whether Google Cloud services such as BigQuery are allowed to access data in this Cloud SQL instance over a private IP connection. SQLSERVER database type is not supported.
    #[builder(into)]
    #[serde(rename = "enablePrivatePathForGoogleCloudServices")]
    pub r#enable_private_path_for_google_cloud_services: bool,
    /// Whether this Cloud SQL instance should be assigned a public IPV4 address. At least ipv4_enabled must be enabled or a private_network must be configured.
    #[builder(into)]
    #[serde(rename = "ipv4Enabled")]
    pub r#ipv_4_enabled: bool,
    /// The VPC network from which the Cloud SQL instance is accessible for private IP. For example, projects/myProject/global/networks/default. Specifying a network enables private IP. At least ipv4_enabled must be enabled or a private_network must be configured. This setting can be updated, but it cannot be removed after it is set.
    #[builder(into)]
    #[serde(rename = "privateNetwork")]
    pub r#private_network: String,
    /// PSC settings for a Cloud SQL instance.
    #[builder(into)]
    #[serde(rename = "pscConfigs")]
    pub r#psc_configs: Vec<super::super::types::sql::GetDatabaseInstancesInstanceSettingIpConfigurationPscConfig>,
    /// Specify how the server certificate's Certificate Authority is hosted.
    #[builder(into)]
    #[serde(rename = "serverCaMode")]
    pub r#server_ca_mode: String,
    /// Specify how SSL connection should be enforced in DB connections.
    #[builder(into)]
    #[serde(rename = "sslMode")]
    pub r#ssl_mode: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetDatabaseInstancesInstanceSettingIpConfiguration {
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
                "allocated_ip_range".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#allocated_ip_range,
                )
                .await,
            );
            map.insert(
                "authorized_networks".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#authorized_networks,
                )
                .await,
            );
            map.insert(
                "enable_private_path_for_google_cloud_services".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#enable_private_path_for_google_cloud_services,
                )
                .await,
            );
            map.insert(
                "ipv_4_enabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#ipv_4_enabled,
                )
                .await,
            );
            map.insert(
                "private_network".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#private_network,
                )
                .await,
            );
            map.insert(
                "psc_configs".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#psc_configs,
                )
                .await,
            );
            map.insert(
                "server_ca_mode".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#server_ca_mode,
                )
                .await,
            );
            map.insert(
                "ssl_mode".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#ssl_mode,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetDatabaseInstancesInstanceSettingIpConfiguration {
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
                    r#allocated_ip_range: {
                        let field_value = match fields_map.get("allocated_ip_range") {
                            Some(value) => value,
                            None => bail!("Missing field 'allocated_ip_range' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#authorized_networks: {
                        let field_value = match fields_map.get("authorized_networks") {
                            Some(value) => value,
                            None => bail!("Missing field 'authorized_networks' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#enable_private_path_for_google_cloud_services: {
                        let field_value = match fields_map.get("enable_private_path_for_google_cloud_services") {
                            Some(value) => value,
                            None => bail!("Missing field 'enable_private_path_for_google_cloud_services' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ipv_4_enabled: {
                        let field_value = match fields_map.get("ipv_4_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'ipv_4_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#private_network: {
                        let field_value = match fields_map.get("private_network") {
                            Some(value) => value,
                            None => bail!("Missing field 'private_network' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#psc_configs: {
                        let field_value = match fields_map.get("psc_configs") {
                            Some(value) => value,
                            None => bail!("Missing field 'psc_configs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#server_ca_mode: {
                        let field_value = match fields_map.get("server_ca_mode") {
                            Some(value) => value,
                            None => bail!("Missing field 'server_ca_mode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ssl_mode: {
                        let field_value = match fields_map.get("ssl_mode") {
                            Some(value) => value,
                            None => bail!("Missing field 'ssl_mode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
