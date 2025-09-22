#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ConnectorCapacity {
    /// Information about the auto scaling parameters for the connector. See `autoscaling` Block for details.
    #[builder(into)]
    #[serde(rename = "autoscaling")]
    pub r#autoscaling: Option<Box<super::super::types::mskconnect::ConnectorCapacityAutoscaling>>,
    /// Details about a fixed capacity allocated to a connector. See `provisioned_capacity` Block for details.
    #[builder(into)]
    #[serde(rename = "provisionedCapacity")]
    pub r#provisioned_capacity: Option<Box<super::super::types::mskconnect::ConnectorCapacityProvisionedCapacity>>,
}
