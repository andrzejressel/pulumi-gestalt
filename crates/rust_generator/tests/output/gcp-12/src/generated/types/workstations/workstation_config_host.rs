#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct WorkstationConfigHost {
    /// A runtime using a Compute Engine instance.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "gceInstance")]
    pub r#gce_instance: Option<Box<super::super::types::workstations::WorkstationConfigHostGceInstance>>,
}
