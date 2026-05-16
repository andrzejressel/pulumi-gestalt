#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct LifecyclePolicyPolicyDetail {
    /// Configuration details for the policy action.
    #[builder(into)]
    #[serde(rename = "action")]
    pub r#action: Option<Box<super::super::types::imagebuilder::LifecyclePolicyPolicyDetailAction>>,
    /// Additional rules to specify resources that should be exempt from policy actions.
    #[builder(into)]
    #[serde(rename = "exclusionRules")]
    pub r#exclusion_rules: Option<Box<super::super::types::imagebuilder::LifecyclePolicyPolicyDetailExclusionRules>>,
    /// Specifies the resources that the lifecycle policy applies to.
    /// 
    /// The following arguments are optional:
    #[builder(into)]
    #[serde(rename = "filter")]
    pub r#filter: Option<Box<super::super::types::imagebuilder::LifecyclePolicyPolicyDetailFilter>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for LifecyclePolicyPolicyDetail {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("action".to_string(), self.r#action.to_pulumi_value().await);
            map.insert("exclusion_rules".to_string(), self.r#exclusion_rules.to_pulumi_value().await);
            map.insert("filter".to_string(), self.r#filter.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for LifecyclePolicyPolicyDetail {
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
                    r#action: {
                        let field_value = match fields_map.get("action") {
                            Some(value) => value,
                            None => bail!("Missing field 'action' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::imagebuilder::LifecyclePolicyPolicyDetailAction>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#exclusion_rules: {
                        let field_value = match fields_map.get("exclusion_rules") {
                            Some(value) => value,
                            None => bail!("Missing field 'exclusion_rules' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::imagebuilder::LifecyclePolicyPolicyDetailExclusionRules>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#filter: {
                        let field_value = match fields_map.get("filter") {
                            Some(value) => value,
                            None => bail!("Missing field 'filter' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::imagebuilder::LifecyclePolicyPolicyDetailFilter>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
