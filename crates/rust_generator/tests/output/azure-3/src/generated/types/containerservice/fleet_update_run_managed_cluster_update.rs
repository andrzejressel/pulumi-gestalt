#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FleetUpdateRunManagedClusterUpdate {
    /// A `node_image_selection` block as defined below.
    #[builder(into)]
    #[serde(rename = "nodeImageSelection")]
    pub r#node_image_selection: Option<Box<super::super::types::containerservice::FleetUpdateRunManagedClusterUpdateNodeImageSelection>>,
    /// A `upgrade` block as defined below.
    #[builder(into)]
    #[serde(rename = "upgrade")]
    pub r#upgrade: Box<super::super::types::containerservice::FleetUpdateRunManagedClusterUpdateUpgrade>,
}
