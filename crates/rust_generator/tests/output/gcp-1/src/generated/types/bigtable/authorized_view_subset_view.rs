#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct AuthorizedViewSubsetView {
    /// A group of column family subsets to be included in the authorized view. This can be specified multiple times. Structure is documented below.
    /// 
    /// -----
    #[builder(into)]
    #[serde(rename = "familySubsets")]
    pub r#family_subsets: Option<Vec<super::super::types::bigtable::AuthorizedViewSubsetViewFamilySubset>>,
    /// A list of Base64-encoded row prefixes to be included in the authorized view. To provide access to all rows, include the empty string as a prefix ("").
    #[builder(into)]
    #[serde(rename = "rowPrefixes")]
    pub r#row_prefixes: Option<Vec<String>>,
}
