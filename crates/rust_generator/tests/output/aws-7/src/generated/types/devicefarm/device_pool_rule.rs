#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct DevicePoolRule {
    /// The rule's stringified attribute. Valid values are: `APPIUM_VERSION`, `ARN`, `AVAILABILITY`, `FLEET_TYPE`, `FORM_FACTOR`, `INSTANCE_ARN`, `INSTANCE_LABELS`, `MANUFACTURER`, `MODEL`, `OS_VERSION`, `PLATFORM`, `REMOTE_ACCESS_ENABLED`, `REMOTE_DEBUG_ENABLED`.
    #[builder(into)]
    #[serde(rename = "attribute")]
    pub r#attribute: Option<String>,
    /// Specifies how Device Farm compares the rule's attribute to the value. For the operators that are supported by each attribute. Valid values are: `EQUALS`, `NOT_IN`, `IN`, `GREATER_THAN`, `GREATER_THAN_OR_EQUALS`, `LESS_THAN`, `LESS_THAN_OR_EQUALS`, `CONTAINS`.
    #[builder(into)]
    #[serde(rename = "operator")]
    pub r#operator: Option<String>,
    /// The rule's value.
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: Option<String>,
}
