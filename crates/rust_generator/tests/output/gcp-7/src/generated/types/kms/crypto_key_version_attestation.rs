#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct CryptoKeyVersionAttestation {
    /// The certificate chains needed to validate the attestation
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "certChains")]
    pub r#cert_chains: Option<Box<super::super::types::kms::CryptoKeyVersionAttestationCertChains>>,
    /// (Output)
    /// The attestation data provided by the HSM when the key operation was performed.
    #[builder(into)]
    #[serde(rename = "content")]
    pub r#content: Option<String>,
    /// ExternalProtectionLevelOptions stores a group of additional fields for configuring a CryptoKeyVersion that are specific to the EXTERNAL protection level and EXTERNAL_VPC protection levels.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "externalProtectionLevelOptions")]
    pub r#external_protection_level_options: Option<Box<super::super::types::kms::CryptoKeyVersionAttestationExternalProtectionLevelOptions>>,
    /// (Output)
    /// The format of the attestation data.
    #[builder(into)]
    #[serde(rename = "format")]
    pub r#format: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for CryptoKeyVersionAttestation {
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
                "cert_chains".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#cert_chains,
                )
                .await,
            );
            map.insert(
                "content".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#content,
                )
                .await,
            );
            map.insert(
                "external_protection_level_options".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#external_protection_level_options,
                )
                .await,
            );
            map.insert(
                "format".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#format,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for CryptoKeyVersionAttestation {
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
                    r#cert_chains: {
                        let field_value = match fields_map.get("cert_chains") {
                            Some(value) => value,
                            None => bail!("Missing field 'cert_chains' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#content: {
                        let field_value = match fields_map.get("content") {
                            Some(value) => value,
                            None => bail!("Missing field 'content' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#external_protection_level_options: {
                        let field_value = match fields_map.get("external_protection_level_options") {
                            Some(value) => value,
                            None => bail!("Missing field 'external_protection_level_options' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#format: {
                        let field_value = match fields_map.get("format") {
                            Some(value) => value,
                            None => bail!("Missing field 'format' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
