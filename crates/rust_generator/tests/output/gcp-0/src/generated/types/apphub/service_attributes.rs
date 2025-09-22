#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ServiceAttributes {
    /// Business team that ensures user needs are met and value is delivered
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "businessOwners")]
    pub r#business_owners: Option<Vec<super::super::types::apphub::ServiceAttributesBusinessOwner>>,
    /// Criticality of the Application, Service, or Workload
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "criticality")]
    pub r#criticality: Option<Box<super::super::types::apphub::ServiceAttributesCriticality>>,
    /// Developer team that owns development and coding.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "developerOwners")]
    pub r#developer_owners: Option<Vec<super::super::types::apphub::ServiceAttributesDeveloperOwner>>,
    /// Environment of the Application, Service, or Workload
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "environment")]
    pub r#environment: Option<Box<super::super::types::apphub::ServiceAttributesEnvironment>>,
    /// Operator team that ensures runtime and operations.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "operatorOwners")]
    pub r#operator_owners: Option<Vec<super::super::types::apphub::ServiceAttributesOperatorOwner>>,
}
