#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DatascanDataQualitySpecRule {
    /// The unnested column which this rule is evaluated against.
    #[builder(into)]
    pub r#column: Option<String>,
    /// Description of the rule.
    /// The maximum length is 1,024 characters.
    #[builder(into)]
    pub r#description: Option<String>,
    /// The dimension a rule belongs to. Results are also aggregated at the dimension level. Supported dimensions are ["COMPLETENESS", "ACCURACY", "CONSISTENCY", "VALIDITY", "UNIQUENESS", "INTEGRITY"]
    #[builder(into)]
    pub r#dimension: String,
    /// Rows with null values will automatically fail a rule, unless ignoreNull is true. In that case, such null rows are trivially considered passing. Only applicable to ColumnMap rules.
    #[builder(into)]
    pub r#ignore_null: Option<bool>,
    /// A mutable name for the rule.
    /// The name must contain only letters (a-z, A-Z), numbers (0-9), or hyphens (-).
    /// The maximum length is 63 characters.
    /// Must start with a letter.
    /// Must end with a number or a letter.
    #[builder(into)]
    pub r#name: Option<String>,
    /// ColumnMap rule which evaluates whether each column value is null.
    #[builder(into)]
    pub r#non_null_expectation: Option<Box<super::super::types::dataplex::DatascanDataQualitySpecRuleNonNullExpectation>>,
    /// ColumnMap rule which evaluates whether each column value lies between a specified range.
    /// Structure is documented below.
    #[builder(into)]
    pub r#range_expectation: Option<Box<super::super::types::dataplex::DatascanDataQualitySpecRuleRangeExpectation>>,
    /// ColumnMap rule which evaluates whether each column value matches a specified regex.
    /// Structure is documented below.
    #[builder(into)]
    pub r#regex_expectation: Option<Box<super::super::types::dataplex::DatascanDataQualitySpecRuleRegexExpectation>>,
    /// Table rule which evaluates whether each row passes the specified condition.
    /// Structure is documented below.
    #[builder(into)]
    pub r#row_condition_expectation: Option<Box<super::super::types::dataplex::DatascanDataQualitySpecRuleRowConditionExpectation>>,
    /// ColumnMap rule which evaluates whether each column value is contained by a specified set.
    /// Structure is documented below.
    #[builder(into)]
    pub r#set_expectation: Option<Box<super::super::types::dataplex::DatascanDataQualitySpecRuleSetExpectation>>,
    /// Table rule which evaluates whether any row matches invalid state.
    /// Structure is documented below.
    #[builder(into)]
    pub r#sql_assertion: Option<Box<super::super::types::dataplex::DatascanDataQualitySpecRuleSqlAssertion>>,
    /// ColumnAggregate rule which evaluates whether the column aggregate statistic lies between a specified range.
    /// Structure is documented below.
    #[builder(into)]
    pub r#statistic_range_expectation: Option<Box<super::super::types::dataplex::DatascanDataQualitySpecRuleStatisticRangeExpectation>>,
    /// Table rule which evaluates whether the provided expression is true.
    /// Structure is documented below.
    #[builder(into)]
    pub r#table_condition_expectation: Option<Box<super::super::types::dataplex::DatascanDataQualitySpecRuleTableConditionExpectation>>,
    /// The minimum ratio of passing_rows / total_rows required to pass this rule, with a range of [0.0, 1.0]. 0 indicates default value (i.e. 1.0).
    #[builder(into)]
    pub r#threshold: Option<f64>,
    /// Row-level rule which evaluates whether each column value is unique.
    #[builder(into)]
    pub r#uniqueness_expectation: Option<Box<super::super::types::dataplex::DatascanDataQualitySpecRuleUniquenessExpectation>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for DatascanDataQualitySpecRule {
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
                    "column",
                    &self.r#column,
                ),
                to_pulumi_object_field(
                    "description",
                    &self.r#description,
                ),
                to_pulumi_object_field(
                    "dimension",
                    &self.r#dimension,
                ),
                to_pulumi_object_field(
                    "ignoreNull",
                    &self.r#ignore_null,
                ),
                to_pulumi_object_field(
                    "name",
                    &self.r#name,
                ),
                to_pulumi_object_field(
                    "nonNullExpectation",
                    &self.r#non_null_expectation,
                ),
                to_pulumi_object_field(
                    "rangeExpectation",
                    &self.r#range_expectation,
                ),
                to_pulumi_object_field(
                    "regexExpectation",
                    &self.r#regex_expectation,
                ),
                to_pulumi_object_field(
                    "rowConditionExpectation",
                    &self.r#row_condition_expectation,
                ),
                to_pulumi_object_field(
                    "setExpectation",
                    &self.r#set_expectation,
                ),
                to_pulumi_object_field(
                    "sqlAssertion",
                    &self.r#sql_assertion,
                ),
                to_pulumi_object_field(
                    "statisticRangeExpectation",
                    &self.r#statistic_range_expectation,
                ),
                to_pulumi_object_field(
                    "tableConditionExpectation",
                    &self.r#table_condition_expectation,
                ),
                to_pulumi_object_field(
                    "threshold",
                    &self.r#threshold,
                ),
                to_pulumi_object_field(
                    "uniquenessExpectation",
                    &self.r#uniqueness_expectation,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
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
                        let field_value = match fields_map.get("ignoreNull") {
                            Some(value) => value,
                            None => bail!("Missing field 'ignoreNull' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("nonNullExpectation") {
                            Some(value) => value,
                            None => bail!("Missing field 'nonNullExpectation' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#range_expectation: {
                        let field_value = match fields_map.get("rangeExpectation") {
                            Some(value) => value,
                            None => bail!("Missing field 'rangeExpectation' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#regex_expectation: {
                        let field_value = match fields_map.get("regexExpectation") {
                            Some(value) => value,
                            None => bail!("Missing field 'regexExpectation' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#row_condition_expectation: {
                        let field_value = match fields_map.get("rowConditionExpectation") {
                            Some(value) => value,
                            None => bail!("Missing field 'rowConditionExpectation' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#set_expectation: {
                        let field_value = match fields_map.get("setExpectation") {
                            Some(value) => value,
                            None => bail!("Missing field 'setExpectation' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#sql_assertion: {
                        let field_value = match fields_map.get("sqlAssertion") {
                            Some(value) => value,
                            None => bail!("Missing field 'sqlAssertion' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#statistic_range_expectation: {
                        let field_value = match fields_map.get("statisticRangeExpectation") {
                            Some(value) => value,
                            None => bail!("Missing field 'statisticRangeExpectation' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#table_condition_expectation: {
                        let field_value = match fields_map.get("tableConditionExpectation") {
                            Some(value) => value,
                            None => bail!("Missing field 'tableConditionExpectation' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("uniquenessExpectation") {
                            Some(value) => value,
                            None => bail!("Missing field 'uniquenessExpectation' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
