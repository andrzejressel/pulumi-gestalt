#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct SecretReplication {
    /// The Secret will automatically be replicated without any restrictions.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "auto")]
    pub r#auto: Option<Box<super::super::types::secretmanager::SecretReplicationAuto>>,
    /// The Secret will be replicated to the regions specified by the user.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "userManaged")]
    pub r#user_managed: Option<Box<super::super::types::secretmanager::SecretReplicationUserManaged>>,
}
