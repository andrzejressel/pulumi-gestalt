#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct CacheDirectoryActiveDirectory {
    /// The NetBIOS name to assign to the HPC Cache when it joins the Active Directory domain as a server.
    #[builder(into)]
    #[serde(rename = "cacheNetbiosName")]
    pub r#cache_netbios_name: String,
    /// The primary DNS IP address used to resolve the Active Directory domain controller's FQDN.
    #[builder(into)]
    #[serde(rename = "dnsPrimaryIp")]
    pub r#dns_primary_ip: String,
    /// The secondary DNS IP address used to resolve the Active Directory domain controller's FQDN.
    #[builder(into)]
    #[serde(rename = "dnsSecondaryIp")]
    pub r#dns_secondary_ip: Option<String>,
    /// The fully qualified domain name of the Active Directory domain controller.
    #[builder(into)]
    #[serde(rename = "domainName")]
    pub r#domain_name: String,
    /// The Active Directory domain's NetBIOS name.
    #[builder(into)]
    #[serde(rename = "domainNetbiosName")]
    pub r#domain_netbios_name: String,
    /// The password of the Active Directory domain administrator.
    #[builder(into)]
    #[serde(rename = "password")]
    pub r#password: String,
    /// The username of the Active Directory domain administrator.
    #[builder(into)]
    #[serde(rename = "username")]
    pub r#username: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for CacheDirectoryActiveDirectory {
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
                "cache_netbios_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#cache_netbios_name,
                )
                .await,
            );
            map.insert(
                "dns_primary_ip".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#dns_primary_ip,
                )
                .await,
            );
            map.insert(
                "dns_secondary_ip".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#dns_secondary_ip,
                )
                .await,
            );
            map.insert(
                "domain_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#domain_name,
                )
                .await,
            );
            map.insert(
                "domain_netbios_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#domain_netbios_name,
                )
                .await,
            );
            map.insert(
                "password".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#password,
                )
                .await,
            );
            map.insert(
                "username".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#username,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for CacheDirectoryActiveDirectory {
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
                    r#cache_netbios_name: {
                        let field_value = match fields_map.get("cache_netbios_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'cache_netbios_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#dns_primary_ip: {
                        let field_value = match fields_map.get("dns_primary_ip") {
                            Some(value) => value,
                            None => bail!("Missing field 'dns_primary_ip' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#dns_secondary_ip: {
                        let field_value = match fields_map.get("dns_secondary_ip") {
                            Some(value) => value,
                            None => bail!("Missing field 'dns_secondary_ip' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#domain_name: {
                        let field_value = match fields_map.get("domain_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'domain_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#domain_netbios_name: {
                        let field_value = match fields_map.get("domain_netbios_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'domain_netbios_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#password: {
                        let field_value = match fields_map.get("password") {
                            Some(value) => value,
                            None => bail!("Missing field 'password' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#username: {
                        let field_value = match fields_map.get("username") {
                            Some(value) => value,
                            None => bail!("Missing field 'username' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
