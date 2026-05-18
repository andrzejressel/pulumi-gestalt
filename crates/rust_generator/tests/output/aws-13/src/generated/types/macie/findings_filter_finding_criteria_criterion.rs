#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FindingsFilterFindingCriteriaCriterion {
    /// The value for the property exclusively matches (equals an exact match for) all the specified values. If you specify multiple values, Amazon Macie uses AND logic to join the values.
    #[builder(into)]
    #[serde(rename = "eqExactMatches")]
    pub r#eq_exact_matches: Option<Vec<String>>,
    /// The value for the property matches (equals) the specified value. If you specify multiple values, Amazon Macie uses OR logic to join the values.
    #[builder(into)]
    #[serde(rename = "eqs")]
    pub r#eqs: Option<Vec<String>>,
    /// The name of the field to be evaluated.
    #[builder(into)]
    #[serde(rename = "field")]
    pub r#field: String,
    /// The value for the property is greater than the specified value.
    #[builder(into)]
    #[serde(rename = "gt")]
    pub r#gt: Option<String>,
    /// The value for the property is greater than or equal to the specified value.
    #[builder(into)]
    #[serde(rename = "gte")]
    pub r#gte: Option<String>,
    /// The value for the property is less than the specified value.
    #[builder(into)]
    #[serde(rename = "lt")]
    pub r#lt: Option<String>,
    /// The value for the property is less than or equal to the specified value.
    #[builder(into)]
    #[serde(rename = "lte")]
    pub r#lte: Option<String>,
    /// The value for the property doesn't match (doesn't equal) the specified value. If you specify multiple values, Amazon Macie uses OR logic to join the values.
    #[builder(into)]
    #[serde(rename = "neqs")]
    pub r#neqs: Option<Vec<String>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for FindingsFilterFindingCriteriaCriterion {
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
                    "eq_exact_matches",
                    &self.r#eq_exact_matches,
                ),
                to_pulumi_object_field(
                    "eqs",
                    &self.r#eqs,
                ),
                to_pulumi_object_field(
                    "field",
                    &self.r#field,
                ),
                to_pulumi_object_field(
                    "gt",
                    &self.r#gt,
                ),
                to_pulumi_object_field(
                    "gte",
                    &self.r#gte,
                ),
                to_pulumi_object_field(
                    "lt",
                    &self.r#lt,
                ),
                to_pulumi_object_field(
                    "lte",
                    &self.r#lte,
                ),
                to_pulumi_object_field(
                    "neqs",
                    &self.r#neqs,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for FindingsFilterFindingCriteriaCriterion {
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
                    r#eq_exact_matches: {
                        let field_value = match fields_map.get("eq_exact_matches") {
                            Some(value) => value,
                            None => bail!("Missing field 'eq_exact_matches' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#eqs: {
                        let field_value = match fields_map.get("eqs") {
                            Some(value) => value,
                            None => bail!("Missing field 'eqs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#gt: {
                        let field_value = match fields_map.get("gt") {
                            Some(value) => value,
                            None => bail!("Missing field 'gt' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#gte: {
                        let field_value = match fields_map.get("gte") {
                            Some(value) => value,
                            None => bail!("Missing field 'gte' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#lt: {
                        let field_value = match fields_map.get("lt") {
                            Some(value) => value,
                            None => bail!("Missing field 'lt' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#lte: {
                        let field_value = match fields_map.get("lte") {
                            Some(value) => value,
                            None => bail!("Missing field 'lte' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#neqs: {
                        let field_value = match fields_map.get("neqs") {
                            Some(value) => value,
                            None => bail!("Missing field 'neqs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
