#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct CertificateCertificateAttribute {
    /// The create time of the Key Vault Certificate.
    #[builder(into)]
    #[serde(rename = "created")]
    pub r#created: Option<String>,
    /// whether the Key Vault Certificate is enabled.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Option<bool>,
    /// The expires time of the Key Vault Certificate.
    #[builder(into)]
    #[serde(rename = "expires")]
    pub r#expires: Option<String>,
    /// The not before valid time of the Key Vault Certificate.
    #[builder(into)]
    #[serde(rename = "notBefore")]
    pub r#not_before: Option<String>,
    /// The deletion recovery level of the Key Vault Certificate.
    #[builder(into)]
    #[serde(rename = "recoveryLevel")]
    pub r#recovery_level: Option<String>,
    /// The recent update time of the Key Vault Certificate.
    #[builder(into)]
    #[serde(rename = "updated")]
    pub r#updated: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for CertificateCertificateAttribute {
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
                "created".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#created,
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
                "expires".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#expires,
                )
                .await,
            );
            map.insert(
                "not_before".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#not_before,
                )
                .await,
            );
            map.insert(
                "recovery_level".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#recovery_level,
                )
                .await,
            );
            map.insert(
                "updated".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#updated,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for CertificateCertificateAttribute {
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
                    r#created: {
                        let field_value = match fields_map.get("created") {
                            Some(value) => value,
                            None => bail!("Missing field 'created' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#expires: {
                        let field_value = match fields_map.get("expires") {
                            Some(value) => value,
                            None => bail!("Missing field 'expires' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#not_before: {
                        let field_value = match fields_map.get("not_before") {
                            Some(value) => value,
                            None => bail!("Missing field 'not_before' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#recovery_level: {
                        let field_value = match fields_map.get("recovery_level") {
                            Some(value) => value,
                            None => bail!("Missing field 'recovery_level' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#updated: {
                        let field_value = match fields_map.get("updated") {
                            Some(value) => value,
                            None => bail!("Missing field 'updated' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
