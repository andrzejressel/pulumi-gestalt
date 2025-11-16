#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ServicePerimeterSpecIngressPolicyIngressFrom {
    /// A list of identities that are allowed access through this ingress policy.
    /// Should be in the format of email address. The email address should represent
    /// individual user or service account only.
    #[builder(into)]
    #[serde(rename = "identities")]
    pub r#identities: Option<Vec<String>>,
    /// Specifies the type of identities that are allowed access from outside the
    /// perimeter. If left unspecified, then members of `identities` field will be
    /// allowed access.
    /// Possible values are: `IDENTITY_TYPE_UNSPECIFIED`, `ANY_IDENTITY`, `ANY_USER_ACCOUNT`, `ANY_SERVICE_ACCOUNT`.
    #[builder(into)]
    #[serde(rename = "identityType")]
    pub r#identity_type: Option<String>,
    /// Sources that this `IngressPolicy` authorizes access from.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "sources")]
    pub r#sources: Option<Vec<super::super::types::accesscontextmanager::ServicePerimeterSpecIngressPolicyIngressFromSource>>,
}
