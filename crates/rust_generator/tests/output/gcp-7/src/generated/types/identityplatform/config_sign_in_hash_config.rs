#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ConfigSignInHashConfig {
    /// Different password hash algorithms used in Identity Toolkit.
    #[builder(into)]
    #[serde(rename = "algorithm")]
    pub r#algorithm: Option<String>,
    /// Memory cost for hash calculation. Used by scrypt and other similar password derivation algorithms. See https://tools.ietf.org/html/rfc7914 for explanation of field.
    #[builder(into)]
    #[serde(rename = "memoryCost")]
    pub r#memory_cost: Option<i32>,
    /// How many rounds for hash calculation. Used by scrypt and other similar password derivation algorithms.
    #[builder(into)]
    #[serde(rename = "rounds")]
    pub r#rounds: Option<i32>,
    /// Non-printable character to be inserted between the salt and plain text password in base64.
    #[builder(into)]
    #[serde(rename = "saltSeparator")]
    pub r#salt_separator: Option<String>,
    /// Signer key in base64.
    #[builder(into)]
    #[serde(rename = "signerKey")]
    pub r#signer_key: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ConfigSignInHashConfig {
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
                "algorithm".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#algorithm,
                )
                .await,
            );
            map.insert(
                "memory_cost".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#memory_cost,
                )
                .await,
            );
            map.insert(
                "rounds".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#rounds,
                )
                .await,
            );
            map.insert(
                "salt_separator".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#salt_separator,
                )
                .await,
            );
            map.insert(
                "signer_key".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#signer_key,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ConfigSignInHashConfig {
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
                    r#algorithm: {
                        let field_value = match fields_map.get("algorithm") {
                            Some(value) => value,
                            None => bail!("Missing field 'algorithm' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#memory_cost: {
                        let field_value = match fields_map.get("memory_cost") {
                            Some(value) => value,
                            None => bail!("Missing field 'memory_cost' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#rounds: {
                        let field_value = match fields_map.get("rounds") {
                            Some(value) => value,
                            None => bail!("Missing field 'rounds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#salt_separator: {
                        let field_value = match fields_map.get("salt_separator") {
                            Some(value) => value,
                            None => bail!("Missing field 'salt_separator' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#signer_key: {
                        let field_value = match fields_map.get("signer_key") {
                            Some(value) => value,
                            None => bail!("Missing field 'signer_key' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
