#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PolicySpecRule {
    /// Setting this to `"TRUE"` means that all values are allowed. This field can be set only in Policies for list constraints.
    #[builder(into)]
    #[serde(rename = "allowAll")]
    pub r#allow_all: Option<String>,
    /// A condition which determines whether this rule is used in the evaluation of the policy. When set, the `expression` field in the `Expr' must include from 1 to 10 subexpressions, joined by the "||" or "&&" operators. Each subexpression must be of the form "resource.matchTag('/tag_key_short_name, 'tag_value_short_name')". or "resource.matchTagId('tagKeys/key_id', 'tagValues/value_id')". where key_name and value_name are the resource names for Label Keys and Values. These names are available from the Tag Manager Service. An example expression is: "resource.matchTag('123456789/environment, 'prod')". or "resource.matchTagId('tagKeys/123', 'tagValues/456')".
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "condition")]
    pub r#condition: Option<Box<super::super::types::orgpolicy::PolicySpecRuleCondition>>,
    /// Setting this to `"TRUE"` means that all values are denied. This field can be set only in Policies for list constraints.
    #[builder(into)]
    #[serde(rename = "denyAll")]
    pub r#deny_all: Option<String>,
    /// If `"TRUE"`, then the `Policy` is enforced. If `"FALSE"`, then any configuration is acceptable. This field can be set only in Policies for boolean constraints.
    #[builder(into)]
    #[serde(rename = "enforce")]
    pub r#enforce: Option<String>,
    /// Optional. Required for Managed Constraints if parameters defined in constraints. Pass parameter values when policy enforcement is enabled. Ensure that parameter value types match those defined in the constraint definition. For example: { \"allowedLocations\" : [\"us-east1\", \"us-west1\"], \"allowAll\" : true }
    #[builder(into)]
    #[serde(rename = "parameters")]
    pub r#parameters: Option<String>,
    /// List of values to be used for this policy rule. This field can be set only in policies for list constraints.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "values")]
    pub r#values: Option<Box<super::super::types::orgpolicy::PolicySpecRuleValues>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for PolicySpecRule {
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
                "parameters".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#parameters,
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
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for PolicySpecRule {
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
                    r#parameters: {
                        let field_value = match fields_map.get("parameters") {
                            Some(value) => value,
                            None => bail!("Missing field 'parameters' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
