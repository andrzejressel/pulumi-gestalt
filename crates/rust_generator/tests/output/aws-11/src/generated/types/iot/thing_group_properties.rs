#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ThingGroupProperties {
    /// The Thing Group attributes. Defined below.
    #[builder(into)]
    #[serde(rename = "attributePayload")]
    pub r#attribute_payload: Option<Box<super::super::types::iot::ThingGroupPropertiesAttributePayload>>,
    /// A description of the Thing Group.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Option<String>,
}
