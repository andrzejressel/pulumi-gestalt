#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FirewallPolicyFirewallPolicyStatefulEngineOptions {
    /// Amount of time that can pass without any traffic sent through the firewall before the firewall determines that the connection is idle.
    #[builder(into)]
    #[serde(rename = "flowTimeouts")]
    pub r#flow_timeouts: Option<Box<super::super::types::networkfirewall::FirewallPolicyFirewallPolicyStatefulEngineOptionsFlowTimeouts>>,
    /// Indicates how to manage the order of stateful rule evaluation for the policy. Default value: `DEFAULT_ACTION_ORDER`. Valid values: `DEFAULT_ACTION_ORDER`, `STRICT_ORDER`.
    #[builder(into)]
    #[serde(rename = "ruleOrder")]
    pub r#rule_order: Option<String>,
    /// Describes how to treat traffic which has broken midstream. Default value: `DROP`. Valid values: `DROP`, `CONTINUE`, `REJECT`.
    #[builder(into)]
    #[serde(rename = "streamExceptionPolicy")]
    pub r#stream_exception_policy: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for FirewallPolicyFirewallPolicyStatefulEngineOptions {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("flow_timeouts".to_string(), self.r#flow_timeouts.to_pulumi_value().await);
            map.insert("rule_order".to_string(), self.r#rule_order.to_pulumi_value().await);
            map.insert("stream_exception_policy".to_string(), self.r#stream_exception_policy.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for FirewallPolicyFirewallPolicyStatefulEngineOptions {
    fn from_pulumi_value(
        value: &pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    ) -> pulumi_gestalt_rust::__private::rootcause::Result<Self> {
        use std::collections::BTreeMap;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValueContent;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue;
        use pulumi_gestalt_rust::__private::rootcause::bail;

        match value.content {
            PulumiValueContent::Object(ref obj) => {
                let fields_map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> =
                    obj.iter().cloned().collect();

                Ok(Self {
                    r#flow_timeouts: {
                        let field_value = match fields_map.get("flow_timeouts") {
                            Some(value) => value,
                            None => bail!("Missing field 'flow_timeouts' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::networkfirewall::FirewallPolicyFirewallPolicyStatefulEngineOptionsFlowTimeouts>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#rule_order: {
                        let field_value = match fields_map.get("rule_order") {
                            Some(value) => value,
                            None => bail!("Missing field 'rule_order' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#stream_exception_policy: {
                        let field_value = match fields_map.get("stream_exception_policy") {
                            Some(value) => value,
                            None => bail!("Missing field 'stream_exception_policy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
