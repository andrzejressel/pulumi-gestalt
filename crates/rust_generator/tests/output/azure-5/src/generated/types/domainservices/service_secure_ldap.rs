#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ServiceSecureLdap {
    /// The expiry time of the certificate.
    #[builder(into)]
    #[serde(rename = "certificateExpiry")]
    pub r#certificate_expiry: Option<String>,
    /// The thumbprint of the certificate.
    #[builder(into)]
    #[serde(rename = "certificateThumbprint")]
    pub r#certificate_thumbprint: Option<String>,
    /// Whether to enable secure LDAP for the managed domain. For more information, please see [official documentation on enabling LDAPS](https://docs.microsoft.com/azure/active-directory-domain-services/tutorial-configure-ldaps), paying particular attention to the section on network security to avoid unnecessarily exposing your service to Internet-borne bruteforce attacks.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: bool,
    /// Whether to enable external access to LDAPS over the Internet. Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "externalAccessEnabled")]
    pub r#external_access_enabled: Option<bool>,
    /// The certificate/private key to use for LDAPS, as a base64-encoded TripleDES-SHA1 encrypted PKCS#12 bundle (PFX file).
    #[builder(into)]
    #[serde(rename = "pfxCertificate")]
    pub r#pfx_certificate: String,
    /// The password to use for decrypting the PKCS#12 bundle (PFX file).
    #[builder(into)]
    #[serde(rename = "pfxCertificatePassword")]
    pub r#pfx_certificate_password: String,
    /// The public certificate.
    #[builder(into)]
    #[serde(rename = "publicCertificate")]
    pub r#public_certificate: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ServiceSecureLdap {
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
                "certificate_expiry".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#certificate_expiry,
                )
                .await,
            );
            map.insert(
                "certificate_thumbprint".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#certificate_thumbprint,
                )
                .await,
            );
            map.insert(
                "enabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#enabled,
                )
                .await,
            );
            map.insert(
                "external_access_enabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#external_access_enabled,
                )
                .await,
            );
            map.insert(
                "pfx_certificate".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#pfx_certificate,
                )
                .await,
            );
            map.insert(
                "pfx_certificate_password".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#pfx_certificate_password,
                )
                .await,
            );
            map.insert(
                "public_certificate".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#public_certificate,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ServiceSecureLdap {
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
                    r#certificate_expiry: {
                        let field_value = match fields_map.get("certificate_expiry") {
                            Some(value) => value,
                            None => bail!("Missing field 'certificate_expiry' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#certificate_thumbprint: {
                        let field_value = match fields_map.get("certificate_thumbprint") {
                            Some(value) => value,
                            None => bail!("Missing field 'certificate_thumbprint' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#enabled: {
                        let field_value = match fields_map.get("enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#external_access_enabled: {
                        let field_value = match fields_map.get("external_access_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'external_access_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#pfx_certificate: {
                        let field_value = match fields_map.get("pfx_certificate") {
                            Some(value) => value,
                            None => bail!("Missing field 'pfx_certificate' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#pfx_certificate_password: {
                        let field_value = match fields_map.get("pfx_certificate_password") {
                            Some(value) => value,
                            None => bail!("Missing field 'pfx_certificate_password' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#public_certificate: {
                        let field_value = match fields_map.get("public_certificate") {
                            Some(value) => value,
                            None => bail!("Missing field 'public_certificate' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
