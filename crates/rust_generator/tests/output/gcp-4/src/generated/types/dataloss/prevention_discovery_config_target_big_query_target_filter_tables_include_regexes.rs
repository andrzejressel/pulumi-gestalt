#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PreventionDiscoveryConfigTargetBigQueryTargetFilterTablesIncludeRegexes {
    /// The group of regular expression patterns to match against one or more file stores. Maximum of 100 entries. The sum of all lengths of regular expressions can't exceed 10 KiB.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "patterns")]
    pub r#patterns: Option<Vec<super::super::types::dataloss::PreventionDiscoveryConfigTargetBigQueryTargetFilterTablesIncludeRegexesPattern>>,
}
