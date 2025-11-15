#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct EventDataStoreAdvancedEventSelector {
    /// Specifies the selector statements in an advanced event selector. Fields documented below.
    #[builder(into)]
    #[serde(rename = "fieldSelectors")]
    pub r#field_selectors: Option<Vec<super::super::types::cloudtrail::EventDataStoreAdvancedEventSelectorFieldSelector>>,
    /// Specifies the name of the advanced event selector.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Option<String>,
}
