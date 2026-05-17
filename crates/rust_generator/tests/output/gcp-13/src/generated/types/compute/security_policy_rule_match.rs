#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct SecurityPolicyRuleMatch {
    /// The configuration options available when specifying versionedExpr.
    /// This field must be specified if versionedExpr is specified and cannot be specified if versionedExpr is not specified.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "config")]
    pub r#config: Option<Box<super::super::types::compute::SecurityPolicyRuleMatchConfig>>,
    /// User defined CEVAL expression. A CEVAL expression is used to specify match criteria such as origin.ip, source.region_code and contents in the request header.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "expr")]
    pub r#expr: Option<Box<super::super::types::compute::SecurityPolicyRuleMatchExpr>>,
    /// The configuration options available when specifying a user defined CEVAL expression (i.e., 'expr').
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "exprOptions")]
    pub r#expr_options: Option<Box<super::super::types::compute::SecurityPolicyRuleMatchExprOptions>>,
    /// Preconfigured versioned expression. If this field is specified, config must also be specified.
    /// Available preconfigured expressions along with their requirements are: SRC_IPS_V1 - must specify the corresponding srcIpRange field in config.
    /// Possible values are: `SRC_IPS_V1`.
    #[builder(into)]
    #[serde(rename = "versionedExpr")]
    pub r#versioned_expr: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for SecurityPolicyRuleMatch {
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
                "config".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#config,
                )
                .await,
            );
            map.insert(
                "expr".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#expr,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for SecurityPolicyRuleMatch {
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
                    r#config: {
                        let field_value = match fields_map.get("config") {
                            Some(value) => value,
                            None => bail!("Missing field 'config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#expr: {
                        let field_value = match fields_map.get("expr") {
                            Some(value) => value,
                            None => bail!("Missing field 'expr' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
