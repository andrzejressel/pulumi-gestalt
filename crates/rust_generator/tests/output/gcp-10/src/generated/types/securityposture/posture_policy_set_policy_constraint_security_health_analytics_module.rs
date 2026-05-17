#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PosturePolicySetPolicyConstraintSecurityHealthAnalyticsModule {
    /// The state of enablement for the module at its level of the resource hierarchy.
    /// Possible values are: `ENABLEMENT_STATE_UNSPECIFIED`, `ENABLED`, `DISABLED`.
    #[builder(into)]
    #[serde(rename = "moduleEnablementState")]
    pub r#module_enablement_state: Option<String>,
    /// The name of the module eg: BIGQUERY_TABLE_CMEK_DISABLED.
    #[builder(into)]
    #[serde(rename = "moduleName")]
    pub r#module_name: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for PosturePolicySetPolicyConstraintSecurityHealthAnalyticsModule {
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
                "module_enablement_state".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#module_enablement_state,
                )
                .await,
            );
            map.insert(
                "module_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#module_name,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for PosturePolicySetPolicyConstraintSecurityHealthAnalyticsModule {
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
                    r#module_enablement_state: {
                        let field_value = match fields_map.get("module_enablement_state") {
                            Some(value) => value,
                            None => bail!("Missing field 'module_enablement_state' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#module_name: {
                        let field_value = match fields_map.get("module_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'module_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
