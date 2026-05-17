#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetEmailIdentityDkimSigningAttribute {
    /// [Easy DKIM] The key length of the DKIM key pair in use.
    #[builder(into)]
    #[serde(rename = "currentSigningKeyLength")]
    pub r#current_signing_key_length: String,
    #[builder(into)]
    #[serde(rename = "domainSigningPrivateKey")]
    pub r#domain_signing_private_key: String,
    #[builder(into)]
    #[serde(rename = "domainSigningSelector")]
    pub r#domain_signing_selector: String,
    /// [Easy DKIM] The last time a key pair was generated for this identity.
    #[builder(into)]
    #[serde(rename = "lastKeyGenerationTimestamp")]
    pub r#last_key_generation_timestamp: String,
    /// [Easy DKIM] The key length of the future DKIM key pair to be generated. This can be changed at most once per day.
    #[builder(into)]
    #[serde(rename = "nextSigningKeyLength")]
    pub r#next_signing_key_length: String,
    /// A string that indicates how DKIM was configured for the identity. `AWS_SES` indicates that DKIM was configured for the identity by using Easy DKIM. `EXTERNAL` indicates that DKIM was configured for the identity by using Bring Your Own DKIM (BYODKIM).
    #[builder(into)]
    #[serde(rename = "signingAttributesOrigin")]
    pub r#signing_attributes_origin: String,
    /// Describes whether or not Amazon SES has successfully located the DKIM records in the DNS records for the domain. See the [AWS SES API v2 Reference](https://docs.aws.amazon.com/ses/latest/APIReference-V2/API_DkimAttributes.html#SES-Type-DkimAttributes-Status) for supported statuses.
    #[builder(into)]
    #[serde(rename = "status")]
    pub r#status: String,
    /// If you used Easy DKIM to configure DKIM authentication for the domain, then this object contains a set of unique strings that you use to create a set of CNAME records that you add to the DNS configuration for your domain. When Amazon SES detects these records in the DNS configuration for your domain, the DKIM authentication process is complete. If you configured DKIM authentication for the domain by providing your own public-private key pair, then this object contains the selector for the public key.
    #[builder(into)]
    #[serde(rename = "tokens")]
    pub r#tokens: Vec<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetEmailIdentityDkimSigningAttribute {
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
                "current_signing_key_length".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#current_signing_key_length,
                )
                .await,
            );
            map.insert(
                "domain_signing_private_key".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#domain_signing_private_key,
                )
                .await,
            );
            map.insert(
                "domain_signing_selector".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#domain_signing_selector,
                )
                .await,
            );
            map.insert(
                "last_key_generation_timestamp".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#last_key_generation_timestamp,
                )
                .await,
            );
            map.insert(
                "next_signing_key_length".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#next_signing_key_length,
                )
                .await,
            );
            map.insert(
                "signing_attributes_origin".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#signing_attributes_origin,
                )
                .await,
            );
            map.insert(
                "status".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#status,
                )
                .await,
            );
            map.insert(
                "tokens".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#tokens,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetEmailIdentityDkimSigningAttribute {
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
                    r#current_signing_key_length: {
                        let field_value = match fields_map.get("current_signing_key_length") {
                            Some(value) => value,
                            None => bail!("Missing field 'current_signing_key_length' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#domain_signing_private_key: {
                        let field_value = match fields_map.get("domain_signing_private_key") {
                            Some(value) => value,
                            None => bail!("Missing field 'domain_signing_private_key' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#domain_signing_selector: {
                        let field_value = match fields_map.get("domain_signing_selector") {
                            Some(value) => value,
                            None => bail!("Missing field 'domain_signing_selector' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#last_key_generation_timestamp: {
                        let field_value = match fields_map.get("last_key_generation_timestamp") {
                            Some(value) => value,
                            None => bail!("Missing field 'last_key_generation_timestamp' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#next_signing_key_length: {
                        let field_value = match fields_map.get("next_signing_key_length") {
                            Some(value) => value,
                            None => bail!("Missing field 'next_signing_key_length' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#signing_attributes_origin: {
                        let field_value = match fields_map.get("signing_attributes_origin") {
                            Some(value) => value,
                            None => bail!("Missing field 'signing_attributes_origin' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#status: {
                        let field_value = match fields_map.get("status") {
                            Some(value) => value,
                            None => bail!("Missing field 'status' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#tokens: {
                        let field_value = match fields_map.get("tokens") {
                            Some(value) => value,
                            None => bail!("Missing field 'tokens' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
