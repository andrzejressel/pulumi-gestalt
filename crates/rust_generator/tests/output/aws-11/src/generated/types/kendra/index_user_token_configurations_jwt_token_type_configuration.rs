#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct IndexUserTokenConfigurationsJwtTokenTypeConfiguration {
    /// The regular expression that identifies the claim. Minimum length of 1. Maximum length of 100.
    #[builder(into)]
    pub r#claim_regex: Option<String>,
    /// The group attribute field. Minimum length of 1. Maximum length of 100.
    #[builder(into)]
    pub r#group_attribute_field: Option<String>,
    /// The issuer of the token. Minimum length of 1. Maximum length of 65.
    #[builder(into)]
    pub r#issuer: Option<String>,
    /// The location of the key. Valid values are `URL` or `SECRET_MANAGER`
    #[builder(into)]
    pub r#key_location: String,
    /// The Amazon Resource Name (ARN) of the secret.
    #[builder(into)]
    pub r#secrets_manager_arn: Option<String>,
    /// The signing key URL. Valid pattern is `^(https?|ftp|file):\/\/([^\s]*)`
    #[builder(into)]
    pub r#url: Option<String>,
    /// The user name attribute field. Minimum length of 1. Maximum length of 100.
    #[builder(into)]
    pub r#user_name_attribute_field: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for IndexUserTokenConfigurationsJwtTokenTypeConfiguration {
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
                    "claimRegex",
                    &self.r#claim_regex,
                ),
                to_pulumi_object_field(
                    "groupAttributeField",
                    &self.r#group_attribute_field,
                ),
                to_pulumi_object_field(
                    "issuer",
                    &self.r#issuer,
                ),
                to_pulumi_object_field(
                    "keyLocation",
                    &self.r#key_location,
                ),
                to_pulumi_object_field(
                    "secretsManagerArn",
                    &self.r#secrets_manager_arn,
                ),
                to_pulumi_object_field(
                    "url",
                    &self.r#url,
                ),
                to_pulumi_object_field(
                    "userNameAttributeField",
                    &self.r#user_name_attribute_field,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for IndexUserTokenConfigurationsJwtTokenTypeConfiguration {
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
                        let field_value = match fields_map.get("claimRegex") {
                            Some(value) => value,
                            None => bail!("Missing field 'claimRegex' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#group_attribute_field: {
                        let field_value = match fields_map.get("groupAttributeField") {
                            Some(value) => value,
                            None => bail!("Missing field 'groupAttributeField' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("keyLocation") {
                            Some(value) => value,
                            None => bail!("Missing field 'keyLocation' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#secrets_manager_arn: {
                        let field_value = match fields_map.get("secretsManagerArn") {
                            Some(value) => value,
                            None => bail!("Missing field 'secretsManagerArn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("userNameAttributeField") {
                            Some(value) => value,
                            None => bail!("Missing field 'userNameAttributeField' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
