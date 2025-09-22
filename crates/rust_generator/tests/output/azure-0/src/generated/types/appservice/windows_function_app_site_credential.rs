#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct WindowsFunctionAppSiteCredential {
    /// The name which should be used for this Windows Function App. Changing this forces a new Windows Function App to be created. Limit the function name to 32 characters to avoid naming collisions. For more information about [Function App naming rule](https://docs.microsoft.com/en-us/azure/azure-resource-manager/management/resource-name-rules#microsoftweb) and [Host ID Collisions](https://github.com/Azure/azure-functions-host/wiki/Host-IDs#host-id-collisions)
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// The Site Credentials Password used for publishing.
    #[builder(into)]
    #[serde(rename = "password")]
    pub r#password: Option<String>,
}
