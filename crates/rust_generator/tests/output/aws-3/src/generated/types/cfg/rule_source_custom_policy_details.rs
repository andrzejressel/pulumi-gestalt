#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RuleSourceCustomPolicyDetails {
    /// The boolean expression for enabling debug logging for your Config Custom Policy rule. The default value is `false`.
    #[builder(into)]
    #[serde(rename = "enableDebugLogDelivery")]
    pub r#enable_debug_log_delivery: Option<bool>,
    /// The runtime system for your Config Custom Policy rule. Guard is a policy-as-code language that allows you to write policies that are enforced by Config Custom Policy rules. For more information about Guard, see the [Guard GitHub Repository](https://github.com/aws-cloudformation/cloudformation-guard).
    #[builder(into)]
    #[serde(rename = "policyRuntime")]
    pub r#policy_runtime: String,
    /// The policy definition containing the logic for your Config Custom Policy rule.
    #[builder(into)]
    #[serde(rename = "policyText")]
    pub r#policy_text: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for RuleSourceCustomPolicyDetails {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("enable_debug_log_delivery".to_string(), self.r#enable_debug_log_delivery.to_pulumi_value().await);
            map.insert("policy_runtime".to_string(), self.r#policy_runtime.to_pulumi_value().await);
            map.insert("policy_text".to_string(), self.r#policy_text.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for RuleSourceCustomPolicyDetails {
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
                    r#enable_debug_log_delivery: {
                        let field_value = match fields_map.get("enable_debug_log_delivery") {
                            Some(value) => value,
                            None => bail!("Missing field 'enable_debug_log_delivery' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<bool> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#policy_runtime: {
                        let field_value = match fields_map.get("policy_runtime") {
                            Some(value) => value,
                            None => bail!("Missing field 'policy_runtime' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#policy_text: {
                        let field_value = match fields_map.get("policy_text") {
                            Some(value) => value,
                            None => bail!("Missing field 'policy_text' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
