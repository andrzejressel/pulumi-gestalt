#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct CacheDirectoryLdap {
    /// The base distinguished name (DN) for the LDAP domain.
    #[builder(into)]
    #[serde(rename = "baseDn")]
    pub r#base_dn: String,
    /// A `bind` block as defined above.
    #[builder(into)]
    #[serde(rename = "bind")]
    pub r#bind: Option<Box<super::super::types::hpc::CacheDirectoryLdapBind>>,
    /// The URI of the CA certificate to validate the LDAP secure connection.
    #[builder(into)]
    #[serde(rename = "certificateValidationUri")]
    pub r#certificate_validation_uri: Option<String>,
    /// Whether the certificate should be automatically downloaded. This can be set to `true` only when `certificate_validation_uri` is provided.
    #[builder(into)]
    #[serde(rename = "downloadCertificateAutomatically")]
    pub r#download_certificate_automatically: Option<bool>,
    /// Whether the LDAP connection should be encrypted?
    #[builder(into)]
    #[serde(rename = "encrypted")]
    pub r#encrypted: Option<bool>,
    /// The FQDN or IP address of the LDAP server.
    #[builder(into)]
    #[serde(rename = "server")]
    pub r#server: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for CacheDirectoryLdap {
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
                    "base_dn",
                    &self.r#base_dn,
                ),
                to_pulumi_object_field(
                    "bind",
                    &self.r#bind,
                ),
                to_pulumi_object_field(
                    "certificate_validation_uri",
                    &self.r#certificate_validation_uri,
                ),
                to_pulumi_object_field(
                    "download_certificate_automatically",
                    &self.r#download_certificate_automatically,
                ),
                to_pulumi_object_field(
                    "encrypted",
                    &self.r#encrypted,
                ),
                to_pulumi_object_field(
                    "server",
                    &self.r#server,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for CacheDirectoryLdap {
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
                    r#base_dn: {
                        let field_value = match fields_map.get("base_dn") {
                            Some(value) => value,
                            None => bail!("Missing field 'base_dn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#bind: {
                        let field_value = match fields_map.get("bind") {
                            Some(value) => value,
                            None => bail!("Missing field 'bind' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#certificate_validation_uri: {
                        let field_value = match fields_map.get("certificate_validation_uri") {
                            Some(value) => value,
                            None => bail!("Missing field 'certificate_validation_uri' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#download_certificate_automatically: {
                        let field_value = match fields_map.get("download_certificate_automatically") {
                            Some(value) => value,
                            None => bail!("Missing field 'download_certificate_automatically' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#encrypted: {
                        let field_value = match fields_map.get("encrypted") {
                            Some(value) => value,
                            None => bail!("Missing field 'encrypted' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#server: {
                        let field_value = match fields_map.get("server") {
                            Some(value) => value,
                            None => bail!("Missing field 'server' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
