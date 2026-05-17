#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PreventionJobTriggerInspectJobInspectConfigRuleSetRuleHotwordRuleLikelihoodAdjustment {
    /// Set the likelihood of a finding to a fixed value. Either this or relative_likelihood can be set.
    /// Possible values are: `VERY_UNLIKELY`, `UNLIKELY`, `POSSIBLE`, `LIKELY`, `VERY_LIKELY`.
    #[builder(into)]
    #[serde(rename = "fixedLikelihood")]
    pub r#fixed_likelihood: Option<String>,
    /// Increase or decrease the likelihood by the specified number of levels. For example,
    /// if a finding would be POSSIBLE without the detection rule and relativeLikelihood is 1,
    /// then it is upgraded to LIKELY, while a value of -1 would downgrade it to UNLIKELY.
    /// Likelihood may never drop below VERY_UNLIKELY or exceed VERY_LIKELY, so applying an
    /// adjustment of 1 followed by an adjustment of -1 when base likelihood is VERY_LIKELY
    /// will result in a final likelihood of LIKELY. Either this or fixed_likelihood can be set.
    #[builder(into)]
    #[serde(rename = "relativeLikelihood")]
    pub r#relative_likelihood: Option<i32>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for PreventionJobTriggerInspectJobInspectConfigRuleSetRuleHotwordRuleLikelihoodAdjustment {
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
                "fixed_likelihood".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#fixed_likelihood,
                )
                .await,
            );
            map.insert(
                "relative_likelihood".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#relative_likelihood,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for PreventionJobTriggerInspectJobInspectConfigRuleSetRuleHotwordRuleLikelihoodAdjustment {
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
                    r#fixed_likelihood: {
                        let field_value = match fields_map.get("fixed_likelihood") {
                            Some(value) => value,
                            None => bail!("Missing field 'fixed_likelihood' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#relative_likelihood: {
                        let field_value = match fields_map.get("relative_likelihood") {
                            Some(value) => value,
                            None => bail!("Missing field 'relative_likelihood' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
