#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct SlotTypeEnumerationValue {
    /// Additional values related to the slot type value. Each item must be less than or equal to 140 characters in length.
    #[builder(into, default)]
    #[serde(rename = "synonyms")]
    pub r#synonyms: Box<Option<Vec<String>>>,
    /// The value of the slot type. Must be less than or equal to 140 characters in length.
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: Box<String>,
}
