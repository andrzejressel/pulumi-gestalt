#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct EmailIdentityDkimSigningAttributes {
    /// [Easy DKIM] The key length of the DKIM key pair in use.
    #[builder(into)]
    #[serde(rename = "currentSigningKeyLength")]
    pub r#current_signing_key_length: Option<String>,
    /// [Bring Your Own DKIM] A private key that's used to generate a DKIM signature. The private key must use 1024 or 2048-bit RSA encryption, and must be encoded using base64 encoding.
    /// 
    /// > **NOTE:** You have to delete the first and last lines ('-----BEGIN PRIVATE KEY-----' and '-----END PRIVATE KEY-----', respectively) of the generated private key. Additionally, you have to remove the line breaks in the generated private key. The resulting value is a string of characters with no spaces or line breaks.
    #[builder(into)]
    #[serde(rename = "domainSigningPrivateKey")]
    pub r#domain_signing_private_key: Option<String>,
    /// [Bring Your Own DKIM] A string that's used to identify a public key in the DNS configuration for a domain.
    #[builder(into)]
    #[serde(rename = "domainSigningSelector")]
    pub r#domain_signing_selector: Option<String>,
    /// [Easy DKIM] The last time a key pair was generated for this identity.
    #[builder(into)]
    #[serde(rename = "lastKeyGenerationTimestamp")]
    pub r#last_key_generation_timestamp: Option<String>,
    /// [Easy DKIM] The key length of the future DKIM key pair to be generated. This can be changed at most once per day. Valid values: `RSA_1024_BIT`, `RSA_2048_BIT`.
    #[builder(into)]
    #[serde(rename = "nextSigningKeyLength")]
    pub r#next_signing_key_length: Option<String>,
    /// A string that indicates how DKIM was configured for the identity. `AWS_SES` indicates that DKIM was configured for the identity by using Easy DKIM. `EXTERNAL` indicates that DKIM was configured for the identity by using Bring Your Own DKIM (BYODKIM).
    #[builder(into)]
    #[serde(rename = "signingAttributesOrigin")]
    pub r#signing_attributes_origin: Option<String>,
    /// Describes whether or not Amazon SES has successfully located the DKIM records in the DNS records for the domain. See the [AWS SES API v2 Reference](https://docs.aws.amazon.com/ses/latest/APIReference-V2/API_DkimAttributes.html#SES-Type-DkimAttributes-Status) for supported statuses.
    #[builder(into)]
    #[serde(rename = "status")]
    pub r#status: Option<String>,
    /// If you used Easy DKIM to configure DKIM authentication for the domain, then this object contains a set of unique strings that you use to create a set of CNAME records that you add to the DNS configuration for your domain. When Amazon SES detects these records in the DNS configuration for your domain, the DKIM authentication process is complete. If you configured DKIM authentication for the domain by providing your own public-private key pair, then this object contains the selector for the public key.
    #[builder(into)]
    #[serde(rename = "tokens")]
    pub r#tokens: Option<Vec<String>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for EmailIdentityDkimSigningAttributes {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("current_signing_key_length".to_string(), self.r#current_signing_key_length.to_pulumi_value().await);
            map.insert("domain_signing_private_key".to_string(), self.r#domain_signing_private_key.to_pulumi_value().await);
            map.insert("domain_signing_selector".to_string(), self.r#domain_signing_selector.to_pulumi_value().await);
            map.insert("last_key_generation_timestamp".to_string(), self.r#last_key_generation_timestamp.to_pulumi_value().await);
            map.insert("next_signing_key_length".to_string(), self.r#next_signing_key_length.to_pulumi_value().await);
            map.insert("signing_attributes_origin".to_string(), self.r#signing_attributes_origin.to_pulumi_value().await);
            map.insert("status".to_string(), self.r#status.to_pulumi_value().await);
            map.insert("tokens".to_string(), self.r#tokens.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for EmailIdentityDkimSigningAttributes {
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
                    r#current_signing_key_length: {
                        let field_value = match fields_map.get("current_signing_key_length") {
                            Some(value) => value,
                            None => bail!("Missing field 'current_signing_key_length' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#domain_signing_private_key: {
                        let field_value = match fields_map.get("domain_signing_private_key") {
                            Some(value) => value,
                            None => bail!("Missing field 'domain_signing_private_key' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#domain_signing_selector: {
                        let field_value = match fields_map.get("domain_signing_selector") {
                            Some(value) => value,
                            None => bail!("Missing field 'domain_signing_selector' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#last_key_generation_timestamp: {
                        let field_value = match fields_map.get("last_key_generation_timestamp") {
                            Some(value) => value,
                            None => bail!("Missing field 'last_key_generation_timestamp' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#next_signing_key_length: {
                        let field_value = match fields_map.get("next_signing_key_length") {
                            Some(value) => value,
                            None => bail!("Missing field 'next_signing_key_length' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#signing_attributes_origin: {
                        let field_value = match fields_map.get("signing_attributes_origin") {
                            Some(value) => value,
                            None => bail!("Missing field 'signing_attributes_origin' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#status: {
                        let field_value = match fields_map.get("status") {
                            Some(value) => value,
                            None => bail!("Missing field 'status' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#tokens: {
                        let field_value = match fields_map.get("tokens") {
                            Some(value) => value,
                            None => bail!("Missing field 'tokens' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<String>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
