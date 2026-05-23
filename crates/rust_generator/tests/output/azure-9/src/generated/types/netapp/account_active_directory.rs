#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AccountActiveDirectory {
    /// If enabled, AES encryption will be enabled for SMB communication. Defaults to `false`.
    #[builder(into)]
    pub r#aes_encryption_enabled: Option<bool>,
    /// A list of DNS server IP addresses for the Active Directory domain. Only allows `IPv4` address.
    #[builder(into)]
    pub r#dns_servers: Vec<String>,
    /// The name of the Active Directory domain.
    #[builder(into)]
    pub r#domain: String,
    /// Name of the active directory machine.
    #[builder(into)]
    pub r#kerberos_ad_name: Option<String>,
    /// kdc server IP addresses for the active directory machine.
    /// 
    /// > **IMPORTANT:** If you plan on using **Kerberos** volumes, both `ad_name` and `kdc_ip` are required in order to create the volume.
    #[builder(into)]
    pub r#kerberos_kdc_ip: Option<String>,
    /// Specifies whether or not the LDAP traffic needs to be secured via TLS. Defaults to `false`.
    #[builder(into)]
    pub r#ldap_over_tls_enabled: Option<bool>,
    /// Specifies whether or not the LDAP traffic needs to be signed. Defaults to `false`.
    #[builder(into)]
    pub r#ldap_signing_enabled: Option<bool>,
    /// If enabled, NFS client local users can also (in addition to LDAP users) access the NFS volumes. Defaults to `false`.
    #[builder(into)]
    pub r#local_nfs_users_with_ldap_allowed: Option<bool>,
    /// The Organizational Unit (OU) within Active Directory where machines will be created. If blank, defaults to `CN=Computers`.
    #[builder(into)]
    pub r#organizational_unit: Option<String>,
    /// The password associated with the `username`.
    #[builder(into)]
    pub r#password: String,
    /// When LDAP over SSL/TLS is enabled, the LDAP client is required to have a *base64 encoded Active Directory Certificate Service's self-signed root CA certificate*, this optional parameter is used only for dual protocol with LDAP user-mapping volumes. Required if `ldap_over_tls_enabled` is set to `true`.
    #[builder(into)]
    pub r#server_root_ca_certificate: Option<String>,
    /// The Active Directory site the service will limit Domain Controller discovery to. If blank, defaults to `Default-First-Site-Name`.
    #[builder(into)]
    pub r#site_name: Option<String>,
    /// The NetBIOS name which should be used for the NetApp SMB Server, which will be registered as a computer account in the AD and used to mount volumes.
    #[builder(into)]
    pub r#smb_server_name: String,
    /// The Username of Active Directory Domain Administrator.
    #[builder(into)]
    pub r#username: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for AccountActiveDirectory {
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
                    "aesEncryptionEnabled",
                    &self.r#aes_encryption_enabled,
                ),
                to_pulumi_object_field(
                    "dnsServers",
                    &self.r#dns_servers,
                ),
                to_pulumi_object_field(
                    "domain",
                    &self.r#domain,
                ),
                to_pulumi_object_field(
                    "kerberosAdName",
                    &self.r#kerberos_ad_name,
                ),
                to_pulumi_object_field(
                    "kerberosKdcIp",
                    &self.r#kerberos_kdc_ip,
                ),
                to_pulumi_object_field(
                    "ldapOverTlsEnabled",
                    &self.r#ldap_over_tls_enabled,
                ),
                to_pulumi_object_field(
                    "ldapSigningEnabled",
                    &self.r#ldap_signing_enabled,
                ),
                to_pulumi_object_field(
                    "localNfsUsersWithLdapAllowed",
                    &self.r#local_nfs_users_with_ldap_allowed,
                ),
                to_pulumi_object_field(
                    "organizationalUnit",
                    &self.r#organizational_unit,
                ),
                to_pulumi_object_field(
                    "password",
                    &self.r#password,
                ),
                to_pulumi_object_field(
                    "serverRootCaCertificate",
                    &self.r#server_root_ca_certificate,
                ),
                to_pulumi_object_field(
                    "siteName",
                    &self.r#site_name,
                ),
                to_pulumi_object_field(
                    "smbServerName",
                    &self.r#smb_server_name,
                ),
                to_pulumi_object_field(
                    "username",
                    &self.r#username,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
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
                        let field_value = match fields_map.get("aesEncryptionEnabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'aesEncryptionEnabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#dns_servers: {
                        let field_value = match fields_map.get("dnsServers") {
                            Some(value) => value,
                            None => bail!("Missing field 'dnsServers' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("kerberosAdName") {
                            Some(value) => value,
                            None => bail!("Missing field 'kerberosAdName' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#kerberos_kdc_ip: {
                        let field_value = match fields_map.get("kerberosKdcIp") {
                            Some(value) => value,
                            None => bail!("Missing field 'kerberosKdcIp' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ldap_over_tls_enabled: {
                        let field_value = match fields_map.get("ldapOverTlsEnabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'ldapOverTlsEnabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ldap_signing_enabled: {
                        let field_value = match fields_map.get("ldapSigningEnabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'ldapSigningEnabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#local_nfs_users_with_ldap_allowed: {
                        let field_value = match fields_map.get("localNfsUsersWithLdapAllowed") {
                            Some(value) => value,
                            None => bail!("Missing field 'localNfsUsersWithLdapAllowed' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#organizational_unit: {
                        let field_value = match fields_map.get("organizationalUnit") {
                            Some(value) => value,
                            None => bail!("Missing field 'organizationalUnit' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("serverRootCaCertificate") {
                            Some(value) => value,
                            None => bail!("Missing field 'serverRootCaCertificate' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#site_name: {
                        let field_value = match fields_map.get("siteName") {
                            Some(value) => value,
                            None => bail!("Missing field 'siteName' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#smb_server_name: {
                        let field_value = match fields_map.get("smbServerName") {
                            Some(value) => value,
                            None => bail!("Missing field 'smbServerName' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
