#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct OrganizationSecurityPolicyRuleMatch {
    /// The configuration options for matching the rule.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "config")]
    pub r#config: Box<super::super::types::compute::OrganizationSecurityPolicyRuleMatchConfig>,
    /// A description of the rule.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    /// Preconfigured versioned expression. For organization security policy rules,
    /// the only supported type is "FIREWALL".
    /// Default value is `FIREWALL`.
    /// Possible values are: `FIREWALL`.
    #[builder(into)]
    #[serde(rename = "versionedExpr")]
    pub r#versioned_expr: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for OrganizationSecurityPolicyRuleMatch {
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
                "description".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#description,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for OrganizationSecurityPolicyRuleMatch {
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
                    r#description: {
                        let field_value = match fields_map.get("description") {
                            Some(value) => value,
                            None => bail!("Missing field 'description' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
