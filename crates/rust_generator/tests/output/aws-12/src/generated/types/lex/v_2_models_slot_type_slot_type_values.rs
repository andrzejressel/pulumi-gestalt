#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct V2ModelsSlotTypeSlotTypeValues {
    /// Value of the slot type entry.
    /// See `sample_value` argument reference below.
    #[builder(into)]
    #[serde(rename = "sampleValues")]
    pub r#sample_values: Option<Vec<super::super::types::lex::V2ModelsSlotTypeSlotTypeValuesSampleValue>>,
    /// A list of additional values related to the slot type entry.
    /// See `synonyms` argument reference below.
    #[builder(into)]
    #[serde(rename = "synonyms")]
    pub r#synonyms: Option<Vec<super::super::types::lex::V2ModelsSlotTypeSlotTypeValuesSynonym>>,
}
