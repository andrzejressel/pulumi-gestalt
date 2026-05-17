#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct CaPoolIssuancePolicy {
    /// IssuanceModes specifies the allowed ways in which Certificates may be requested from this CaPool.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "allowedIssuanceModes")]
    pub r#allowed_issuance_modes: Option<Box<super::super::types::certificateauthority::CaPoolIssuancePolicyAllowedIssuanceModes>>,
    /// If any AllowedKeyType is specified, then the certificate request's public key must match one of the key types listed here.
    /// Otherwise, any key may be used.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "allowedKeyTypes")]
    pub r#allowed_key_types: Option<Vec<super::super::types::certificateauthority::CaPoolIssuancePolicyAllowedKeyType>>,
    /// A set of X.509 values that will be applied to all certificates issued through this CaPool. If a certificate request
    /// includes conflicting values for the same properties, they will be overwritten by the values defined here. If a certificate
    /// request uses a CertificateTemplate that defines conflicting predefinedValues for the same properties, the certificate
    /// issuance request will fail.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "baselineValues")]
    pub r#baseline_values: Option<Box<super::super::types::certificateauthority::CaPoolIssuancePolicyBaselineValues>>,
    /// Describes constraints on identities that may appear in Certificates issued through this CaPool.
    /// If this is omitted, then this CaPool will not add restrictions on a certificate's identity.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "identityConstraints")]
    pub r#identity_constraints: Option<Box<super::super::types::certificateauthority::CaPoolIssuancePolicyIdentityConstraints>>,
    /// The maximum lifetime allowed for issued Certificates. Note that if the issuing CertificateAuthority
    /// expires before a Certificate's requested maximumLifetime, the effective lifetime will be explicitly truncated to match it.
    #[builder(into)]
    #[serde(rename = "maximumLifetime")]
    pub r#maximum_lifetime: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for CaPoolIssuancePolicy {
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
                "allowed_issuance_modes".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#allowed_issuance_modes,
                )
                .await,
            );
            map.insert(
                "allowed_key_types".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#allowed_key_types,
                )
                .await,
            );
            map.insert(
                "baseline_values".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#baseline_values,
                )
                .await,
            );
            map.insert(
                "identity_constraints".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#identity_constraints,
                )
                .await,
            );
            map.insert(
                "maximum_lifetime".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#maximum_lifetime,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for CaPoolIssuancePolicy {
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
                    r#allowed_issuance_modes: {
                        let field_value = match fields_map.get("allowed_issuance_modes") {
                            Some(value) => value,
                            None => bail!("Missing field 'allowed_issuance_modes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#allowed_key_types: {
                        let field_value = match fields_map.get("allowed_key_types") {
                            Some(value) => value,
                            None => bail!("Missing field 'allowed_key_types' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#baseline_values: {
                        let field_value = match fields_map.get("baseline_values") {
                            Some(value) => value,
                            None => bail!("Missing field 'baseline_values' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#identity_constraints: {
                        let field_value = match fields_map.get("identity_constraints") {
                            Some(value) => value,
                            None => bail!("Missing field 'identity_constraints' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#maximum_lifetime: {
                        let field_value = match fields_map.get("maximum_lifetime") {
                            Some(value) => value,
                            None => bail!("Missing field 'maximum_lifetime' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
