#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ConnectionSslConfigAdditionalVariable {
    /// Boolean Value of configVariable.
    #[builder(into)]
    #[serde(rename = "booleanValue")]
    pub r#boolean_value: Option<bool>,
    /// Encryption key value of configVariable.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "encryptionKeyValue")]
    pub r#encryption_key_value: Option<Box<super::super::types::integrationconnectors::ConnectionSslConfigAdditionalVariableEncryptionKeyValue>>,
    /// Integer Value of configVariable.
    #[builder(into)]
    #[serde(rename = "integerValue")]
    pub r#integer_value: Option<i32>,
    /// Key for the configVariable
    #[builder(into)]
    #[serde(rename = "key")]
    pub r#key: String,
    /// Secret value of configVariable
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "secretValue")]
    pub r#secret_value: Option<Box<super::super::types::integrationconnectors::ConnectionSslConfigAdditionalVariableSecretValue>>,
    /// String Value of configVariabley.
    #[builder(into)]
    #[serde(rename = "stringValue")]
    pub r#string_value: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ConnectionSslConfigAdditionalVariable {
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
                "boolean_value".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#boolean_value,
                )
                .await,
            );
            map.insert(
                "encryption_key_value".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#encryption_key_value,
                )
                .await,
            );
            map.insert(
                "integer_value".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#integer_value,
                )
                .await,
            );
            map.insert(
                "key".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#key,
                )
                .await,
            );
            map.insert(
                "secret_value".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#secret_value,
                )
                .await,
            );
            map.insert(
                "string_value".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#string_value,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ConnectionSslConfigAdditionalVariable {
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
                    r#boolean_value: {
                        let field_value = match fields_map.get("boolean_value") {
                            Some(value) => value,
                            None => bail!("Missing field 'boolean_value' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#encryption_key_value: {
                        let field_value = match fields_map.get("encryption_key_value") {
                            Some(value) => value,
                            None => bail!("Missing field 'encryption_key_value' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#integer_value: {
                        let field_value = match fields_map.get("integer_value") {
                            Some(value) => value,
                            None => bail!("Missing field 'integer_value' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#key: {
                        let field_value = match fields_map.get("key") {
                            Some(value) => value,
                            None => bail!("Missing field 'key' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#secret_value: {
                        let field_value = match fields_map.get("secret_value") {
                            Some(value) => value,
                            None => bail!("Missing field 'secret_value' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#string_value: {
                        let field_value = match fields_map.get("string_value") {
                            Some(value) => value,
                            None => bail!("Missing field 'string_value' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
