#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ServiceTaskSpecResources {
    /// Describes the resources which can be advertised by a node and requested by a task
    #[builder(into, default)]
    #[serde(rename = "limits")]
    pub r#limits: Box<Option<super::types::ServiceTaskSpecResourcesLimits>>,
    /// An object describing the resources which can be advertised by a node and requested by a task
    #[builder(into, default)]
    #[serde(rename = "reservation")]
    pub r#reservation: Box<Option<super::types::ServiceTaskSpecResourcesReservation>>,
}
