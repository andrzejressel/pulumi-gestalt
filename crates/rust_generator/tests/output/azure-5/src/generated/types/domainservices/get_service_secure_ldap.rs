#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetServiceSecureLdap {
    #[builder(into)]
    #[serde(rename = "certificateExpiry")]
    pub r#certificate_expiry: String,
    #[builder(into)]
    #[serde(rename = "certificateThumbprint")]
    pub r#certificate_thumbprint: String,
    /// Whether secure LDAP is enabled for the managed domain.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: bool,
    /// Whether external access to LDAPS over the Internet, is enabled.
    #[builder(into)]
    #[serde(rename = "externalAccessEnabled")]
    pub r#external_access_enabled: bool,
    #[builder(into)]
    #[serde(rename = "publicCertificate")]
    pub r#public_certificate: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetServiceSecureLdap {
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
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetServiceSecureLdap {
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
