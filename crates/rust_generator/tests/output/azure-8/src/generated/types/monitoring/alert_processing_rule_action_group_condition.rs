#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AlertProcessingRuleActionGroupCondition {
    /// A `alert_context` block as defined above.
    #[builder(into)]
    #[serde(rename = "alertContext")]
    pub r#alert_context: Option<Box<super::super::types::monitoring::AlertProcessingRuleActionGroupConditionAlertContext>>,
    /// A `alert_rule_id` block as defined above.
    #[builder(into)]
    #[serde(rename = "alertRuleId")]
    pub r#alert_rule_id: Option<Box<super::super::types::monitoring::AlertProcessingRuleActionGroupConditionAlertRuleId>>,
    /// A `alert_rule_name` block as defined above.
    #[builder(into)]
    #[serde(rename = "alertRuleName")]
    pub r#alert_rule_name: Option<Box<super::super::types::monitoring::AlertProcessingRuleActionGroupConditionAlertRuleName>>,
    /// A `description` block as defined below.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Option<Box<super::super::types::monitoring::AlertProcessingRuleActionGroupConditionDescription>>,
    /// A `monitor_condition` block as defined below.
    #[builder(into)]
    #[serde(rename = "monitorCondition")]
    pub r#monitor_condition: Option<Box<super::super::types::monitoring::AlertProcessingRuleActionGroupConditionMonitorCondition>>,
    /// A `monitor_service` block as defined below.
    #[builder(into)]
    #[serde(rename = "monitorService")]
    pub r#monitor_service: Option<Box<super::super::types::monitoring::AlertProcessingRuleActionGroupConditionMonitorService>>,
    /// A `severity` block as defined below.
    #[builder(into)]
    #[serde(rename = "severity")]
    pub r#severity: Option<Box<super::super::types::monitoring::AlertProcessingRuleActionGroupConditionSeverity>>,
    /// A `signal_type` block as defined below.
    #[builder(into)]
    #[serde(rename = "signalType")]
    pub r#signal_type: Option<Box<super::super::types::monitoring::AlertProcessingRuleActionGroupConditionSignalType>>,
    /// A `target_resource` block as defined below.
    #[builder(into)]
    #[serde(rename = "targetResource")]
    pub r#target_resource: Option<Box<super::super::types::monitoring::AlertProcessingRuleActionGroupConditionTargetResource>>,
    /// A `target_resource_group` block as defined below.
    #[builder(into)]
    #[serde(rename = "targetResourceGroup")]
    pub r#target_resource_group: Option<Box<super::super::types::monitoring::AlertProcessingRuleActionGroupConditionTargetResourceGroup>>,
    /// A `target_resource_type` block as defined below.
    /// 
    /// > **Note:** At least one of the `alert_context`, `alert_rule_id`, `alert_rule_name`, `description`, `monitor_condition`, `monitor_service`, `severity`, `signal_type`, `target_resource`, `target_resource_group`, `target_resource_type` must be specified.
    #[builder(into)]
    #[serde(rename = "targetResourceType")]
    pub r#target_resource_type: Option<Box<super::super::types::monitoring::AlertProcessingRuleActionGroupConditionTargetResourceType>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for AlertProcessingRuleActionGroupCondition {
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
                    "alert_context",
                    &self.r#alert_context,
                ),
                to_pulumi_object_field(
                    "alert_rule_id",
                    &self.r#alert_rule_id,
                ),
                to_pulumi_object_field(
                    "alert_rule_name",
                    &self.r#alert_rule_name,
                ),
                to_pulumi_object_field(
                    "description",
                    &self.r#description,
                ),
                to_pulumi_object_field(
                    "monitor_condition",
                    &self.r#monitor_condition,
                ),
                to_pulumi_object_field(
                    "monitor_service",
                    &self.r#monitor_service,
                ),
                to_pulumi_object_field(
                    "severity",
                    &self.r#severity,
                ),
                to_pulumi_object_field(
                    "signal_type",
                    &self.r#signal_type,
                ),
                to_pulumi_object_field(
                    "target_resource",
                    &self.r#target_resource,
                ),
                to_pulumi_object_field(
                    "target_resource_group",
                    &self.r#target_resource_group,
                ),
                to_pulumi_object_field(
                    "target_resource_type",
                    &self.r#target_resource_type,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for AlertProcessingRuleActionGroupCondition {
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
                    r#alert_context: {
                        let field_value = match fields_map.get("alert_context") {
                            Some(value) => value,
                            None => bail!("Missing field 'alert_context' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#alert_rule_id: {
                        let field_value = match fields_map.get("alert_rule_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'alert_rule_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#alert_rule_name: {
                        let field_value = match fields_map.get("alert_rule_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'alert_rule_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#monitor_condition: {
                        let field_value = match fields_map.get("monitor_condition") {
                            Some(value) => value,
                            None => bail!("Missing field 'monitor_condition' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#monitor_service: {
                        let field_value = match fields_map.get("monitor_service") {
                            Some(value) => value,
                            None => bail!("Missing field 'monitor_service' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#severity: {
                        let field_value = match fields_map.get("severity") {
                            Some(value) => value,
                            None => bail!("Missing field 'severity' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#signal_type: {
                        let field_value = match fields_map.get("signal_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'signal_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#target_resource: {
                        let field_value = match fields_map.get("target_resource") {
                            Some(value) => value,
                            None => bail!("Missing field 'target_resource' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#target_resource_group: {
                        let field_value = match fields_map.get("target_resource_group") {
                            Some(value) => value,
                            None => bail!("Missing field 'target_resource_group' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#target_resource_type: {
                        let field_value = match fields_map.get("target_resource_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'target_resource_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
