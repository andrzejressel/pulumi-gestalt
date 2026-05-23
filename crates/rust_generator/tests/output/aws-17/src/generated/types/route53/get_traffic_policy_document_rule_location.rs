#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetTrafficPolicyDocumentRuleLocation {
    /// Value of a continent.
    #[builder(into)]
    pub r#continent: Option<String>,
    /// Value of a country.
    #[builder(into)]
    pub r#country: Option<String>,
    /// References to an endpoint.
    #[builder(into)]
    pub r#endpoint_reference: Option<String>,
    /// Indicates whether you want Amazon Route 53 to evaluate the health of the endpoint and route traffic only to healthy endpoints.
    #[builder(into)]
    pub r#evaluate_target_health: Option<bool>,
    /// If you want to associate a health check with the endpoint or rule.
    #[builder(into)]
    pub r#health_check: Option<String>,
    /// Indicates whether this set of values represents the default location.
    #[builder(into)]
    pub r#is_default: Option<bool>,
    /// References to a rule.
    #[builder(into)]
    pub r#rule_reference: Option<String>,
    /// Value of a subdivision.
    #[builder(into)]
    pub r#subdivision: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetTrafficPolicyDocumentRuleLocation {
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
                    "continent",
                    &self.r#continent,
                ),
                to_pulumi_object_field(
                    "country",
                    &self.r#country,
                ),
                to_pulumi_object_field(
                    "endpointReference",
                    &self.r#endpoint_reference,
                ),
                to_pulumi_object_field(
                    "evaluateTargetHealth",
                    &self.r#evaluate_target_health,
                ),
                to_pulumi_object_field(
                    "healthCheck",
                    &self.r#health_check,
                ),
                to_pulumi_object_field(
                    "isDefault",
                    &self.r#is_default,
                ),
                to_pulumi_object_field(
                    "ruleReference",
                    &self.r#rule_reference,
                ),
                to_pulumi_object_field(
                    "subdivision",
                    &self.r#subdivision,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetTrafficPolicyDocumentRuleLocation {
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
                    r#continent: {
                        let field_value = match fields_map.get("continent") {
                            Some(value) => value,
                            None => bail!("Missing field 'continent' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#country: {
                        let field_value = match fields_map.get("country") {
                            Some(value) => value,
                            None => bail!("Missing field 'country' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#endpoint_reference: {
                        let field_value = match fields_map.get("endpointReference") {
                            Some(value) => value,
                            None => bail!("Missing field 'endpointReference' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#evaluate_target_health: {
                        let field_value = match fields_map.get("evaluateTargetHealth") {
                            Some(value) => value,
                            None => bail!("Missing field 'evaluateTargetHealth' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#health_check: {
                        let field_value = match fields_map.get("healthCheck") {
                            Some(value) => value,
                            None => bail!("Missing field 'healthCheck' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#is_default: {
                        let field_value = match fields_map.get("isDefault") {
                            Some(value) => value,
                            None => bail!("Missing field 'isDefault' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#rule_reference: {
                        let field_value = match fields_map.get("ruleReference") {
                            Some(value) => value,
                            None => bail!("Missing field 'ruleReference' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#subdivision: {
                        let field_value = match fields_map.get("subdivision") {
                            Some(value) => value,
                            None => bail!("Missing field 'subdivision' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
