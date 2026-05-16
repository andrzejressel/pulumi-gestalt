#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct TrustedTokenIssuerTrustedTokenIssuerConfigurationOidcJwtConfiguration {
    /// Specifies the path of the source attribute in the JWT from the trusted token issuer.
    #[builder(into)]
    #[serde(rename = "claimAttributePath")]
    pub r#claim_attribute_path: String,
    /// Specifies path of the destination attribute in a JWT from IAM Identity Center. The attribute mapped by this JMESPath expression is compared against the attribute mapped by `claim_attribute_path` when a trusted token issuer token is exchanged for an IAM Identity Center token.
    #[builder(into)]
    #[serde(rename = "identityStoreAttributePath")]
    pub r#identity_store_attribute_path: String,
    /// Specifies the URL that IAM Identity Center uses for OpenID Discovery. OpenID Discovery is used to obtain the information required to verify the tokens that the trusted token issuer generates.
    #[builder(into)]
    #[serde(rename = "issuerUrl")]
    pub r#issuer_url: String,
    /// The method that the trusted token issuer can use to retrieve the JSON Web Key Set used to verify a JWT. Valid values are `OPEN_ID_DISCOVERY`
    #[builder(into)]
    #[serde(rename = "jwksRetrievalOption")]
    pub r#jwks_retrieval_option: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for TrustedTokenIssuerTrustedTokenIssuerConfigurationOidcJwtConfiguration {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("claim_attribute_path".to_string(), self.r#claim_attribute_path.to_pulumi_value().await);
            map.insert("identity_store_attribute_path".to_string(), self.r#identity_store_attribute_path.to_pulumi_value().await);
            map.insert("issuer_url".to_string(), self.r#issuer_url.to_pulumi_value().await);
            map.insert("jwks_retrieval_option".to_string(), self.r#jwks_retrieval_option.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for TrustedTokenIssuerTrustedTokenIssuerConfigurationOidcJwtConfiguration {
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
                    r#claim_attribute_path: {
                        let field_value = match fields_map.get("claim_attribute_path") {
                            Some(value) => value,
                            None => bail!("Missing field 'claim_attribute_path' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#identity_store_attribute_path: {
                        let field_value = match fields_map.get("identity_store_attribute_path") {
                            Some(value) => value,
                            None => bail!("Missing field 'identity_store_attribute_path' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#issuer_url: {
                        let field_value = match fields_map.get("issuer_url") {
                            Some(value) => value,
                            None => bail!("Missing field 'issuer_url' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#jwks_retrieval_option: {
                        let field_value = match fields_map.get("jwks_retrieval_option") {
                            Some(value) => value,
                            None => bail!("Missing field 'jwks_retrieval_option' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
