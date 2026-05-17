#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DatascanDataQualitySpecRule {
    /// The unnested column which this rule is evaluated against.
    #[builder(into)]
    #[serde(rename = "column")]
    pub r#column: Option<String>,
    /// Description of the rule.
    /// The maximum length is 1,024 characters.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    /// The dimension a rule belongs to. Results are also aggregated at the dimension level. Supported dimensions are ["COMPLETENESS", "ACCURACY", "CONSISTENCY", "VALIDITY", "UNIQUENESS", "INTEGRITY"]
    #[builder(into)]
    #[serde(rename = "dimension")]
    pub r#dimension: String,
    /// Rows with null values will automatically fail a rule, unless ignoreNull is true. In that case, such null rows are trivially considered passing. Only applicable to ColumnMap rules.
    #[builder(into)]
    #[serde(rename = "ignoreNull")]
    pub r#ignore_null: Option<bool>,
    /// A mutable name for the rule.
    /// The name must contain only letters (a-z, A-Z), numbers (0-9), or hyphens (-).
    /// The maximum length is 63 characters.
    /// Must start with a letter.
    /// Must end with a number or a letter.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// ColumnMap rule which evaluates whether each column value is null.
    #[builder(into)]
    #[serde(rename = "nonNullExpectation")]
    pub r#non_null_expectation: Option<Box<super::super::types::dataplex::DatascanDataQualitySpecRuleNonNullExpectation>>,
    /// ColumnMap rule which evaluates whether each column value lies between a specified range.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "rangeExpectation")]
    pub r#range_expectation: Option<Box<super::super::types::dataplex::DatascanDataQualitySpecRuleRangeExpectation>>,
    /// ColumnMap rule which evaluates whether each column value matches a specified regex.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "regexExpectation")]
    pub r#regex_expectation: Option<Box<super::super::types::dataplex::DatascanDataQualitySpecRuleRegexExpectation>>,
    /// Table rule which evaluates whether each row passes the specified condition.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "rowConditionExpectation")]
    pub r#row_condition_expectation: Option<Box<super::super::types::dataplex::DatascanDataQualitySpecRuleRowConditionExpectation>>,
    /// ColumnMap rule which evaluates whether each column value is contained by a specified set.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "setExpectation")]
    pub r#set_expectation: Option<Box<super::super::types::dataplex::DatascanDataQualitySpecRuleSetExpectation>>,
    /// Table rule which evaluates whether any row matches invalid state.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "sqlAssertion")]
    pub r#sql_assertion: Option<Box<super::super::types::dataplex::DatascanDataQualitySpecRuleSqlAssertion>>,
    /// ColumnAggregate rule which evaluates whether the column aggregate statistic lies between a specified range.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "statisticRangeExpectation")]
    pub r#statistic_range_expectation: Option<Box<super::super::types::dataplex::DatascanDataQualitySpecRuleStatisticRangeExpectation>>,
    /// Table rule which evaluates whether the provided expression is true.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "tableConditionExpectation")]
    pub r#table_condition_expectation: Option<Box<super::super::types::dataplex::DatascanDataQualitySpecRuleTableConditionExpectation>>,
    /// The minimum ratio of passing_rows / total_rows required to pass this rule, with a range of [0.0, 1.0]. 0 indicates default value (i.e. 1.0).
    #[builder(into)]
    #[serde(rename = "threshold")]
    pub r#threshold: Option<f64>,
    /// Row-level rule which evaluates whether each column value is unique.
    #[builder(into)]
    #[serde(rename = "uniquenessExpectation")]
    pub r#uniqueness_expectation: Option<Box<super::super::types::dataplex::DatascanDataQualitySpecRuleUniquenessExpectation>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for DatascanDataQualitySpecRule {
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
                "column".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#column,
                )
                .await,
            );
            map.insert(
                "description".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#description,
                )
                .await,
            );
            map.insert(
                "dimension".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#dimension,
                )
                .await,
            );
            map.insert(
                "ignore_null".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#ignore_null,
                )
                .await,
            );
            map.insert(
                "name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#name,
                )
                .await,
            );
            map.insert(
                "non_null_expectation".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#non_null_expectation,
                )
                .await,
            );
            map.insert(
                "range_expectation".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#range_expectation,
                )
                .await,
            );
            map.insert(
                "regex_expectation".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#regex_expectation,
                )
                .await,
            );
            map.insert(
                "row_condition_expectation".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#row_condition_expectation,
                )
                .await,
            );
            map.insert(
                "set_expectation".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#set_expectation,
                )
                .await,
            );
            map.insert(
                "sql_assertion".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#sql_assertion,
                )
                .await,
            );
            map.insert(
                "statistic_range_expectation".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#statistic_range_expectation,
                )
                .await,
            );
            map.insert(
                "table_condition_expectation".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#table_condition_expectation,
                )
                .await,
            );
            map.insert(
                "threshold".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#threshold,
                )
                .await,
            );
            map.insert(
                "uniqueness_expectation".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#uniqueness_expectation,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for DatascanDataQualitySpecRule {
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
                    r#column: {
                        let field_value = match fields_map.get("column") {
                            Some(value) => value,
                            None => bail!("Missing field 'column' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#description: {
                        let field_value = match fields_map.get("description") {
                            Some(value) => value,
                            None => bail!("Missing field 'description' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#dimension: {
                        let field_value = match fields_map.get("dimension") {
                            Some(value) => value,
                            None => bail!("Missing field 'dimension' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ignore_null: {
                        let field_value = match fields_map.get("ignore_null") {
                            Some(value) => value,
                            None => bail!("Missing field 'ignore_null' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#name: {
                        let field_value = match fields_map.get("name") {
                            Some(value) => value,
                            None => bail!("Missing field 'name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#non_null_expectation: {
                        let field_value = match fields_map.get("non_null_expectation") {
                            Some(value) => value,
                            None => bail!("Missing field 'non_null_expectation' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#range_expectation: {
                        let field_value = match fields_map.get("range_expectation") {
                            Some(value) => value,
                            None => bail!("Missing field 'range_expectation' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#regex_expectation: {
                        let field_value = match fields_map.get("regex_expectation") {
                            Some(value) => value,
                            None => bail!("Missing field 'regex_expectation' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#row_condition_expectation: {
                        let field_value = match fields_map.get("row_condition_expectation") {
                            Some(value) => value,
                            None => bail!("Missing field 'row_condition_expectation' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#set_expectation: {
                        let field_value = match fields_map.get("set_expectation") {
                            Some(value) => value,
                            None => bail!("Missing field 'set_expectation' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#sql_assertion: {
                        let field_value = match fields_map.get("sql_assertion") {
                            Some(value) => value,
                            None => bail!("Missing field 'sql_assertion' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#statistic_range_expectation: {
                        let field_value = match fields_map.get("statistic_range_expectation") {
                            Some(value) => value,
                            None => bail!("Missing field 'statistic_range_expectation' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#table_condition_expectation: {
                        let field_value = match fields_map.get("table_condition_expectation") {
                            Some(value) => value,
                            None => bail!("Missing field 'table_condition_expectation' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#threshold: {
                        let field_value = match fields_map.get("threshold") {
                            Some(value) => value,
                            None => bail!("Missing field 'threshold' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#uniqueness_expectation: {
                        let field_value = match fields_map.get("uniqueness_expectation") {
                            Some(value) => value,
                            None => bail!("Missing field 'uniqueness_expectation' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
