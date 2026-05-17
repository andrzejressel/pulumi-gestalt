#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AlertPolicyConditionConditionPrometheusQueryLanguage {
    /// The alerting rule name of this alert in the corresponding Prometheus
    /// configuration file.
    /// Some external tools may require this field to be populated correctly
    /// in order to refer to the original Prometheus configuration file.
    /// The rule group name and the alert name are necessary to update the
    /// relevant AlertPolicies in case the definition of the rule group changes
    /// in the future.
    /// This field is optional. If this field is not empty, then it must be a
    /// valid Prometheus label name.
    #[builder(into)]
    #[serde(rename = "alertRule")]
    pub r#alert_rule: Option<String>,
    #[builder(into)]
    #[serde(rename = "disableMetricValidation")]
    pub r#disable_metric_validation: Option<bool>,
    /// Alerts are considered firing once their PromQL expression evaluated
    /// to be "true" for this long. Alerts whose PromQL expression was not
    /// evaluated to be "true" for long enough are considered pending. The
    /// default value is zero. Must be zero or positive.
    #[builder(into)]
    #[serde(rename = "duration")]
    pub r#duration: Option<String>,
    /// How often this rule should be evaluated. Must be a positive multiple
    /// of 30 seconds or missing. The default value is 30 seconds. If this
    /// PrometheusQueryLanguageCondition was generated from a Prometheus
    /// alerting rule, then this value should be taken from the enclosing
    /// rule group.
    #[builder(into)]
    #[serde(rename = "evaluationInterval")]
    pub r#evaluation_interval: Option<String>,
    /// Labels to add to or overwrite in the PromQL query result. Label names
    /// must be valid.
    /// Label values can be templatized by using variables. The only available
    /// variable names are the names of the labels in the PromQL result, including
    /// "__name__" and "value". "labels" may be empty. This field is intended to be
    /// used for organizing and identifying the AlertPolicy
    #[builder(into)]
    #[serde(rename = "labels")]
    pub r#labels: Option<std::collections::HashMap<String, String>>,
    /// The PromQL expression to evaluate. Every evaluation cycle this
    /// expression is evaluated at the current time, and all resultant time
    /// series become pending/firing alerts. This field must not be empty.
    #[builder(into)]
    #[serde(rename = "query")]
    pub r#query: String,
    /// The rule group name of this alert in the corresponding Prometheus
    /// configuration file.
    /// Some external tools may require this field to be populated correctly
    /// in order to refer to the original Prometheus configuration file.
    /// The rule group name and the alert name are necessary to update the
    /// relevant AlertPolicies in case the definition of the rule group changes
    /// in the future. This field is optional.
    #[builder(into)]
    #[serde(rename = "ruleGroup")]
    pub r#rule_group: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for AlertPolicyConditionConditionPrometheusQueryLanguage {
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
                    "alert_rule",
                    &self.r#alert_rule,
                ),
                to_pulumi_object_field(
                    "disable_metric_validation",
                    &self.r#disable_metric_validation,
                ),
                to_pulumi_object_field(
                    "duration",
                    &self.r#duration,
                ),
                to_pulumi_object_field(
                    "evaluation_interval",
                    &self.r#evaluation_interval,
                ),
                to_pulumi_object_field(
                    "labels",
                    &self.r#labels,
                ),
                to_pulumi_object_field(
                    "query",
                    &self.r#query,
                ),
                to_pulumi_object_field(
                    "rule_group",
                    &self.r#rule_group,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for AlertPolicyConditionConditionPrometheusQueryLanguage {
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
                    r#alert_rule: {
                        let field_value = match fields_map.get("alert_rule") {
                            Some(value) => value,
                            None => bail!("Missing field 'alert_rule' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#disable_metric_validation: {
                        let field_value = match fields_map.get("disable_metric_validation") {
                            Some(value) => value,
                            None => bail!("Missing field 'disable_metric_validation' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#duration: {
                        let field_value = match fields_map.get("duration") {
                            Some(value) => value,
                            None => bail!("Missing field 'duration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#evaluation_interval: {
                        let field_value = match fields_map.get("evaluation_interval") {
                            Some(value) => value,
                            None => bail!("Missing field 'evaluation_interval' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#labels: {
                        let field_value = match fields_map.get("labels") {
                            Some(value) => value,
                            None => bail!("Missing field 'labels' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#query: {
                        let field_value = match fields_map.get("query") {
                            Some(value) => value,
                            None => bail!("Missing field 'query' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#rule_group: {
                        let field_value = match fields_map.get("rule_group") {
                            Some(value) => value,
                            None => bail!("Missing field 'rule_group' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
