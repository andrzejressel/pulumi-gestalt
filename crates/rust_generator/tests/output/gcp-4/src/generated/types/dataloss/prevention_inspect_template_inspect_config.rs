#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PreventionInspectTemplateInspectConfig {
    /// List of options defining data content to scan. If empty, text, images, and other content will be included.
    /// Each value may be one of: `CONTENT_TEXT`, `CONTENT_IMAGE`.
    #[builder(into)]
    #[serde(rename = "contentOptions")]
    pub r#content_options: Option<Vec<String>>,
    /// Custom info types to be used. See https://cloud.google.com/dlp/docs/creating-custom-infotypes to learn more.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "customInfoTypes")]
    pub r#custom_info_types: Option<Vec<super::super::types::dataloss::PreventionInspectTemplateInspectConfigCustomInfoType>>,
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
    pub r#info_types: Option<Vec<super::super::types::dataloss::PreventionInspectTemplateInspectConfigInfoType>>,
    /// Configuration to control the number of findings returned.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "limits")]
    pub r#limits: Option<Box<super::super::types::dataloss::PreventionInspectTemplateInspectConfigLimits>>,
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
    pub r#rule_sets: Option<Vec<super::super::types::dataloss::PreventionInspectTemplateInspectConfigRuleSet>>,
}
