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
    > + Send {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "algorithm",
                    &self.r#algorithm,
                ),
                to_pulumi_object_field(
                    "memory_cost",
                    &self.r#memory_cost,
                ),
                to_pulumi_object_field(
                    "rounds",
                    &self.r#rounds,
                ),
                to_pulumi_object_field(
                    "salt_separator",
                    &self.r#salt_separator,
                ),
                to_pulumi_object_field(
                    "signer_key",
                    &self.r#signer_key,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
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
