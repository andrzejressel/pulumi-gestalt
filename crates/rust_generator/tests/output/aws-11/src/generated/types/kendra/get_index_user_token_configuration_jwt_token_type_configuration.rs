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
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "claim_regex".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#claim_regex,
                )
                .await,
            );
            map.insert(
                "group_attribute_field".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#group_attribute_field,
                )
                .await,
            );
            map.insert(
                "issuer".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#issuer,
                )
                .await,
            );
            map.insert(
                "key_location".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#key_location,
                )
                .await,
            );
            map.insert(
                "secrets_manager_arn".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#secrets_manager_arn,
                )
                .await,
            );
            map.insert(
                "url".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#url,
                )
                .await,
            );
            map.insert(
                "user_name_attribute_field".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#user_name_attribute_field,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetIndexUserTokenConfigurationJwtTokenTypeConfiguration {
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
                    r#claim_regex: {
                        let field_value = match fields_map.get("claim_regex") {
                            Some(value) => value,
                            None => bail!("Missing field 'claim_regex' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#group_attribute_field: {
                        let field_value = match fields_map.get("group_attribute_field") {
                            Some(value) => value,
                            None => bail!("Missing field 'group_attribute_field' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#issuer: {
                        let field_value = match fields_map.get("issuer") {
                            Some(value) => value,
                            None => bail!("Missing field 'issuer' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#key_location: {
                        let field_value = match fields_map.get("key_location") {
                            Some(value) => value,
                            None => bail!("Missing field 'key_location' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#secrets_manager_arn: {
                        let field_value = match fields_map.get("secrets_manager_arn") {
                            Some(value) => value,
                            None => bail!("Missing field 'secrets_manager_arn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#url: {
                        let field_value = match fields_map.get("url") {
                            Some(value) => value,
                            None => bail!("Missing field 'url' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#user_name_attribute_field: {
                        let field_value = match fields_map.get("user_name_attribute_field") {
                            Some(value) => value,
                            None => bail!("Missing field 'user_name_attribute_field' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
