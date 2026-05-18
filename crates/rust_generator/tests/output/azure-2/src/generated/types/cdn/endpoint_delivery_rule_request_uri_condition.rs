#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct EndpointDeliveryRuleRequestUriCondition {
    /// List of string values. This is required if `operator` is not `Any`.
    #[builder(into)]
    #[serde(rename = "matchValues")]
    pub r#match_values: Option<Vec<String>>,
    /// Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "negateCondition")]
    pub r#negate_condition: Option<bool>,
    /// Valid values are `Any`, `BeginsWith`, `Contains`, `EndsWith`, `Equal`, `GreaterThan`, `GreaterThanOrEqual`, `LessThan` and `LessThanOrEqual`.
    #[builder(into)]
    #[serde(rename = "operator")]
    pub r#operator: String,
    /// A list of transforms. Valid values are `Lowercase` and `Uppercase`.
    #[builder(into)]
    #[serde(rename = "transforms")]
    pub r#transforms: Option<Vec<String>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for EndpointDeliveryRuleRequestUriCondition {
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
                    "match_values",
                    &self.r#match_values,
                ),
                to_pulumi_object_field(
                    "negate_condition",
                    &self.r#negate_condition,
                ),
                to_pulumi_object_field(
                    "operator",
                    &self.r#operator,
                ),
                to_pulumi_object_field(
                    "transforms",
                    &self.r#transforms,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for EndpointDeliveryRuleRequestUriCondition {
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
                    r#match_values: {
                        let field_value = match fields_map.get("match_values") {
                            Some(value) => value,
                            None => bail!("Missing field 'match_values' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#negate_condition: {
                        let field_value = match fields_map.get("negate_condition") {
                            Some(value) => value,
                            None => bail!("Missing field 'negate_condition' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#operator: {
                        let field_value = match fields_map.get("operator") {
                            Some(value) => value,
                            None => bail!("Missing field 'operator' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#transforms: {
                        let field_value = match fields_map.get("transforms") {
                            Some(value) => value,
                            None => bail!("Missing field 'transforms' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
