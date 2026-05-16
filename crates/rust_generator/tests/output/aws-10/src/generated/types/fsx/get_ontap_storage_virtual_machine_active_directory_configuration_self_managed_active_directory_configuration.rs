#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetOntapStorageVirtualMachineActiveDirectoryConfigurationSelfManagedActiveDirectoryConfiguration {
    /// A list of up to three IP addresses of DNS servers or domain controllers in the self-managed AD directory.
    #[builder(into)]
    #[serde(rename = "dnsIps")]
    pub r#dns_ips: Vec<String>,
    /// The fully qualified domain name of the self-managed AD directory.
    #[builder(into)]
    #[serde(rename = "domainName")]
    pub r#domain_name: String,
    /// The name of the domain group whose members have administrative privileges for the FSx file system.
    #[builder(into)]
    #[serde(rename = "fileSystemAdministratorsGroup")]
    pub r#file_system_administrators_group: String,
    /// The fully qualified distinguished name of the organizational unit within the self-managed AD directory to which the Windows File Server or ONTAP storage virtual machine (SVM) instance is joined.
    #[builder(into)]
    #[serde(rename = "organizationalUnitDistinguishedName")]
    pub r#organizational_unit_distinguished_name: String,
    /// The user name for the service account on your self-managed AD domain that FSx uses to join to your AD domain.
    #[builder(into)]
    #[serde(rename = "username")]
    pub r#username: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetOntapStorageVirtualMachineActiveDirectoryConfigurationSelfManagedActiveDirectoryConfiguration {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("dns_ips".to_string(), self.r#dns_ips.to_pulumi_value().await);
            map.insert("domain_name".to_string(), self.r#domain_name.to_pulumi_value().await);
            map.insert("file_system_administrators_group".to_string(), self.r#file_system_administrators_group.to_pulumi_value().await);
            map.insert("organizational_unit_distinguished_name".to_string(), self.r#organizational_unit_distinguished_name.to_pulumi_value().await);
            map.insert("username".to_string(), self.r#username.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetOntapStorageVirtualMachineActiveDirectoryConfigurationSelfManagedActiveDirectoryConfiguration {
    fn from_pulumi_value(
        value: &pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    ) -> pulumi_gestalt_rust::__private::rootcause::Result<Self> {
        use std::collections::BTreeMap;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValueContent;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue;
        use pulumi_gestalt_rust::__private::rootcause::bail;

        match value.content {
            PulumiValueContent::Object(ref obj) => {
                let fields_map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> =
                    obj.iter().cloned().collect();

                Ok(Self {
                    r#dns_ips: {
                        let field_value = match fields_map.get("dns_ips") {
                            Some(value) => value,
                            None => bail!("Missing field 'dns_ips' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Vec<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#domain_name: {
                        let field_value = match fields_map.get("domain_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'domain_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#file_system_administrators_group: {
                        let field_value = match fields_map.get("file_system_administrators_group") {
                            Some(value) => value,
                            None => bail!("Missing field 'file_system_administrators_group' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#organizational_unit_distinguished_name: {
                        let field_value = match fields_map.get("organizational_unit_distinguished_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'organizational_unit_distinguished_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#username: {
                        let field_value = match fields_map.get("username") {
                            Some(value) => value,
                            None => bail!("Missing field 'username' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
