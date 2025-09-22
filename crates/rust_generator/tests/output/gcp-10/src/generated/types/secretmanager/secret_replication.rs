#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct SecretReplication {
    /// The Secret will automatically be replicated without any restrictions.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "auto")]
    pub r#auto: Box<Option<super::super::types::secretmanager::SecretReplicationAuto>>,
    /// The Secret will be replicated to the regions specified by the user.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "userManaged")]
    pub r#user_managed: Box<Option<super::super::types::secretmanager::SecretReplicationUserManaged>>,
}
