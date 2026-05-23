#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct TrustedTokenIssuerTrustedTokenIssuerConfigurationOidcJwtConfiguration {
    /// Specifies the path of the source attribute in the JWT from the trusted token issuer.
    #[builder(into)]
    pub r#claim_attribute_path: String,
    /// Specifies path of the destination attribute in a JWT from IAM Identity Center. The attribute mapped by this JMESPath expression is compared against the attribute mapped by `claim_attribute_path` when a trusted token issuer token is exchanged for an IAM Identity Center token.
    #[builder(into)]
    pub r#identity_store_attribute_path: String,
    /// Specifies the URL that IAM Identity Center uses for OpenID Discovery. OpenID Discovery is used to obtain the information required to verify the tokens that the trusted token issuer generates.
    #[builder(into)]
    pub r#issuer_url: String,
    /// The method that the trusted token issuer can use to retrieve the JSON Web Key Set used to verify a JWT. Valid values are `OPEN_ID_DISCOVERY`
    #[builder(into)]
    pub r#jwks_retrieval_option: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for TrustedTokenIssuerTrustedTokenIssuerConfigurationOidcJwtConfiguration {
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
                    "claimAttributePath",
                    &self.r#claim_attribute_path,
                ),
                to_pulumi_object_field(
                    "identityStoreAttributePath",
                    &self.r#identity_store_attribute_path,
                ),
                to_pulumi_object_field(
                    "issuerUrl",
                    &self.r#issuer_url,
                ),
                to_pulumi_object_field(
                    "jwksRetrievalOption",
                    &self.r#jwks_retrieval_option,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for TrustedTokenIssuerTrustedTokenIssuerConfigurationOidcJwtConfiguration {
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
                    r#claim_attribute_path: {
                        let field_value = match fields_map.get("claimAttributePath") {
                            Some(value) => value,
                            None => bail!("Missing field 'claimAttributePath' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#identity_store_attribute_path: {
                        let field_value = match fields_map.get("identityStoreAttributePath") {
                            Some(value) => value,
                            None => bail!("Missing field 'identityStoreAttributePath' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#issuer_url: {
                        let field_value = match fields_map.get("issuerUrl") {
                            Some(value) => value,
                            None => bail!("Missing field 'issuerUrl' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#jwks_retrieval_option: {
                        let field_value = match fields_map.get("jwksRetrievalOption") {
                            Some(value) => value,
                            None => bail!("Missing field 'jwksRetrievalOption' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
