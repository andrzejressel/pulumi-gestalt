#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetRouterNatRule {
    /// The action to be enforced for traffic that matches this rule.
    #[builder(into)]
    #[serde(rename = "actions")]
    pub r#actions: Vec<super::super::types::compute::GetRouterNatRuleAction>,
    /// An optional description of this rule.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: String,
    /// CEL expression that specifies the match condition that egress traffic from a VM is evaluated against.
    /// If it evaluates to true, the corresponding action is enforced.
    /// 
    /// The following examples are valid match expressions for public NAT:
    /// 
    /// "inIpRange(destination.ip, '1.1.0.0/16') || inIpRange(destination.ip, '2.2.0.0/16')"
    /// 
    /// "destination.ip == '1.1.0.1' || destination.ip == '8.8.8.8'"
    /// 
    /// The following example is a valid match expression for private NAT:
    /// 
    /// "nexthop.hub == 'https://networkconnectivity.googleapis.com/v1alpha1/projects/my-project/global/hub/hub-1'"
    #[builder(into)]
    #[serde(rename = "match")]
    pub r#match_: String,
    /// An integer uniquely identifying a rule in the list.
    /// The rule number must be a positive value between 0 and 65000, and must be unique among rules within a NAT.
    #[builder(into)]
    #[serde(rename = "ruleNumber")]
    pub r#rule_number: i32,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetRouterNatRule {
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
                    "actions",
                    &self.r#actions,
                ),
                to_pulumi_object_field(
                    "description",
                    &self.r#description,
                ),
                to_pulumi_object_field(
                    "match_",
                    &self.r#match_,
                ),
                to_pulumi_object_field(
                    "rule_number",
                    &self.r#rule_number,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetRouterNatRule {
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
                    r#actions: {
                        let field_value = match fields_map.get("actions") {
                            Some(value) => value,
                            None => bail!("Missing field 'actions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#match_: {
                        let field_value = match fields_map.get("match_") {
                            Some(value) => value,
                            None => bail!("Missing field 'match_' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#rule_number: {
                        let field_value = match fields_map.get("rule_number") {
                            Some(value) => value,
                            None => bail!("Missing field 'rule_number' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
