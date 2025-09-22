#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct AddressMapMembership {
    /// Controls whether the membership can be deleted via the API or not.
    #[builder(into)]
    #[serde(rename = "canDelete")]
    pub r#can_delete: Option<bool>,
    /// Identifier of the account or zone.
    #[builder(into)]
    #[serde(rename = "identifier")]
    pub r#identifier: String,
    /// The type of the membership.
    #[builder(into)]
    #[serde(rename = "kind")]
    pub r#kind: String,
}
