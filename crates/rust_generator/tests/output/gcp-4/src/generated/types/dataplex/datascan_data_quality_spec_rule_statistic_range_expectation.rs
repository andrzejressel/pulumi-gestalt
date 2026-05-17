#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DatascanDataQualitySpecRuleStatisticRangeExpectation {
    /// The maximum column statistic value allowed for a row to pass this validation.
    /// At least one of minValue and maxValue need to be provided.
    #[builder(into)]
    #[serde(rename = "maxValue")]
    pub r#max_value: Option<String>,
    /// The minimum column statistic value allowed for a row to pass this validation.
    /// At least one of minValue and maxValue need to be provided.
    #[builder(into)]
    #[serde(rename = "minValue")]
    pub r#min_value: Option<String>,
    /// column statistics.
    /// Possible values are: `STATISTIC_UNDEFINED`, `MEAN`, `MIN`, `MAX`.
    #[builder(into)]
    #[serde(rename = "statistic")]
    pub r#statistic: String,
    /// Whether column statistic needs to be strictly lesser than ('<') the maximum, or if equality is allowed.
    /// Only relevant if a maxValue has been defined. Default = false.
    #[builder(into)]
    #[serde(rename = "strictMaxEnabled")]
    pub r#strict_max_enabled: Option<bool>,
    /// Whether column statistic needs to be strictly greater than ('>') the minimum, or if equality is allowed.
    /// Only relevant if a minValue has been defined. Default = false.
    #[builder(into)]
    #[serde(rename = "strictMinEnabled")]
    pub r#strict_min_enabled: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for DatascanDataQualitySpecRuleStatisticRangeExpectation {
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
                "max_value".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#max_value,
                )
                .await,
            );
            map.insert(
                "min_value".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#min_value,
                )
                .await,
            );
            map.insert(
                "statistic".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#statistic,
                )
                .await,
            );
            map.insert(
                "strict_max_enabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#strict_max_enabled,
                )
                .await,
            );
            map.insert(
                "strict_min_enabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#strict_min_enabled,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for DatascanDataQualitySpecRuleStatisticRangeExpectation {
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
                    r#max_value: {
                        let field_value = match fields_map.get("max_value") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_value' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#min_value: {
                        let field_value = match fields_map.get("min_value") {
                            Some(value) => value,
                            None => bail!("Missing field 'min_value' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#statistic: {
                        let field_value = match fields_map.get("statistic") {
                            Some(value) => value,
                            None => bail!("Missing field 'statistic' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#strict_max_enabled: {
                        let field_value = match fields_map.get("strict_max_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'strict_max_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#strict_min_enabled: {
                        let field_value = match fields_map.get("strict_min_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'strict_min_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
