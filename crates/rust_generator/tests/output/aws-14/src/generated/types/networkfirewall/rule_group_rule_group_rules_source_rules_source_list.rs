#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RuleGroupRuleGroupRulesSourceRulesSourceList {
    /// String value to specify whether domains in the target list are allowed or denied access. Valid values: `ALLOWLIST`, `DENYLIST`.
    #[builder(into)]
    #[serde(rename = "generatedRulesType")]
    pub r#generated_rules_type: String,
    /// Set of types of domain specifications that are provided in the `targets` argument. Valid values: `HTTP_HOST`, `TLS_SNI`.
    #[builder(into)]
    #[serde(rename = "targetTypes")]
    pub r#target_types: Vec<String>,
    /// Set of domains that you want to inspect for in your traffic flows.
    #[builder(into)]
    #[serde(rename = "targets")]
    pub r#targets: Vec<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for RuleGroupRuleGroupRulesSourceRulesSourceList {
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
                    "generated_rules_type",
                    &self.r#generated_rules_type,
                ),
                to_pulumi_object_field(
                    "target_types",
                    &self.r#target_types,
                ),
                to_pulumi_object_field(
                    "targets",
                    &self.r#targets,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for RuleGroupRuleGroupRulesSourceRulesSourceList {
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
                    r#generated_rules_type: {
                        let field_value = match fields_map.get("generated_rules_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'generated_rules_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#target_types: {
                        let field_value = match fields_map.get("target_types") {
                            Some(value) => value,
                            None => bail!("Missing field 'target_types' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#targets: {
                        let field_value = match fields_map.get("targets") {
                            Some(value) => value,
                            None => bail!("Missing field 'targets' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
