#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct CaPoolIssuancePolicyAllowedKeyTypeRsa {
    /// The maximum allowed RSA modulus size, in bits. If this is not set, or if set to zero, the
    /// service will not enforce an explicit upper bound on RSA modulus sizes.
    #[builder(into)]
    #[serde(rename = "maxModulusSize")]
    pub r#max_modulus_size: Option<String>,
    /// The minimum allowed RSA modulus size, in bits. If this is not set, or if set to zero, the
    /// service-level min RSA modulus size will continue to apply.
    #[builder(into)]
    #[serde(rename = "minModulusSize")]
    pub r#min_modulus_size: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for CaPoolIssuancePolicyAllowedKeyTypeRsa {
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
                "max_modulus_size".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#max_modulus_size,
                )
                .await,
            );
            map.insert(
                "min_modulus_size".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#min_modulus_size,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for CaPoolIssuancePolicyAllowedKeyTypeRsa {
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
                    r#max_modulus_size: {
                        let field_value = match fields_map.get("max_modulus_size") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_modulus_size' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#min_modulus_size: {
                        let field_value = match fields_map.get("min_modulus_size") {
                            Some(value) => value,
                            None => bail!("Missing field 'min_modulus_size' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
