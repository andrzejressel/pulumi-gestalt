#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ServicePerimeterEgressPolicyEgressFrom {
    /// Identities can be an individual user, service account, Google group,
    /// or third-party identity. For third-party identity, only single identities
    /// are supported and other identity types are not supported.The v1 identities
    /// that have the prefix user, group and serviceAccount in
    /// https://cloud.google.com/iam/docs/principal-identifiers#v1 are supported.
    #[builder(into)]
    #[serde(rename = "identities")]
    pub r#identities: Option<Vec<String>>,
    /// Specifies the type of identities that are allowed access to outside the
    /// perimeter. If left unspecified, then members of `identities` field will
    /// be allowed access.
    /// Possible values are: `ANY_IDENTITY`, `ANY_USER_ACCOUNT`, `ANY_SERVICE_ACCOUNT`.
    #[builder(into)]
    #[serde(rename = "identityType")]
    pub r#identity_type: Option<String>,
    /// Whether to enforce traffic restrictions based on `sources` field. If the `sources` field is non-empty, then this field must be set to `SOURCE_RESTRICTION_ENABLED`.
    /// Possible values are: `SOURCE_RESTRICTION_UNSPECIFIED`, `SOURCE_RESTRICTION_ENABLED`, `SOURCE_RESTRICTION_DISABLED`.
    #[builder(into)]
    #[serde(rename = "sourceRestriction")]
    pub r#source_restriction: Option<String>,
    /// Sources that this EgressPolicy authorizes access from.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "sources")]
    pub r#sources: Option<Vec<super::super::types::accesscontextmanager::ServicePerimeterEgressPolicyEgressFromSource>>,
}
