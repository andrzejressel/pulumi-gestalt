#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RecordSetRoutingPolicyPrimaryBackupBackupGeo {
    /// For A and AAAA types only. The list of targets to be health checked. These can be specified along with `rrdatas` within this item.
    #[builder(into)]
    #[serde(rename = "healthCheckedTargets")]
    pub r#health_checked_targets: Option<Box<super::super::types::dns::RecordSetRoutingPolicyPrimaryBackupBackupGeoHealthCheckedTargets>>,
    /// The location name defined in Google Cloud.
    #[builder(into)]
    #[serde(rename = "location")]
    pub r#location: String,
    #[builder(into)]
    #[serde(rename = "rrdatas")]
    pub r#rrdatas: Option<Vec<String>>,
}
