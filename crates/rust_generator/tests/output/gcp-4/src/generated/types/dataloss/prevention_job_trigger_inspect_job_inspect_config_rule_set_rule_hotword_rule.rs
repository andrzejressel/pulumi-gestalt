#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PreventionJobTriggerInspectJobInspectConfigRuleSetRuleHotwordRule {
    /// Regular expression pattern defining what qualifies as a hotword.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "hotwordRegex")]
    pub r#hotword_regex: Option<Box<super::super::types::dataloss::PreventionJobTriggerInspectJobInspectConfigRuleSetRuleHotwordRuleHotwordRegex>>,
    /// Likelihood adjustment to apply to all matching findings.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "likelihoodAdjustment")]
    pub r#likelihood_adjustment: Option<Box<super::super::types::dataloss::PreventionJobTriggerInspectJobInspectConfigRuleSetRuleHotwordRuleLikelihoodAdjustment>>,
    /// Proximity of the finding within which the entire hotword must reside. The total length of the window cannot
    /// exceed 1000 characters. Note that the finding itself will be included in the window, so that hotwords may be
    /// used to match substrings of the finding itself. For example, the certainty of a phone number regex
    /// `(\d{3}) \d{3}-\d{4}` could be adjusted upwards if the area code is known to be the local area code of a company
    /// office using the hotword regex `(xxx)`, where `xxx` is the area code in question.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "proximity")]
    pub r#proximity: Option<Box<super::super::types::dataloss::PreventionJobTriggerInspectJobInspectConfigRuleSetRuleHotwordRuleProximity>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for PreventionJobTriggerInspectJobInspectConfigRuleSetRuleHotwordRule {
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
                "hotword_regex".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#hotword_regex,
                )
                .await,
            );
            map.insert(
                "likelihood_adjustment".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#likelihood_adjustment,
                )
                .await,
            );
            map.insert(
                "proximity".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#proximity,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for PreventionJobTriggerInspectJobInspectConfigRuleSetRuleHotwordRule {
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
                    r#hotword_regex: {
                        let field_value = match fields_map.get("hotword_regex") {
                            Some(value) => value,
                            None => bail!("Missing field 'hotword_regex' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#likelihood_adjustment: {
                        let field_value = match fields_map.get("likelihood_adjustment") {
                            Some(value) => value,
                            None => bail!("Missing field 'likelihood_adjustment' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#proximity: {
                        let field_value = match fields_map.get("proximity") {
                            Some(value) => value,
                            None => bail!("Missing field 'proximity' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
