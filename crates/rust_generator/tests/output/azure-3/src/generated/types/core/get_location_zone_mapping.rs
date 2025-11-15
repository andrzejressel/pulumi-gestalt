#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetLocationZoneMapping {
    /// The logical zone id for the availability zone
    #[builder(into)]
    #[serde(rename = "logicalZone")]
    pub r#logical_zone: String,
    /// The fully qualified physical zone id of availability zone to which logical zone id is mapped to
    #[builder(into)]
    #[serde(rename = "physicalZone")]
    pub r#physical_zone: String,
}
