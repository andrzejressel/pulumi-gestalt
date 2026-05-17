#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetFirewallPolicyFirewallPolicy {
    #[builder(into)]
    #[serde(rename = "statefulDefaultActions")]
    pub r#stateful_default_actions: Vec<String>,
    #[builder(into)]
    #[serde(rename = "statefulEngineOptions")]
    pub r#stateful_engine_options: Vec<super::super::types::networkfirewall::GetFirewallPolicyFirewallPolicyStatefulEngineOption>,
    #[builder(into)]
    #[serde(rename = "statefulRuleGroupReferences")]
    pub r#stateful_rule_group_references: Vec<super::super::types::networkfirewall::GetFirewallPolicyFirewallPolicyStatefulRuleGroupReference>,
    #[builder(into)]
    #[serde(rename = "statelessCustomActions")]
    pub r#stateless_custom_actions: Vec<super::super::types::networkfirewall::GetFirewallPolicyFirewallPolicyStatelessCustomAction>,
    #[builder(into)]
    #[serde(rename = "statelessDefaultActions")]
    pub r#stateless_default_actions: Vec<String>,
    #[builder(into)]
    #[serde(rename = "statelessFragmentDefaultActions")]
    pub r#stateless_fragment_default_actions: Vec<String>,
    #[builder(into)]
    #[serde(rename = "statelessRuleGroupReferences")]
    pub r#stateless_rule_group_references: Vec<super::super::types::networkfirewall::GetFirewallPolicyFirewallPolicyStatelessRuleGroupReference>,
    #[builder(into)]
    #[serde(rename = "tlsInspectionConfigurationArn")]
    pub r#tls_inspection_configuration_arn: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetFirewallPolicyFirewallPolicy {
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
                    "stateful_default_actions",
                    &self.r#stateful_default_actions,
                ),
                to_pulumi_object_field(
                    "stateful_engine_options",
                    &self.r#stateful_engine_options,
                ),
                to_pulumi_object_field(
                    "stateful_rule_group_references",
                    &self.r#stateful_rule_group_references,
                ),
                to_pulumi_object_field(
                    "stateless_custom_actions",
                    &self.r#stateless_custom_actions,
                ),
                to_pulumi_object_field(
                    "stateless_default_actions",
                    &self.r#stateless_default_actions,
                ),
                to_pulumi_object_field(
                    "stateless_fragment_default_actions",
                    &self.r#stateless_fragment_default_actions,
                ),
                to_pulumi_object_field(
                    "stateless_rule_group_references",
                    &self.r#stateless_rule_group_references,
                ),
                to_pulumi_object_field(
                    "tls_inspection_configuration_arn",
                    &self.r#tls_inspection_configuration_arn,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetFirewallPolicyFirewallPolicy {
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
                    r#stateful_default_actions: {
                        let field_value = match fields_map.get("stateful_default_actions") {
                            Some(value) => value,
                            None => bail!("Missing field 'stateful_default_actions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#stateful_engine_options: {
                        let field_value = match fields_map.get("stateful_engine_options") {
                            Some(value) => value,
                            None => bail!("Missing field 'stateful_engine_options' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#stateful_rule_group_references: {
                        let field_value = match fields_map.get("stateful_rule_group_references") {
                            Some(value) => value,
                            None => bail!("Missing field 'stateful_rule_group_references' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#stateless_custom_actions: {
                        let field_value = match fields_map.get("stateless_custom_actions") {
                            Some(value) => value,
                            None => bail!("Missing field 'stateless_custom_actions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#stateless_default_actions: {
                        let field_value = match fields_map.get("stateless_default_actions") {
                            Some(value) => value,
                            None => bail!("Missing field 'stateless_default_actions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#stateless_fragment_default_actions: {
                        let field_value = match fields_map.get("stateless_fragment_default_actions") {
                            Some(value) => value,
                            None => bail!("Missing field 'stateless_fragment_default_actions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#stateless_rule_group_references: {
                        let field_value = match fields_map.get("stateless_rule_group_references") {
                            Some(value) => value,
                            None => bail!("Missing field 'stateless_rule_group_references' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#tls_inspection_configuration_arn: {
                        let field_value = match fields_map.get("tls_inspection_configuration_arn") {
                            Some(value) => value,
                            None => bail!("Missing field 'tls_inspection_configuration_arn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
