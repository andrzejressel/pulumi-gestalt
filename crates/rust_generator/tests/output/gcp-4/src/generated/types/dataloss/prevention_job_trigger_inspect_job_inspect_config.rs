#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PreventionJobTriggerInspectJobInspectConfig {
    /// Custom info types to be used. See https://cloud.google.com/dlp/docs/creating-custom-infotypes to learn more.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "customInfoTypes")]
    pub r#custom_info_types: Option<Vec<super::super::types::dataloss::PreventionJobTriggerInspectJobInspectConfigCustomInfoType>>,
    /// When true, excludes type information of the findings.
    #[builder(into)]
    #[serde(rename = "excludeInfoTypes")]
    pub r#exclude_info_types: Option<bool>,
    /// When true, a contextual quote from the data that triggered a finding is included in the response.
    #[builder(into)]
    #[serde(rename = "includeQuote")]
    pub r#include_quote: Option<bool>,
    /// Restricts what infoTypes to look for. The values must correspond to InfoType values returned by infoTypes.list
    /// or listed at https://cloud.google.com/dlp/docs/infotypes-reference.
    /// When no InfoTypes or CustomInfoTypes are specified in a request, the system may automatically choose what detectors to run.
    /// By default this may be all types, but may change over time as detectors are updated.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "infoTypes")]
    pub r#info_types: Option<Vec<super::super::types::dataloss::PreventionJobTriggerInspectJobInspectConfigInfoType>>,
    /// Configuration to control the number of findings returned.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "limits")]
    pub r#limits: Option<Box<super::super::types::dataloss::PreventionJobTriggerInspectJobInspectConfigLimits>>,
    /// Only returns findings equal or above this threshold. See https://cloud.google.com/dlp/docs/likelihood for more info
    /// Default value is `POSSIBLE`.
    /// Possible values are: `VERY_UNLIKELY`, `UNLIKELY`, `POSSIBLE`, `LIKELY`, `VERY_LIKELY`.
    #[builder(into)]
    #[serde(rename = "minLikelihood")]
    pub r#min_likelihood: Option<String>,
    /// Set of rules to apply to the findings for this InspectConfig. Exclusion rules, contained in the set are executed in the end,
    /// other rules are executed in the order they are specified for each info type.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "ruleSets")]
    pub r#rule_sets: Option<Vec<super::super::types::dataloss::PreventionJobTriggerInspectJobInspectConfigRuleSet>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for PreventionJobTriggerInspectJobInspectConfig {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > + Send {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "custom_info_types",
                    &self.r#custom_info_types,
                ),
                to_pulumi_object_field(
                    "exclude_info_types",
                    &self.r#exclude_info_types,
                ),
                to_pulumi_object_field(
                    "include_quote",
                    &self.r#include_quote,
                ),
                to_pulumi_object_field(
                    "info_types",
                    &self.r#info_types,
                ),
                to_pulumi_object_field(
                    "limits",
                    &self.r#limits,
                ),
                to_pulumi_object_field(
                    "min_likelihood",
                    &self.r#min_likelihood,
                ),
                to_pulumi_object_field(
                    "rule_sets",
                    &self.r#rule_sets,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for PreventionJobTriggerInspectJobInspectConfig {
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
                    r#custom_info_types: {
                        let field_value = match fields_map.get("custom_info_types") {
                            Some(value) => value,
                            None => bail!("Missing field 'custom_info_types' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#exclude_info_types: {
                        let field_value = match fields_map.get("exclude_info_types") {
                            Some(value) => value,
                            None => bail!("Missing field 'exclude_info_types' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#include_quote: {
                        let field_value = match fields_map.get("include_quote") {
                            Some(value) => value,
                            None => bail!("Missing field 'include_quote' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#info_types: {
                        let field_value = match fields_map.get("info_types") {
                            Some(value) => value,
                            None => bail!("Missing field 'info_types' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#limits: {
                        let field_value = match fields_map.get("limits") {
                            Some(value) => value,
                            None => bail!("Missing field 'limits' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#min_likelihood: {
                        let field_value = match fields_map.get("min_likelihood") {
                            Some(value) => value,
                            None => bail!("Missing field 'min_likelihood' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#rule_sets: {
                        let field_value = match fields_map.get("rule_sets") {
                            Some(value) => value,
                            None => bail!("Missing field 'rule_sets' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
