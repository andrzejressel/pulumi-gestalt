#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DatabaseInstanceSettingsIpConfiguration {
    /// The name of the allocated ip range for the private ip CloudSQL instance. For example: "google-managed-services-default". If set, the instance ip will be created in the allocated range. The range name must comply with [RFC 1035](https://datatracker.ietf.org/doc/html/rfc1035). Specifically, the name must be 1-63 characters long and match the regular expression a-z?.
    #[builder(into)]
    #[serde(rename = "allocatedIpRange")]
    pub r#allocated_ip_range: Option<String>,
    #[builder(into)]
    #[serde(rename = "authorizedNetworks")]
    pub r#authorized_networks: Option<Vec<super::super::types::sql::DatabaseInstanceSettingsIpConfigurationAuthorizedNetwork>>,
    /// Whether Google Cloud services such as BigQuery are allowed to access data in this Cloud SQL instance over a private IP connection. SQLSERVER database type is not supported.
    #[builder(into)]
    #[serde(rename = "enablePrivatePathForGoogleCloudServices")]
    pub r#enable_private_path_for_google_cloud_services: Option<bool>,
    /// Whether this Cloud SQL instance should be assigned
    /// a public IPV4 address. At least `ipv4_enabled` must be enabled or a
    /// `private_network` must be configured.
    #[builder(into)]
    #[serde(rename = "ipv4Enabled")]
    pub r#ipv_4_enabled: Option<bool>,
    /// The VPC network from which the Cloud SQL
    /// instance is accessible for private IP. For example, projects/myProject/global/networks/default.
    /// Specifying a network enables private IP.
    /// At least `ipv4_enabled` must be enabled or a `private_network` must be configured.
    /// This setting can be updated, but it cannot be removed after it is set.
    #[builder(into)]
    #[serde(rename = "privateNetwork")]
    pub r#private_network: Option<String>,
    /// PSC settings for a Cloud SQL instance.
    #[builder(into)]
    #[serde(rename = "pscConfigs")]
    pub r#psc_configs: Option<Vec<super::super::types::sql::DatabaseInstanceSettingsIpConfigurationPscConfig>>,
    /// Specify how the server certificate's Certificate Authority is hosted. Supported values are `GOOGLE_MANAGED_INTERNAL_CA` and `GOOGLE_MANAGED_CAS_CA`.
    #[builder(into)]
    #[serde(rename = "serverCaMode")]
    pub r#server_ca_mode: Option<String>,
    /// Specify how SSL connection should be enforced in DB connections. Supported values are `ALLOW_UNENCRYPTED_AND_ENCRYPTED`, `ENCRYPTED_ONLY`, and `TRUSTED_CLIENT_CERTIFICATE_REQUIRED` (not supported for SQL Server). See [API reference doc](https://cloud.google.com/sql/docs/postgres/admin-api/rest/v1/instances#ipconfiguration) for details.
    #[builder(into)]
    #[serde(rename = "sslMode")]
    pub r#ssl_mode: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for DatabaseInstanceSettingsIpConfiguration {
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
                    "allocated_ip_range",
                    &self.r#allocated_ip_range,
                ),
                to_pulumi_object_field(
                    "authorized_networks",
                    &self.r#authorized_networks,
                ),
                to_pulumi_object_field(
                    "enable_private_path_for_google_cloud_services",
                    &self.r#enable_private_path_for_google_cloud_services,
                ),
                to_pulumi_object_field(
                    "ipv_4_enabled",
                    &self.r#ipv_4_enabled,
                ),
                to_pulumi_object_field(
                    "private_network",
                    &self.r#private_network,
                ),
                to_pulumi_object_field(
                    "psc_configs",
                    &self.r#psc_configs,
                ),
                to_pulumi_object_field(
                    "server_ca_mode",
                    &self.r#server_ca_mode,
                ),
                to_pulumi_object_field(
                    "ssl_mode",
                    &self.r#ssl_mode,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for DatabaseInstanceSettingsIpConfiguration {
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
