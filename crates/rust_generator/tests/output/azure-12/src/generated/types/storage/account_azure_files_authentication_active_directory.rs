#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AccountAzureFilesAuthenticationActiveDirectory {
    /// Specifies the domain GUID.
    #[builder(into)]
    #[serde(rename = "domainGuid")]
    pub r#domain_guid: String,
    /// Specifies the primary domain that the AD DNS server is authoritative for.
    #[builder(into)]
    #[serde(rename = "domainName")]
    pub r#domain_name: String,
    /// Specifies the security identifier (SID). This is required when `directory_type` is set to `AD`.
    #[builder(into)]
    #[serde(rename = "domainSid")]
    pub r#domain_sid: Option<String>,
    /// Specifies the Active Directory forest. This is required when `directory_type` is set to `AD`.
    #[builder(into)]
    #[serde(rename = "forestName")]
    pub r#forest_name: Option<String>,
    /// Specifies the NetBIOS domain name. This is required when `directory_type` is set to `AD`.
    #[builder(into)]
    #[serde(rename = "netbiosDomainName")]
    pub r#netbios_domain_name: Option<String>,
    /// Specifies the security identifier (SID) for Azure Storage. This is required when `directory_type` is set to `AD`.
    #[builder(into)]
    #[serde(rename = "storageSid")]
    pub r#storage_sid: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for AccountAzureFilesAuthenticationActiveDirectory {
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
                "domain_guid".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#domain_guid,
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
                "domain_sid".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#domain_sid,
                )
                .await,
            );
            map.insert(
                "forest_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#forest_name,
                )
                .await,
            );
            map.insert(
                "netbios_domain_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#netbios_domain_name,
                )
                .await,
            );
            map.insert(
                "storage_sid".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#storage_sid,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for AccountAzureFilesAuthenticationActiveDirectory {
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
                    r#domain_guid: {
                        let field_value = match fields_map.get("domain_guid") {
                            Some(value) => value,
                            None => bail!("Missing field 'domain_guid' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#domain_sid: {
                        let field_value = match fields_map.get("domain_sid") {
                            Some(value) => value,
                            None => bail!("Missing field 'domain_sid' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#forest_name: {
                        let field_value = match fields_map.get("forest_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'forest_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#netbios_domain_name: {
                        let field_value = match fields_map.get("netbios_domain_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'netbios_domain_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#storage_sid: {
                        let field_value = match fields_map.get("storage_sid") {
                            Some(value) => value,
                            None => bail!("Missing field 'storage_sid' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
