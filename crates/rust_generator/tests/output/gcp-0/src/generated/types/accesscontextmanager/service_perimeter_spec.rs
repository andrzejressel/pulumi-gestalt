#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ServicePerimeterSpec {
    /// A list of AccessLevel resource names that allow resources within
    /// the ServicePerimeter to be accessed from the internet.
    /// AccessLevels listed must be in the same policy as this
    /// ServicePerimeter. Referencing a nonexistent AccessLevel is a
    /// syntax error. If no AccessLevel names are listed, resources within
    /// the perimeter can only be accessed via GCP calls with request
    /// origins within the perimeter. For Service Perimeter Bridge, must
    /// be empty.
    /// Format: accessPolicies/{policy_id}/accessLevels/{access_level_name}
    #[builder(into)]
    #[serde(rename = "accessLevels")]
    pub r#access_levels: Option<Vec<String>>,
    /// List of EgressPolicies to apply to the perimeter. A perimeter may
    /// have multiple EgressPolicies, each of which is evaluated separately.
    /// Access is granted if any EgressPolicy grants it. Must be empty for
    /// a perimeter bridge.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "egressPolicies")]
    pub r#egress_policies: Option<Vec<super::super::types::accesscontextmanager::ServicePerimeterSpecEgressPolicy>>,
    /// List of `IngressPolicies` to apply to the perimeter. A perimeter may
    /// have multiple `IngressPolicies`, each of which is evaluated
    /// separately. Access is granted if any `Ingress Policy` grants it.
    /// Must be empty for a perimeter bridge.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "ingressPolicies")]
    pub r#ingress_policies: Option<Vec<super::super::types::accesscontextmanager::ServicePerimeterSpecIngressPolicy>>,
    /// A list of GCP resources that are inside of the service perimeter.
    /// Currently only projects are allowed.
    /// Format: projects/{project_number}
    #[builder(into)]
    #[serde(rename = "resources")]
    pub r#resources: Option<Vec<String>>,
    /// GCP services that are subject to the Service Perimeter
    /// restrictions. Must contain a list of services. For example, if
    /// `storage.googleapis.com` is specified, access to the storage
    /// buckets inside the perimeter must meet the perimeter's access
    /// restrictions.
    #[builder(into)]
    #[serde(rename = "restrictedServices")]
    pub r#restricted_services: Option<Vec<String>>,
    /// Specifies how APIs are allowed to communicate within the Service
    /// Perimeter.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "vpcAccessibleServices")]
    pub r#vpc_accessible_services: Option<Box<super::super::types::accesscontextmanager::ServicePerimeterSpecVpcAccessibleServices>>,
}
