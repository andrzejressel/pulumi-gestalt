#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct SystemTopicEventSubscriptionAdvancedFilter {
    /// Compares a value of an event using a single boolean value.
    #[builder(into)]
    #[serde(rename = "boolEquals")]
    pub r#bool_equals: Option<Vec<super::super::types::eventgrid::SystemTopicEventSubscriptionAdvancedFilterBoolEqual>>,
    /// Evaluates if a value of an event isn't NULL or undefined.
    #[builder(into)]
    #[serde(rename = "isNotNulls")]
    pub r#is_not_nulls: Option<Vec<super::super::types::eventgrid::SystemTopicEventSubscriptionAdvancedFilterIsNotNull>>,
    /// Evaluates if a value of an event is NULL or undefined.
    /// 
    /// Each nested block consists of a key and a value(s) element.
    #[builder(into)]
    #[serde(rename = "isNullOrUndefineds")]
    pub r#is_null_or_undefineds: Option<Vec<super::super::types::eventgrid::SystemTopicEventSubscriptionAdvancedFilterIsNullOrUndefined>>,
    /// Compares a value of an event using a single floating point number.
    #[builder(into)]
    #[serde(rename = "numberGreaterThanOrEquals")]
    pub r#number_greater_than_or_equals: Option<Vec<super::super::types::eventgrid::SystemTopicEventSubscriptionAdvancedFilterNumberGreaterThanOrEqual>>,
    /// Compares a value of an event using a single floating point number.
    #[builder(into)]
    #[serde(rename = "numberGreaterThans")]
    pub r#number_greater_thans: Option<Vec<super::super::types::eventgrid::SystemTopicEventSubscriptionAdvancedFilterNumberGreaterThan>>,
    /// Compares a value of an event using multiple floating point number ranges.
    #[builder(into)]
    #[serde(rename = "numberInRanges")]
    pub r#number_in_ranges: Option<Vec<super::super::types::eventgrid::SystemTopicEventSubscriptionAdvancedFilterNumberInRange>>,
    /// Compares a value of an event using multiple floating point numbers.
    #[builder(into)]
    #[serde(rename = "numberIns")]
    pub r#number_ins: Option<Vec<super::super::types::eventgrid::SystemTopicEventSubscriptionAdvancedFilterNumberIn>>,
    /// Compares a value of an event using a single floating point number.
    #[builder(into)]
    #[serde(rename = "numberLessThanOrEquals")]
    pub r#number_less_than_or_equals: Option<Vec<super::super::types::eventgrid::SystemTopicEventSubscriptionAdvancedFilterNumberLessThanOrEqual>>,
    /// Compares a value of an event using a single floating point number.
    #[builder(into)]
    #[serde(rename = "numberLessThans")]
    pub r#number_less_thans: Option<Vec<super::super::types::eventgrid::SystemTopicEventSubscriptionAdvancedFilterNumberLessThan>>,
    /// Compares a value of an event using multiple floating point number ranges.
    #[builder(into)]
    #[serde(rename = "numberNotInRanges")]
    pub r#number_not_in_ranges: Option<Vec<super::super::types::eventgrid::SystemTopicEventSubscriptionAdvancedFilterNumberNotInRange>>,
    /// Compares a value of an event using multiple floating point numbers.
    #[builder(into)]
    #[serde(rename = "numberNotIns")]
    pub r#number_not_ins: Option<Vec<super::super::types::eventgrid::SystemTopicEventSubscriptionAdvancedFilterNumberNotIn>>,
    /// Compares a value of an event using multiple string values.
    #[builder(into)]
    #[serde(rename = "stringBeginsWiths")]
    pub r#string_begins_withs: Option<Vec<super::super::types::eventgrid::SystemTopicEventSubscriptionAdvancedFilterStringBeginsWith>>,
    /// Compares a value of an event using multiple string values.
    #[builder(into)]
    #[serde(rename = "stringContains")]
    pub r#string_contains: Option<Vec<super::super::types::eventgrid::SystemTopicEventSubscriptionAdvancedFilterStringContain>>,
    /// Compares a value of an event using multiple string values.
    #[builder(into)]
    #[serde(rename = "stringEndsWiths")]
    pub r#string_ends_withs: Option<Vec<super::super::types::eventgrid::SystemTopicEventSubscriptionAdvancedFilterStringEndsWith>>,
    /// Compares a value of an event using multiple string values.
    #[builder(into)]
    #[serde(rename = "stringIns")]
    pub r#string_ins: Option<Vec<super::super::types::eventgrid::SystemTopicEventSubscriptionAdvancedFilterStringIn>>,
    /// Compares a value of an event using multiple string values.
    #[builder(into)]
    #[serde(rename = "stringNotBeginsWiths")]
    pub r#string_not_begins_withs: Option<Vec<super::super::types::eventgrid::SystemTopicEventSubscriptionAdvancedFilterStringNotBeginsWith>>,
    /// Compares a value of an event using multiple string values.
    #[builder(into)]
    #[serde(rename = "stringNotContains")]
    pub r#string_not_contains: Option<Vec<super::super::types::eventgrid::SystemTopicEventSubscriptionAdvancedFilterStringNotContain>>,
    /// Compares a value of an event using multiple string values.
    #[builder(into)]
    #[serde(rename = "stringNotEndsWiths")]
    pub r#string_not_ends_withs: Option<Vec<super::super::types::eventgrid::SystemTopicEventSubscriptionAdvancedFilterStringNotEndsWith>>,
    /// Compares a value of an event using multiple string values.
    #[builder(into)]
    #[serde(rename = "stringNotIns")]
    pub r#string_not_ins: Option<Vec<super::super::types::eventgrid::SystemTopicEventSubscriptionAdvancedFilterStringNotIn>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for SystemTopicEventSubscriptionAdvancedFilter {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_concurrent, to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "bool_equals",
                    &self.r#bool_equals,
                ),
                to_pulumi_object_field(
                    "is_not_nulls",
                    &self.r#is_not_nulls,
                ),
                to_pulumi_object_field(
                    "is_null_or_undefineds",
                    &self.r#is_null_or_undefineds,
                ),
                to_pulumi_object_field(
                    "number_greater_than_or_equals",
                    &self.r#number_greater_than_or_equals,
                ),
                to_pulumi_object_field(
                    "number_greater_thans",
                    &self.r#number_greater_thans,
                ),
                to_pulumi_object_field(
                    "number_in_ranges",
                    &self.r#number_in_ranges,
                ),
                to_pulumi_object_field(
                    "number_ins",
                    &self.r#number_ins,
                ),
                to_pulumi_object_field(
                    "number_less_than_or_equals",
                    &self.r#number_less_than_or_equals,
                ),
                to_pulumi_object_field(
                    "number_less_thans",
                    &self.r#number_less_thans,
                ),
                to_pulumi_object_field(
                    "number_not_in_ranges",
                    &self.r#number_not_in_ranges,
                ),
                to_pulumi_object_field(
                    "number_not_ins",
                    &self.r#number_not_ins,
                ),
                to_pulumi_object_field(
                    "string_begins_withs",
                    &self.r#string_begins_withs,
                ),
                to_pulumi_object_field(
                    "string_contains",
                    &self.r#string_contains,
                ),
                to_pulumi_object_field(
                    "string_ends_withs",
                    &self.r#string_ends_withs,
                ),
                to_pulumi_object_field(
                    "string_ins",
                    &self.r#string_ins,
                ),
                to_pulumi_object_field(
                    "string_not_begins_withs",
                    &self.r#string_not_begins_withs,
                ),
                to_pulumi_object_field(
                    "string_not_contains",
                    &self.r#string_not_contains,
                ),
                to_pulumi_object_field(
                    "string_not_ends_withs",
                    &self.r#string_not_ends_withs,
                ),
                to_pulumi_object_field(
                    "string_not_ins",
                    &self.r#string_not_ins,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for SystemTopicEventSubscriptionAdvancedFilter {
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
                    r#bool_equals: {
                        let field_value = match fields_map.get("bool_equals") {
                            Some(value) => value,
                            None => bail!("Missing field 'bool_equals' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#is_not_nulls: {
                        let field_value = match fields_map.get("is_not_nulls") {
                            Some(value) => value,
                            None => bail!("Missing field 'is_not_nulls' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#is_null_or_undefineds: {
                        let field_value = match fields_map.get("is_null_or_undefineds") {
                            Some(value) => value,
                            None => bail!("Missing field 'is_null_or_undefineds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#number_greater_than_or_equals: {
                        let field_value = match fields_map.get("number_greater_than_or_equals") {
                            Some(value) => value,
                            None => bail!("Missing field 'number_greater_than_or_equals' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#number_greater_thans: {
                        let field_value = match fields_map.get("number_greater_thans") {
                            Some(value) => value,
                            None => bail!("Missing field 'number_greater_thans' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#number_in_ranges: {
                        let field_value = match fields_map.get("number_in_ranges") {
                            Some(value) => value,
                            None => bail!("Missing field 'number_in_ranges' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#number_ins: {
                        let field_value = match fields_map.get("number_ins") {
                            Some(value) => value,
                            None => bail!("Missing field 'number_ins' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#number_less_than_or_equals: {
                        let field_value = match fields_map.get("number_less_than_or_equals") {
                            Some(value) => value,
                            None => bail!("Missing field 'number_less_than_or_equals' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#number_less_thans: {
                        let field_value = match fields_map.get("number_less_thans") {
                            Some(value) => value,
                            None => bail!("Missing field 'number_less_thans' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#number_not_in_ranges: {
                        let field_value = match fields_map.get("number_not_in_ranges") {
                            Some(value) => value,
                            None => bail!("Missing field 'number_not_in_ranges' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#number_not_ins: {
                        let field_value = match fields_map.get("number_not_ins") {
                            Some(value) => value,
                            None => bail!("Missing field 'number_not_ins' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#string_begins_withs: {
                        let field_value = match fields_map.get("string_begins_withs") {
                            Some(value) => value,
                            None => bail!("Missing field 'string_begins_withs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#string_contains: {
                        let field_value = match fields_map.get("string_contains") {
                            Some(value) => value,
                            None => bail!("Missing field 'string_contains' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#string_ends_withs: {
                        let field_value = match fields_map.get("string_ends_withs") {
                            Some(value) => value,
                            None => bail!("Missing field 'string_ends_withs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#string_ins: {
                        let field_value = match fields_map.get("string_ins") {
                            Some(value) => value,
                            None => bail!("Missing field 'string_ins' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#string_not_begins_withs: {
                        let field_value = match fields_map.get("string_not_begins_withs") {
                            Some(value) => value,
                            None => bail!("Missing field 'string_not_begins_withs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#string_not_contains: {
                        let field_value = match fields_map.get("string_not_contains") {
                            Some(value) => value,
                            None => bail!("Missing field 'string_not_contains' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#string_not_ends_withs: {
                        let field_value = match fields_map.get("string_not_ends_withs") {
                            Some(value) => value,
                            None => bail!("Missing field 'string_not_ends_withs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#string_not_ins: {
                        let field_value = match fields_map.get("string_not_ins") {
                            Some(value) => value,
                            None => bail!("Missing field 'string_not_ins' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
