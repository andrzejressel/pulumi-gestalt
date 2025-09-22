#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ServicePerimeterStatusEgressPolicy {
    /// Defines conditions on the source of a request causing this `EgressPolicy` to apply.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "egressFrom")]
    pub r#egress_from: Box<Option<super::super::types::accesscontextmanager::ServicePerimeterStatusEgressPolicyEgressFrom>>,
    /// Defines the conditions on the `ApiOperation` and destination resources that
    /// cause this `EgressPolicy` to apply.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "egressTo")]
    pub r#egress_to: Box<Option<super::super::types::accesscontextmanager::ServicePerimeterStatusEgressPolicyEgressTo>>,
}
