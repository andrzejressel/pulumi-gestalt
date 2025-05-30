#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetResourcesResourceTagMappingListComplianceDetail {
    /// Whether the resource is compliant.
    /// * `keys_with_noncompliant_values ` - Set of tag keys with non-compliant tag values.
    /// * `non_compliant_keys ` - Set of non-compliant tag keys.
    #[builder(into)]
    #[serde(rename = "complianceStatus")]
    pub r#compliance_status: Box<bool>,
    #[builder(into)]
    #[serde(rename = "keysWithNoncompliantValues")]
    pub r#keys_with_noncompliant_values: Box<Vec<String>>,
    #[builder(into)]
    #[serde(rename = "nonCompliantKeys")]
    pub r#non_compliant_keys: Box<Vec<String>>,
}
