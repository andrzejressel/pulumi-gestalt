#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetSecurityPolicyRuleMatch {
    /// The configuration options available when specifying versioned_expr. This field must be specified if versioned_expr is specified and cannot be specified if versioned_expr is not specified.
    #[builder(into)]
    #[serde(rename = "configs")]
    pub r#configs: Vec<super::super::types::compute::GetSecurityPolicyRuleMatchConfig>,
    /// The configuration options available when specifying a user defined CEVAL expression (i.e., 'expr').
    #[builder(into)]
    #[serde(rename = "exprOptions")]
    pub r#expr_options: Vec<super::super::types::compute::GetSecurityPolicyRuleMatchExprOption>,
    /// User defined CEVAL expression. A CEVAL expression is used to specify match criteria such as origin.ip, source.region_code and contents in the request header.
    #[builder(into)]
    #[serde(rename = "exprs")]
    pub r#exprs: Vec<super::super::types::compute::GetSecurityPolicyRuleMatchExpr>,
    /// Predefined rule expression. If this field is specified, config must also be specified. Available options:   SRC_IPS_V1: Must specify the corresponding src_ip_ranges field in config.
    #[builder(into)]
    #[serde(rename = "versionedExpr")]
    pub r#versioned_expr: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetSecurityPolicyRuleMatch {
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
                "configs".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#configs,
                )
                .await,
            );
            map.insert(
                "expr_options".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#expr_options,
                )
                .await,
            );
            map.insert(
                "exprs".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#exprs,
                )
                .await,
            );
            map.insert(
                "versioned_expr".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#versioned_expr,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetSecurityPolicyRuleMatch {
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
                    r#configs: {
                        let field_value = match fields_map.get("configs") {
                            Some(value) => value,
                            None => bail!("Missing field 'configs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#expr_options: {
                        let field_value = match fields_map.get("expr_options") {
                            Some(value) => value,
                            None => bail!("Missing field 'expr_options' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#exprs: {
                        let field_value = match fields_map.get("exprs") {
                            Some(value) => value,
                            None => bail!("Missing field 'exprs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#versioned_expr: {
                        let field_value = match fields_map.get("versioned_expr") {
                            Some(value) => value,
                            None => bail!("Missing field 'versioned_expr' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
