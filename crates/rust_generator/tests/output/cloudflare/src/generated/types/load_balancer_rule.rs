#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct LoadBalancerRule {
    /// The statement to evaluate to determine if this rule's effects should be applied. An empty condition is always true. See [load balancing rules](https://developers.cloudflare.com/load-balancing/understand-basics/load-balancing-rules).
    #[builder(into)]
    pub r#condition: Option<String>,
    /// A disabled rule will not be executed.
    #[builder(into)]
    pub r#disabled: Option<bool>,
    /// Settings for a HTTP response to return directly to the eyeball if the condition is true. Note: `overrides` or `fixed_response` must be set.
    #[builder(into)]
    pub r#fixed_response: Option<Box<super::types::LoadBalancerRuleFixedResponse>>,
    /// Human readable name for this rule.
    #[builder(into)]
    pub r#name: String,
    /// The load balancer settings to alter if this rule's `condition` is true. Note: `overrides` or `fixed_response` must be set.
    #[builder(into)]
    pub r#overrides: Option<Vec<super::types::LoadBalancerRuleOverride>>,
    /// Priority used when determining the order of rule execution. Lower values are executed first. If not provided, the list order will be used.
    #[builder(into)]
    pub r#priority: Option<i32>,
    /// Terminates indicates that if this rule is true no further rules should be executed. Note: setting a `fixed_response` forces this field to `true`.
    #[builder(into)]
    pub r#terminates: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for LoadBalancerRule {
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
                    "condition",
                    &self.r#condition,
                ),
                to_pulumi_object_field(
                    "disabled",
                    &self.r#disabled,
                ),
                to_pulumi_object_field(
                    "fixedResponse",
                    &self.r#fixed_response,
                ),
                to_pulumi_object_field(
                    "name",
                    &self.r#name,
                ),
                to_pulumi_object_field(
                    "overrides",
                    &self.r#overrides,
                ),
                to_pulumi_object_field(
                    "priority",
                    &self.r#priority,
                ),
                to_pulumi_object_field(
                    "terminates",
                    &self.r#terminates,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for LoadBalancerRule {
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
                    r#condition: {
                        let field_value = match fields_map.get("condition") {
                            Some(value) => value,
                            None => bail!("Missing field 'condition' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#disabled: {
                        let field_value = match fields_map.get("disabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'disabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#fixed_response: {
                        let field_value = match fields_map.get("fixedResponse") {
                            Some(value) => value,
                            None => bail!("Missing field 'fixedResponse' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#overrides: {
                        let field_value = match fields_map.get("overrides") {
                            Some(value) => value,
                            None => bail!("Missing field 'overrides' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#priority: {
                        let field_value = match fields_map.get("priority") {
                            Some(value) => value,
                            None => bail!("Missing field 'priority' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#terminates: {
                        let field_value = match fields_map.get("terminates") {
                            Some(value) => value,
                            None => bail!("Missing field 'terminates' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
