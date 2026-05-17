#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AccountActiveDirectory {
    /// If enabled, AES encryption will be enabled for SMB communication. Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "aesEncryptionEnabled")]
    pub r#aes_encryption_enabled: Option<bool>,
    /// A list of DNS server IP addresses for the Active Directory domain. Only allows `IPv4` address.
    #[builder(into)]
    #[serde(rename = "dnsServers")]
    pub r#dns_servers: Vec<String>,
    /// The name of the Active Directory domain.
    #[builder(into)]
    #[serde(rename = "domain")]
    pub r#domain: String,
    /// Name of the active directory machine.
    #[builder(into)]
    #[serde(rename = "kerberosAdName")]
    pub r#kerberos_ad_name: Option<String>,
    /// kdc server IP addresses for the active directory machine.
    /// 
    /// > **IMPORTANT:** If you plan on using **Kerberos** volumes, both `ad_name` and `kdc_ip` are required in order to create the volume.
    #[builder(into)]
    #[serde(rename = "kerberosKdcIp")]
    pub r#kerberos_kdc_ip: Option<String>,
    /// Specifies whether or not the LDAP traffic needs to be secured via TLS. Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "ldapOverTlsEnabled")]
    pub r#ldap_over_tls_enabled: Option<bool>,
    /// Specifies whether or not the LDAP traffic needs to be signed. Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "ldapSigningEnabled")]
    pub r#ldap_signing_enabled: Option<bool>,
    /// If enabled, NFS client local users can also (in addition to LDAP users) access the NFS volumes. Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "localNfsUsersWithLdapAllowed")]
    pub r#local_nfs_users_with_ldap_allowed: Option<bool>,
    /// The Organizational Unit (OU) within Active Directory where machines will be created. If blank, defaults to `CN=Computers`.
    #[builder(into)]
    #[serde(rename = "organizationalUnit")]
    pub r#organizational_unit: Option<String>,
    /// The password associated with the `username`.
    #[builder(into)]
    #[serde(rename = "password")]
    pub r#password: String,
    /// When LDAP over SSL/TLS is enabled, the LDAP client is required to have a *base64 encoded Active Directory Certificate Service's self-signed root CA certificate*, this optional parameter is used only for dual protocol with LDAP user-mapping volumes. Required if `ldap_over_tls_enabled` is set to `true`.
    #[builder(into)]
    #[serde(rename = "serverRootCaCertificate")]
    pub r#server_root_ca_certificate: Option<String>,
    /// The Active Directory site the service will limit Domain Controller discovery to. If blank, defaults to `Default-First-Site-Name`.
    #[builder(into)]
    #[serde(rename = "siteName")]
    pub r#site_name: Option<String>,
    /// The NetBIOS name which should be used for the NetApp SMB Server, which will be registered as a computer account in the AD and used to mount volumes.
    #[builder(into)]
    #[serde(rename = "smbServerName")]
    pub r#smb_server_name: String,
    /// The Username of Active Directory Domain Administrator.
    #[builder(into)]
    #[serde(rename = "username")]
    pub r#username: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for AccountActiveDirectory {
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
                "aes_encryption_enabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#aes_encryption_enabled,
                )
                .await,
            );
            map.insert(
                "dns_servers".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#dns_servers,
                )
                .await,
            );
            map.insert(
                "domain".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#domain,
                )
                .await,
            );
            map.insert(
                "kerberos_ad_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#kerberos_ad_name,
                )
                .await,
            );
            map.insert(
                "kerberos_kdc_ip".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#kerberos_kdc_ip,
                )
                .await,
            );
            map.insert(
                "ldap_over_tls_enabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#ldap_over_tls_enabled,
                )
                .await,
            );
            map.insert(
                "ldap_signing_enabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#ldap_signing_enabled,
                )
                .await,
            );
            map.insert(
                "local_nfs_users_with_ldap_allowed".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#local_nfs_users_with_ldap_allowed,
                )
                .await,
            );
            map.insert(
                "organizational_unit".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#organizational_unit,
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
                "server_root_ca_certificate".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#server_root_ca_certificate,
                )
                .await,
            );
            map.insert(
                "site_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#site_name,
                )
                .await,
            );
            map.insert(
                "smb_server_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#smb_server_name,
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
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for AccountActiveDirectory {
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
                    r#aes_encryption_enabled: {
                        let field_value = match fields_map.get("aes_encryption_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'aes_encryption_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#dns_servers: {
                        let field_value = match fields_map.get("dns_servers") {
                            Some(value) => value,
                            None => bail!("Missing field 'dns_servers' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#domain: {
                        let field_value = match fields_map.get("domain") {
                            Some(value) => value,
                            None => bail!("Missing field 'domain' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#kerberos_ad_name: {
                        let field_value = match fields_map.get("kerberos_ad_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'kerberos_ad_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#kerberos_kdc_ip: {
                        let field_value = match fields_map.get("kerberos_kdc_ip") {
                            Some(value) => value,
                            None => bail!("Missing field 'kerberos_kdc_ip' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ldap_over_tls_enabled: {
                        let field_value = match fields_map.get("ldap_over_tls_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'ldap_over_tls_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ldap_signing_enabled: {
                        let field_value = match fields_map.get("ldap_signing_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'ldap_signing_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#local_nfs_users_with_ldap_allowed: {
                        let field_value = match fields_map.get("local_nfs_users_with_ldap_allowed") {
                            Some(value) => value,
                            None => bail!("Missing field 'local_nfs_users_with_ldap_allowed' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#organizational_unit: {
                        let field_value = match fields_map.get("organizational_unit") {
                            Some(value) => value,
                            None => bail!("Missing field 'organizational_unit' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#server_root_ca_certificate: {
                        let field_value = match fields_map.get("server_root_ca_certificate") {
                            Some(value) => value,
                            None => bail!("Missing field 'server_root_ca_certificate' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#site_name: {
                        let field_value = match fields_map.get("site_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'site_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#smb_server_name: {
                        let field_value = match fields_map.get("smb_server_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'smb_server_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
