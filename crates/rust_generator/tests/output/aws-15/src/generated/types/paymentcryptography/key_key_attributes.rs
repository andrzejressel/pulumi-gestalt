#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct KeyKeyAttributes {
    /// Key algorithm to be use during creation of an AWS Payment Cryptography key.
    #[builder(into)]
    #[serde(rename = "keyAlgorithm")]
    pub r#key_algorithm: String,
    /// Type of AWS Payment Cryptography key to create.
    #[builder(into)]
    #[serde(rename = "keyClass")]
    pub r#key_class: String,
    /// List of cryptographic operations that you can perform using the key.
    #[builder(into)]
    #[serde(rename = "keyModesOfUse")]
    pub r#key_modes_of_use: Option<Box<super::super::types::paymentcryptography::KeyKeyAttributesKeyModesOfUse>>,
    /// Cryptographic usage of an AWS Payment Cryptography key as defined in section A.5.2 of the TR-31 spec.
    #[builder(into)]
    #[serde(rename = "keyUsage")]
    pub r#key_usage: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for KeyKeyAttributes {
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
                "key_algorithm".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#key_algorithm,
                )
                .await,
            );
            map.insert(
                "key_class".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#key_class,
                )
                .await,
            );
            map.insert(
                "key_modes_of_use".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#key_modes_of_use,
                )
                .await,
            );
            map.insert(
                "key_usage".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#key_usage,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for KeyKeyAttributes {
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
                    r#key_algorithm: {
                        let field_value = match fields_map.get("key_algorithm") {
                            Some(value) => value,
                            None => bail!("Missing field 'key_algorithm' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#key_class: {
                        let field_value = match fields_map.get("key_class") {
                            Some(value) => value,
                            None => bail!("Missing field 'key_class' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#key_modes_of_use: {
                        let field_value = match fields_map.get("key_modes_of_use") {
                            Some(value) => value,
                            None => bail!("Missing field 'key_modes_of_use' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#key_usage: {
                        let field_value = match fields_map.get("key_usage") {
                            Some(value) => value,
                            None => bail!("Missing field 'key_usage' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
