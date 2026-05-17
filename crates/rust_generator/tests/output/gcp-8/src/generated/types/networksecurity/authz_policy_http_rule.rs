#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AuthzPolicyHttpRule {
    /// Describes properties of one or more sources of a request.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "from")]
    pub r#from: Option<Box<super::super::types::networksecurity::AuthzPolicyHttpRuleFrom>>,
    /// Describes properties of one or more targets of a request
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "to")]
    pub r#to: Option<Box<super::super::types::networksecurity::AuthzPolicyHttpRuleTo>>,
    /// CEL expression that describes the conditions to be satisfied for the action. The result of the CEL expression is ANDed with the from and to. Refer to the CEL language reference for a list of available attributes.
    #[builder(into)]
    #[serde(rename = "when")]
    pub r#when: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for AuthzPolicyHttpRule {
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
                "from".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#from,
                )
                .await,
            );
            map.insert(
                "to".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#to,
                )
                .await,
            );
            map.insert(
                "when".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#when,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for AuthzPolicyHttpRule {
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
                    r#from: {
                        let field_value = match fields_map.get("from") {
                            Some(value) => value,
                            None => bail!("Missing field 'from' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#to: {
                        let field_value = match fields_map.get("to") {
                            Some(value) => value,
                            None => bail!("Missing field 'to' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#when: {
                        let field_value = match fields_map.get("when") {
                            Some(value) => value,
                            None => bail!("Missing field 'when' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
