#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PosturePolicySetPolicyConstraintOrgPolicyConstraintPolicyRule {
    /// Setting this to true means that all values are allowed. This field can be set only in policies for list constraints.
    #[builder(into)]
    #[serde(rename = "allowAll")]
    pub r#allow_all: Option<bool>,
    /// Represents a textual expression in the Common Expression Language (CEL) syntax. CEL is a C-like expression language.
    /// This page details the objects and attributes that are used to the build the CEL expressions for
    /// custom access levels - https://cloud.google.com/access-context-manager/docs/custom-access-level-spec.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "condition")]
    pub r#condition: Option<Box<super::super::types::securityposture::PosturePolicySetPolicyConstraintOrgPolicyConstraintPolicyRuleCondition>>,
    /// Setting this to true means that all values are denied. This field can be set only in policies for list constraints.
    #[builder(into)]
    #[serde(rename = "denyAll")]
    pub r#deny_all: Option<bool>,
    /// If `true`, then the policy is enforced. If `false`, then any configuration is acceptable.
    /// This field can be set only in policies for boolean constraints.
    #[builder(into)]
    #[serde(rename = "enforce")]
    pub r#enforce: Option<bool>,
    /// List of values to be used for this policy rule. This field can be set only in policies for list constraints.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "values")]
    pub r#values: Option<Box<super::super::types::securityposture::PosturePolicySetPolicyConstraintOrgPolicyConstraintPolicyRuleValues>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for PosturePolicySetPolicyConstraintOrgPolicyConstraintPolicyRule {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "allow_all".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#allow_all,
                )
                .await,
            );
            map.insert(
                "condition".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#condition,
                )
                .await,
            );
            map.insert(
                "deny_all".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#deny_all,
                )
                .await,
            );
            map.insert(
                "enforce".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#enforce,
                )
                .await,
            );
            map.insert(
                "values".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#values,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for PosturePolicySetPolicyConstraintOrgPolicyConstraintPolicyRule {
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
                    r#allow_all: {
                        let field_value = match fields_map.get("allow_all") {
                            Some(value) => value,
                            None => bail!("Missing field 'allow_all' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#condition: {
                        let field_value = match fields_map.get("condition") {
                            Some(value) => value,
                            None => bail!("Missing field 'condition' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#deny_all: {
                        let field_value = match fields_map.get("deny_all") {
                            Some(value) => value,
                            None => bail!("Missing field 'deny_all' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#enforce: {
                        let field_value = match fields_map.get("enforce") {
                            Some(value) => value,
                            None => bail!("Missing field 'enforce' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#values: {
                        let field_value = match fields_map.get("values") {
                            Some(value) => value,
                            None => bail!("Missing field 'values' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
