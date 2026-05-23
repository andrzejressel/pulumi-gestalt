#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetAccountAzureFilesAuthenticationActiveDirectory {
    /// The domain GUID.
    #[builder(into)]
    pub r#domain_guid: String,
    /// The primary domain that the AD DNS server is authoritative for.
    #[builder(into)]
    pub r#domain_name: String,
    /// The domain security identifier.
    #[builder(into)]
    pub r#domain_sid: String,
    /// The name of the Active Directory forest.
    #[builder(into)]
    pub r#forest_name: String,
    /// The NetBIOS domain name.
    #[builder(into)]
    pub r#netbios_domain_name: String,
    /// The security identifier for Azure Storage.
    #[builder(into)]
    pub r#storage_sid: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetAccountAzureFilesAuthenticationActiveDirectory {
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
                    "domainGuid",
                    &self.r#domain_guid,
                ),
                to_pulumi_object_field(
                    "domainName",
                    &self.r#domain_name,
                ),
                to_pulumi_object_field(
                    "domainSid",
                    &self.r#domain_sid,
                ),
                to_pulumi_object_field(
                    "forestName",
                    &self.r#forest_name,
                ),
                to_pulumi_object_field(
                    "netbiosDomainName",
                    &self.r#netbios_domain_name,
                ),
                to_pulumi_object_field(
                    "storageSid",
                    &self.r#storage_sid,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetAccountAzureFilesAuthenticationActiveDirectory {
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
                        let field_value = match fields_map.get("domainGuid") {
                            Some(value) => value,
                            None => bail!("Missing field 'domainGuid' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#domain_name: {
                        let field_value = match fields_map.get("domainName") {
                            Some(value) => value,
                            None => bail!("Missing field 'domainName' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#domain_sid: {
                        let field_value = match fields_map.get("domainSid") {
                            Some(value) => value,
                            None => bail!("Missing field 'domainSid' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#forest_name: {
                        let field_value = match fields_map.get("forestName") {
                            Some(value) => value,
                            None => bail!("Missing field 'forestName' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#netbios_domain_name: {
                        let field_value = match fields_map.get("netbiosDomainName") {
                            Some(value) => value,
                            None => bail!("Missing field 'netbiosDomainName' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#storage_sid: {
                        let field_value = match fields_map.get("storageSid") {
                            Some(value) => value,
                            None => bail!("Missing field 'storageSid' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
