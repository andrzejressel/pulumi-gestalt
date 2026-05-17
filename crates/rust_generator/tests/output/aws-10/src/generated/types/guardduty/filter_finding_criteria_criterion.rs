#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FilterFindingCriteriaCriterion {
    /// List of string values to be evaluated.
    #[builder(into)]
    #[serde(rename = "equals")]
    pub r#equals: Option<Vec<String>>,
    /// The name of the field to be evaluated. The full list of field names can be found in [AWS documentation](https://docs.aws.amazon.com/guardduty/latest/ug/guardduty_filter-findings.html#filter_criteria).
    #[builder(into)]
    #[serde(rename = "field")]
    pub r#field: String,
    /// A value to be evaluated. Accepts either an integer or a date in [RFC 3339 format](https://tools.ietf.org/html/rfc3339#section-5.8).
    #[builder(into)]
    #[serde(rename = "greaterThan")]
    pub r#greater_than: Option<String>,
    /// A value to be evaluated. Accepts either an integer or a date in [RFC 3339 format](https://tools.ietf.org/html/rfc3339#section-5.8).
    #[builder(into)]
    #[serde(rename = "greaterThanOrEqual")]
    pub r#greater_than_or_equal: Option<String>,
    /// A value to be evaluated. Accepts either an integer or a date in [RFC 3339 format](https://tools.ietf.org/html/rfc3339#section-5.8).
    #[builder(into)]
    #[serde(rename = "lessThan")]
    pub r#less_than: Option<String>,
    /// A value to be evaluated. Accepts either an integer or a date in [RFC 3339 format](https://tools.ietf.org/html/rfc3339#section-5.8).
    #[builder(into)]
    #[serde(rename = "lessThanOrEqual")]
    pub r#less_than_or_equal: Option<String>,
    /// List of string values to be evaluated.
    #[builder(into)]
    #[serde(rename = "notEquals")]
    pub r#not_equals: Option<Vec<String>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for FilterFindingCriteriaCriterion {
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
                "equals".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#equals,
                )
                .await,
            );
            map.insert(
                "field".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#field,
                )
                .await,
            );
            map.insert(
                "greater_than".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#greater_than,
                )
                .await,
            );
            map.insert(
                "greater_than_or_equal".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#greater_than_or_equal,
                )
                .await,
            );
            map.insert(
                "less_than".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#less_than,
                )
                .await,
            );
            map.insert(
                "less_than_or_equal".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#less_than_or_equal,
                )
                .await,
            );
            map.insert(
                "not_equals".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#not_equals,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for FilterFindingCriteriaCriterion {
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
                    r#equals: {
                        let field_value = match fields_map.get("equals") {
                            Some(value) => value,
                            None => bail!("Missing field 'equals' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#field: {
                        let field_value = match fields_map.get("field") {
                            Some(value) => value,
                            None => bail!("Missing field 'field' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#greater_than: {
                        let field_value = match fields_map.get("greater_than") {
                            Some(value) => value,
                            None => bail!("Missing field 'greater_than' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#greater_than_or_equal: {
                        let field_value = match fields_map.get("greater_than_or_equal") {
                            Some(value) => value,
                            None => bail!("Missing field 'greater_than_or_equal' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#less_than: {
                        let field_value = match fields_map.get("less_than") {
                            Some(value) => value,
                            None => bail!("Missing field 'less_than' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#less_than_or_equal: {
                        let field_value = match fields_map.get("less_than_or_equal") {
                            Some(value) => value,
                            None => bail!("Missing field 'less_than_or_equal' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#not_equals: {
                        let field_value = match fields_map.get("not_equals") {
                            Some(value) => value,
                            None => bail!("Missing field 'not_equals' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
