#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PreventionDeidentifyTemplateDeidentifyConfigInfoTypeTransformationsTransformationPrimitiveTransformationDateShiftConfig {
    /// Points to the field that contains the context, for example, an entity id.
    /// If set, must also set cryptoKey. If set, shift will be consistent for the given context.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "context")]
    pub r#context: Option<Box<super::super::types::dataloss::PreventionDeidentifyTemplateDeidentifyConfigInfoTypeTransformationsTransformationPrimitiveTransformationDateShiftConfigContext>>,
    /// Causes the shift to be computed based on this key and the context. This results in the same shift for the same context and cryptoKey. If set, must also set context. Can only be applied to table items.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "cryptoKey")]
    pub r#crypto_key: Option<Box<super::super::types::dataloss::PreventionDeidentifyTemplateDeidentifyConfigInfoTypeTransformationsTransformationPrimitiveTransformationDateShiftConfigCryptoKey>>,
    /// For example, -5 means shift date to at most 5 days back in the past.
    #[builder(into)]
    #[serde(rename = "lowerBoundDays")]
    pub r#lower_bound_days: i32,
    /// Range of shift in days. Actual shift will be selected at random within this range (inclusive ends). Negative means shift to earlier in time. Must not be more than 365250 days (1000 years) each direction.
    /// For example, 3 means shift date to at most 3 days into the future.
    #[builder(into)]
    #[serde(rename = "upperBoundDays")]
    pub r#upper_bound_days: i32,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for PreventionDeidentifyTemplateDeidentifyConfigInfoTypeTransformationsTransformationPrimitiveTransformationDateShiftConfig {
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
                "context".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#context,
                )
                .await,
            );
            map.insert(
                "crypto_key".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#crypto_key,
                )
                .await,
            );
            map.insert(
                "lower_bound_days".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#lower_bound_days,
                )
                .await,
            );
            map.insert(
                "upper_bound_days".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#upper_bound_days,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for PreventionDeidentifyTemplateDeidentifyConfigInfoTypeTransformationsTransformationPrimitiveTransformationDateShiftConfig {
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
                    r#context: {
                        let field_value = match fields_map.get("context") {
                            Some(value) => value,
                            None => bail!("Missing field 'context' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#crypto_key: {
                        let field_value = match fields_map.get("crypto_key") {
                            Some(value) => value,
                            None => bail!("Missing field 'crypto_key' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#lower_bound_days: {
                        let field_value = match fields_map.get("lower_bound_days") {
                            Some(value) => value,
                            None => bail!("Missing field 'lower_bound_days' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#upper_bound_days: {
                        let field_value = match fields_map.get("upper_bound_days") {
                            Some(value) => value,
                            None => bail!("Missing field 'upper_bound_days' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
