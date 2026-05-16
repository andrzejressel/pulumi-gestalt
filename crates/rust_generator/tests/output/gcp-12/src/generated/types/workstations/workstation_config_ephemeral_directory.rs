#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct WorkstationConfigEphemeralDirectory {
    /// An EphemeralDirectory backed by a Compute Engine persistent disk.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "gcePd")]
    pub r#gce_pd: Option<Box<super::super::types::workstations::WorkstationConfigEphemeralDirectoryGcePd>>,
    /// Location of this directory in the running workstation.
    #[builder(into)]
    #[serde(rename = "mountPath")]
    pub r#mount_path: Option<String>,
}
