#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FrontdoorFirewallPolicyManagedRuleOverrideRule {
    /// The action to be applied when the managed rule matches or when the anomaly score is 5 or greater. Possible values for DRS `1.1` and below are `Allow`, `Log`, `Block`, and `Redirect`. For DRS `2.0` and above the possible values are `Log` or `AnomalyScoring`.
    /// 
    /// ->**NOTE:** Please see the DRS [product documentation](https://learn.microsoft.com/azure/web-application-firewall/afds/waf-front-door-drs?tabs=drs20#anomaly-scoring-mode) for more information.
    #[builder(into)]
    #[serde(rename = "action")]
    pub r#action: String,
    /// Is the managed rule override enabled or disabled. Defaults to `false`
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Option<bool>,
    /// One or more `exclusion` blocks as defined below.
    #[builder(into)]
    #[serde(rename = "exclusions")]
    pub r#exclusions: Option<Vec<super::super::types::cdn::FrontdoorFirewallPolicyManagedRuleOverrideRuleExclusion>>,
    /// Identifier for the managed rule.
    #[builder(into)]
    #[serde(rename = "ruleId")]
    pub r#rule_id: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for FrontdoorFirewallPolicyManagedRuleOverrideRule {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "action".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#action,
                )
                .await,
            );
            map.insert(
                "enabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#enabled,
                )
                .await,
            );
            map.insert(
                "exclusions".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#exclusions,
                )
                .await,
            );
            map.insert(
                "rule_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#rule_id,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for FrontdoorFirewallPolicyManagedRuleOverrideRule {
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
                    r#action: {
                        let field_value = match fields_map.get("action") {
                            Some(value) => value,
                            None => bail!("Missing field 'action' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#enabled: {
                        let field_value = match fields_map.get("enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#exclusions: {
                        let field_value = match fields_map.get("exclusions") {
                            Some(value) => value,
                            None => bail!("Missing field 'exclusions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#rule_id: {
                        let field_value = match fields_map.get("rule_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'rule_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
