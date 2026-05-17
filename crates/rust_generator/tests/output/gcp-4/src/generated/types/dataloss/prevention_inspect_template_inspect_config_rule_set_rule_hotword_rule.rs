#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PreventionInspectTemplateInspectConfigRuleSetRuleHotwordRule {
    /// Regular expression pattern defining what qualifies as a hotword.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "hotwordRegex")]
    pub r#hotword_regex: Box<super::super::types::dataloss::PreventionInspectTemplateInspectConfigRuleSetRuleHotwordRuleHotwordRegex>,
    /// Likelihood adjustment to apply to all matching findings.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "likelihoodAdjustment")]
    pub r#likelihood_adjustment: Box<super::super::types::dataloss::PreventionInspectTemplateInspectConfigRuleSetRuleHotwordRuleLikelihoodAdjustment>,
    /// Proximity of the finding within which the entire hotword must reside. The total length of the window cannot
    /// exceed 1000 characters. Note that the finding itself will be included in the window, so that hotwords may be
    /// used to match substrings of the finding itself. For example, the certainty of a phone number regex
    /// `(\d{3}) \d{3}-\d{4}` could be adjusted upwards if the area code is known to be the local area code of a company
    /// office using the hotword regex `(xxx)`, where `xxx` is the area code in question.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "proximity")]
    pub r#proximity: Box<super::super::types::dataloss::PreventionInspectTemplateInspectConfigRuleSetRuleHotwordRuleProximity>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for PreventionInspectTemplateInspectConfigRuleSetRuleHotwordRule {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_concurrent, to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "hotword_regex",
                    &self.r#hotword_regex,
                ),
                to_pulumi_object_field(
                    "likelihood_adjustment",
                    &self.r#likelihood_adjustment,
                ),
                to_pulumi_object_field(
                    "proximity",
                    &self.r#proximity,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for PreventionInspectTemplateInspectConfigRuleSetRuleHotwordRule {
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
