#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct LocalUserPermissionScope {
    /// A `permissions` block as defined below.
    #[builder(into)]
    #[serde(rename = "permissions")]
    pub r#permissions: Box<super::super::types::storage::LocalUserPermissionScopePermissions>,
    /// The container name (when `service` is set to `blob`) or the file share name (when `service` is set to `file`), used by the Storage Account Local User.
    #[builder(into)]
    #[serde(rename = "resourceName")]
    pub r#resource_name: String,
    /// The storage service used by this Storage Account Local User. Possible values are `blob` and `file`.
    #[builder(into)]
    #[serde(rename = "service")]
    pub r#service: String,
}
