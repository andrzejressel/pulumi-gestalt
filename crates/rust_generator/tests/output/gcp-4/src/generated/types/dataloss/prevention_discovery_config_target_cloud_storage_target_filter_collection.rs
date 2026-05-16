#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PreventionDiscoveryConfigTargetCloudStorageTargetFilterCollection {
    /// A collection of regular expressions to match a file store against.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "includeRegexes")]
    pub r#include_regexes: Option<Box<super::super::types::dataloss::PreventionDiscoveryConfigTargetCloudStorageTargetFilterCollectionIncludeRegexes>>,
}
