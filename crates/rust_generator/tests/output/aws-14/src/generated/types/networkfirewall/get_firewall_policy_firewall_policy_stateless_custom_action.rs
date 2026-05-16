#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetFirewallPolicyFirewallPolicyStatelessCustomAction {
    #[builder(into)]
    #[serde(rename = "actionDefinitions")]
    pub r#action_definitions: Vec<super::super::types::networkfirewall::GetFirewallPolicyFirewallPolicyStatelessCustomActionActionDefinition>,
    #[builder(into)]
    #[serde(rename = "actionName")]
    pub r#action_name: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetFirewallPolicyFirewallPolicyStatelessCustomAction {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("action_definitions".to_string(), self.r#action_definitions.to_pulumi_value().await);
            map.insert("action_name".to_string(), self.r#action_name.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetFirewallPolicyFirewallPolicyStatelessCustomAction {
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
                    r#action_definitions: {
                        let field_value = match fields_map.get("action_definitions") {
                            Some(value) => value,
                            None => bail!("Missing field 'action_definitions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Vec<super::super::types::networkfirewall::GetFirewallPolicyFirewallPolicyStatelessCustomActionActionDefinition> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#action_name: {
                        let field_value = match fields_map.get("action_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'action_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
