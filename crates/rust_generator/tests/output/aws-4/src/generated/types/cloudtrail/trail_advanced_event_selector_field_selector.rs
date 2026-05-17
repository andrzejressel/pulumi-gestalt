#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct TrailAdvancedEventSelectorFieldSelector {
    /// A list of values that includes events that match the last few characters of the event record field specified as the value of `field`.
    #[builder(into)]
    #[serde(rename = "endsWiths")]
    pub r#ends_withs: Option<Vec<String>>,
    /// A list of values that includes events that match the exact value of the event record field specified as the value of `field`. This is the only valid operator that you can use with the `readOnly`, `eventCategory`, and `resources.type` fields.
    #[builder(into)]
    #[serde(rename = "equals")]
    pub r#equals: Option<Vec<String>>,
    /// Field in an event record on which to filter events to be logged. You can specify only the following values: `readOnly`, `eventSource`, `eventName`, `eventCategory`, `resources.type`, `resources.ARN`.
    #[builder(into)]
    #[serde(rename = "field")]
    pub r#field: String,
    /// A list of values that excludes events that match the last few characters of the event record field specified as the value of `field`.
    #[builder(into)]
    #[serde(rename = "notEndsWiths")]
    pub r#not_ends_withs: Option<Vec<String>>,
    /// A list of values that excludes events that match the exact value of the event record field specified as the value of `field`.
    #[builder(into)]
    #[serde(rename = "notEquals")]
    pub r#not_equals: Option<Vec<String>>,
    /// A list of values that excludes events that match the first few characters of the event record field specified as the value of `field`.
    #[builder(into)]
    #[serde(rename = "notStartsWiths")]
    pub r#not_starts_withs: Option<Vec<String>>,
    /// A list of values that includes events that match the first few characters of the event record field specified as the value of `field`.
    #[builder(into)]
    #[serde(rename = "startsWiths")]
    pub r#starts_withs: Option<Vec<String>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for TrailAdvancedEventSelectorFieldSelector {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "ends_withs",
                    &self.r#ends_withs,
                ),
                to_pulumi_object_field(
                    "equals",
                    &self.r#equals,
                ),
                to_pulumi_object_field(
                    "field",
                    &self.r#field,
                ),
                to_pulumi_object_field(
                    "not_ends_withs",
                    &self.r#not_ends_withs,
                ),
                to_pulumi_object_field(
                    "not_equals",
                    &self.r#not_equals,
                ),
                to_pulumi_object_field(
                    "not_starts_withs",
                    &self.r#not_starts_withs,
                ),
                to_pulumi_object_field(
                    "starts_withs",
                    &self.r#starts_withs,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for TrailAdvancedEventSelectorFieldSelector {
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
                    r#ends_withs: {
                        let field_value = match fields_map.get("ends_withs") {
                            Some(value) => value,
                            None => bail!("Missing field 'ends_withs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
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
                    r#not_ends_withs: {
                        let field_value = match fields_map.get("not_ends_withs") {
                            Some(value) => value,
                            None => bail!("Missing field 'not_ends_withs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#not_starts_withs: {
                        let field_value = match fields_map.get("not_starts_withs") {
                            Some(value) => value,
                            None => bail!("Missing field 'not_starts_withs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#starts_withs: {
                        let field_value = match fields_map.get("starts_withs") {
                            Some(value) => value,
                            None => bail!("Missing field 'starts_withs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
