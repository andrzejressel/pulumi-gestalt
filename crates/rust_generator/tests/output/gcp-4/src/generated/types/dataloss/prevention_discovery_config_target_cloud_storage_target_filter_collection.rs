#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PreventionDiscoveryConfigTargetCloudStorageTargetFilterCollection {
    /// A collection of regular expressions to match a file store against.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "includeRegexes")]
    pub r#include_regexes: Option<Box<super::super::types::dataloss::PreventionDiscoveryConfigTargetCloudStorageTargetFilterCollectionIncludeRegexes>>,
}
