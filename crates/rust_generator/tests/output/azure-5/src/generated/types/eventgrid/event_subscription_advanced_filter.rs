#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct EventSubscriptionAdvancedFilter {
    /// Compares a value of an event using a single boolean value.
    #[builder(into)]
    pub r#bool_equals: Option<Vec<super::super::types::eventgrid::EventSubscriptionAdvancedFilterBoolEqual>>,
    /// Evaluates if a value of an event isn't NULL or undefined.
    #[builder(into)]
    pub r#is_not_nulls: Option<Vec<super::super::types::eventgrid::EventSubscriptionAdvancedFilterIsNotNull>>,
    /// Evaluates if a value of an event is NULL or undefined.
    /// 
    /// Each nested block consists of a key and a value(s) element.
    #[builder(into)]
    pub r#is_null_or_undefineds: Option<Vec<super::super::types::eventgrid::EventSubscriptionAdvancedFilterIsNullOrUndefined>>,
    /// Compares a value of an event using a single floating point number.
    #[builder(into)]
    pub r#number_greater_than_or_equals: Option<Vec<super::super::types::eventgrid::EventSubscriptionAdvancedFilterNumberGreaterThanOrEqual>>,
    /// Compares a value of an event using a single floating point number.
    #[builder(into)]
    pub r#number_greater_thans: Option<Vec<super::super::types::eventgrid::EventSubscriptionAdvancedFilterNumberGreaterThan>>,
    /// Compares a value of an event using multiple floating point number ranges.
    #[builder(into)]
    pub r#number_in_ranges: Option<Vec<super::super::types::eventgrid::EventSubscriptionAdvancedFilterNumberInRange>>,
    /// Compares a value of an event using multiple floating point numbers.
    #[builder(into)]
    pub r#number_ins: Option<Vec<super::super::types::eventgrid::EventSubscriptionAdvancedFilterNumberIn>>,
    /// Compares a value of an event using a single floating point number.
    #[builder(into)]
    pub r#number_less_than_or_equals: Option<Vec<super::super::types::eventgrid::EventSubscriptionAdvancedFilterNumberLessThanOrEqual>>,
    /// Compares a value of an event using a single floating point number.
    #[builder(into)]
    pub r#number_less_thans: Option<Vec<super::super::types::eventgrid::EventSubscriptionAdvancedFilterNumberLessThan>>,
    /// Compares a value of an event using multiple floating point number ranges.
    #[builder(into)]
    pub r#number_not_in_ranges: Option<Vec<super::super::types::eventgrid::EventSubscriptionAdvancedFilterNumberNotInRange>>,
    /// Compares a value of an event using multiple floating point numbers.
    #[builder(into)]
    pub r#number_not_ins: Option<Vec<super::super::types::eventgrid::EventSubscriptionAdvancedFilterNumberNotIn>>,
    /// Compares a value of an event using multiple string values.
    #[builder(into)]
    pub r#string_begins_withs: Option<Vec<super::super::types::eventgrid::EventSubscriptionAdvancedFilterStringBeginsWith>>,
    /// Compares a value of an event using multiple string values.
    #[builder(into)]
    pub r#string_contains: Option<Vec<super::super::types::eventgrid::EventSubscriptionAdvancedFilterStringContain>>,
    /// Compares a value of an event using multiple string values.
    #[builder(into)]
    pub r#string_ends_withs: Option<Vec<super::super::types::eventgrid::EventSubscriptionAdvancedFilterStringEndsWith>>,
    /// Compares a value of an event using multiple string values.
    #[builder(into)]
    pub r#string_ins: Option<Vec<super::super::types::eventgrid::EventSubscriptionAdvancedFilterStringIn>>,
    /// Compares a value of an event using multiple string values.
    #[builder(into)]
    pub r#string_not_begins_withs: Option<Vec<super::super::types::eventgrid::EventSubscriptionAdvancedFilterStringNotBeginsWith>>,
    /// Compares a value of an event using multiple string values.
    #[builder(into)]
    pub r#string_not_contains: Option<Vec<super::super::types::eventgrid::EventSubscriptionAdvancedFilterStringNotContain>>,
    /// Compares a value of an event using multiple string values.
    #[builder(into)]
    pub r#string_not_ends_withs: Option<Vec<super::super::types::eventgrid::EventSubscriptionAdvancedFilterStringNotEndsWith>>,
    /// Compares a value of an event using multiple string values.
    #[builder(into)]
    pub r#string_not_ins: Option<Vec<super::super::types::eventgrid::EventSubscriptionAdvancedFilterStringNotIn>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for EventSubscriptionAdvancedFilter {
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
                    "boolEquals",
                    &self.r#bool_equals,
                ),
                to_pulumi_object_field(
                    "isNotNulls",
                    &self.r#is_not_nulls,
                ),
                to_pulumi_object_field(
                    "isNullOrUndefineds",
                    &self.r#is_null_or_undefineds,
                ),
                to_pulumi_object_field(
                    "numberGreaterThanOrEquals",
                    &self.r#number_greater_than_or_equals,
                ),
                to_pulumi_object_field(
                    "numberGreaterThans",
                    &self.r#number_greater_thans,
                ),
                to_pulumi_object_field(
                    "numberInRanges",
                    &self.r#number_in_ranges,
                ),
                to_pulumi_object_field(
                    "numberIns",
                    &self.r#number_ins,
                ),
                to_pulumi_object_field(
                    "numberLessThanOrEquals",
                    &self.r#number_less_than_or_equals,
                ),
                to_pulumi_object_field(
                    "numberLessThans",
                    &self.r#number_less_thans,
                ),
                to_pulumi_object_field(
                    "numberNotInRanges",
                    &self.r#number_not_in_ranges,
                ),
                to_pulumi_object_field(
                    "numberNotIns",
                    &self.r#number_not_ins,
                ),
                to_pulumi_object_field(
                    "stringBeginsWiths",
                    &self.r#string_begins_withs,
                ),
                to_pulumi_object_field(
                    "stringContains",
                    &self.r#string_contains,
                ),
                to_pulumi_object_field(
                    "stringEndsWiths",
                    &self.r#string_ends_withs,
                ),
                to_pulumi_object_field(
                    "stringIns",
                    &self.r#string_ins,
                ),
                to_pulumi_object_field(
                    "stringNotBeginsWiths",
                    &self.r#string_not_begins_withs,
                ),
                to_pulumi_object_field(
                    "stringNotContains",
                    &self.r#string_not_contains,
                ),
                to_pulumi_object_field(
                    "stringNotEndsWiths",
                    &self.r#string_not_ends_withs,
                ),
                to_pulumi_object_field(
                    "stringNotIns",
                    &self.r#string_not_ins,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for EventSubscriptionAdvancedFilter {
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
                        let field_value = match fields_map.get("boolEquals") {
                            Some(value) => value,
                            None => bail!("Missing field 'boolEquals' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#is_not_nulls: {
                        let field_value = match fields_map.get("isNotNulls") {
                            Some(value) => value,
                            None => bail!("Missing field 'isNotNulls' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#is_null_or_undefineds: {
                        let field_value = match fields_map.get("isNullOrUndefineds") {
                            Some(value) => value,
                            None => bail!("Missing field 'isNullOrUndefineds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#number_greater_than_or_equals: {
                        let field_value = match fields_map.get("numberGreaterThanOrEquals") {
                            Some(value) => value,
                            None => bail!("Missing field 'numberGreaterThanOrEquals' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#number_greater_thans: {
                        let field_value = match fields_map.get("numberGreaterThans") {
                            Some(value) => value,
                            None => bail!("Missing field 'numberGreaterThans' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#number_in_ranges: {
                        let field_value = match fields_map.get("numberInRanges") {
                            Some(value) => value,
                            None => bail!("Missing field 'numberInRanges' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#number_ins: {
                        let field_value = match fields_map.get("numberIns") {
                            Some(value) => value,
                            None => bail!("Missing field 'numberIns' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#number_less_than_or_equals: {
                        let field_value = match fields_map.get("numberLessThanOrEquals") {
                            Some(value) => value,
                            None => bail!("Missing field 'numberLessThanOrEquals' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#number_less_thans: {
                        let field_value = match fields_map.get("numberLessThans") {
                            Some(value) => value,
                            None => bail!("Missing field 'numberLessThans' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#number_not_in_ranges: {
                        let field_value = match fields_map.get("numberNotInRanges") {
                            Some(value) => value,
                            None => bail!("Missing field 'numberNotInRanges' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#number_not_ins: {
                        let field_value = match fields_map.get("numberNotIns") {
                            Some(value) => value,
                            None => bail!("Missing field 'numberNotIns' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#string_begins_withs: {
                        let field_value = match fields_map.get("stringBeginsWiths") {
                            Some(value) => value,
                            None => bail!("Missing field 'stringBeginsWiths' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#string_contains: {
                        let field_value = match fields_map.get("stringContains") {
                            Some(value) => value,
                            None => bail!("Missing field 'stringContains' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#string_ends_withs: {
                        let field_value = match fields_map.get("stringEndsWiths") {
                            Some(value) => value,
                            None => bail!("Missing field 'stringEndsWiths' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#string_ins: {
                        let field_value = match fields_map.get("stringIns") {
                            Some(value) => value,
                            None => bail!("Missing field 'stringIns' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#string_not_begins_withs: {
                        let field_value = match fields_map.get("stringNotBeginsWiths") {
                            Some(value) => value,
                            None => bail!("Missing field 'stringNotBeginsWiths' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#string_not_contains: {
                        let field_value = match fields_map.get("stringNotContains") {
                            Some(value) => value,
                            None => bail!("Missing field 'stringNotContains' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#string_not_ends_withs: {
                        let field_value = match fields_map.get("stringNotEndsWiths") {
                            Some(value) => value,
                            None => bail!("Missing field 'stringNotEndsWiths' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#string_not_ins: {
                        let field_value = match fields_map.get("stringNotIns") {
                            Some(value) => value,
                            None => bail!("Missing field 'stringNotIns' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
