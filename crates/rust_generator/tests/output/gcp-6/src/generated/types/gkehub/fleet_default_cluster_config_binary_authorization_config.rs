#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FleetDefaultClusterConfigBinaryAuthorizationConfig {
    /// Mode of operation for binauthz policy evaluation.
    /// Possible values are: `DISABLED`, `POLICY_BINDINGS`.
    #[builder(into)]
    #[serde(rename = "evaluationMode")]
    pub r#evaluation_mode: Option<String>,
    /// Binauthz policies that apply to this cluster.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "policyBindings")]
    pub r#policy_bindings: Option<Vec<super::super::types::gkehub::FleetDefaultClusterConfigBinaryAuthorizationConfigPolicyBinding>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for FleetDefaultClusterConfigBinaryAuthorizationConfig {
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
                "evaluation_mode".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#evaluation_mode,
                )
                .await,
            );
            map.insert(
                "policy_bindings".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#policy_bindings,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for FleetDefaultClusterConfigBinaryAuthorizationConfig {
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
                    r#evaluation_mode: {
                        let field_value = match fields_map.get("evaluation_mode") {
                            Some(value) => value,
                            None => bail!("Missing field 'evaluation_mode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#policy_bindings: {
                        let field_value = match fields_map.get("policy_bindings") {
                            Some(value) => value,
                            None => bail!("Missing field 'policy_bindings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
