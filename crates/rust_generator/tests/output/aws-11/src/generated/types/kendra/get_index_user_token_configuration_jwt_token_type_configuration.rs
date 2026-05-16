#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetIndexUserTokenConfigurationJwtTokenTypeConfiguration {
    /// Regular expression that identifies the claim.
    #[builder(into)]
    #[serde(rename = "claimRegex")]
    pub r#claim_regex: String,
    /// The group attribute field.
    #[builder(into)]
    #[serde(rename = "groupAttributeField")]
    pub r#group_attribute_field: String,
    /// Issuer of the token.
    #[builder(into)]
    #[serde(rename = "issuer")]
    pub r#issuer: String,
    /// Location of the key. Valid values are `URL` or `SECRET_MANAGER`
    #[builder(into)]
    #[serde(rename = "keyLocation")]
    pub r#key_location: String,
    /// ARN of the secret.
    #[builder(into)]
    #[serde(rename = "secretsManagerArn")]
    pub r#secrets_manager_arn: String,
    /// Signing key URL.
    #[builder(into)]
    #[serde(rename = "url")]
    pub r#url: String,
    /// The user name attribute field.
    #[builder(into)]
    #[serde(rename = "userNameAttributeField")]
    pub r#user_name_attribute_field: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetIndexUserTokenConfigurationJwtTokenTypeConfiguration {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("claim_regex".to_string(), self.r#claim_regex.to_pulumi_value().await);
            map.insert("group_attribute_field".to_string(), self.r#group_attribute_field.to_pulumi_value().await);
            map.insert("issuer".to_string(), self.r#issuer.to_pulumi_value().await);
            map.insert("key_location".to_string(), self.r#key_location.to_pulumi_value().await);
            map.insert("secrets_manager_arn".to_string(), self.r#secrets_manager_arn.to_pulumi_value().await);
            map.insert("url".to_string(), self.r#url.to_pulumi_value().await);
            map.insert("user_name_attribute_field".to_string(), self.r#user_name_attribute_field.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetIndexUserTokenConfigurationJwtTokenTypeConfiguration {
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
                    r#claim_regex: {
                        let field_value = match fields_map.get("claim_regex") {
                            Some(value) => value,
                            None => bail!("Missing field 'claim_regex' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#group_attribute_field: {
                        let field_value = match fields_map.get("group_attribute_field") {
                            Some(value) => value,
                            None => bail!("Missing field 'group_attribute_field' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#issuer: {
                        let field_value = match fields_map.get("issuer") {
                            Some(value) => value,
                            None => bail!("Missing field 'issuer' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#key_location: {
                        let field_value = match fields_map.get("key_location") {
                            Some(value) => value,
                            None => bail!("Missing field 'key_location' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#secrets_manager_arn: {
                        let field_value = match fields_map.get("secrets_manager_arn") {
                            Some(value) => value,
                            None => bail!("Missing field 'secrets_manager_arn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#url: {
                        let field_value = match fields_map.get("url") {
                            Some(value) => value,
                            None => bail!("Missing field 'url' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#user_name_attribute_field: {
                        let field_value = match fields_map.get("user_name_attribute_field") {
                            Some(value) => value,
                            None => bail!("Missing field 'user_name_attribute_field' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
