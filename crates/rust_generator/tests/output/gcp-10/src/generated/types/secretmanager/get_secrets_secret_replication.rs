#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetSecretsSecretReplication {
    /// The Secret will automatically be replicated without any restrictions.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "autos")]
    pub r#autos: Vec<super::super::types::secretmanager::GetSecretsSecretReplicationAuto>,
    /// The Secret will be replicated to the regions specified by the user.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "userManageds")]
    pub r#user_manageds: Vec<super::super::types::secretmanager::GetSecretsSecretReplicationUserManaged>,
}
