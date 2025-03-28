#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_client_config {
    #[allow(dead_code)]
    pub struct GetClientConfigResult {
        /// is set to the Azure Client ID (Application Object ID).
        pub client_id: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// is set to the Azure Object ID.
        pub object_id: pulumi_gestalt_rust::Output<String>,
        /// is set to the Azure Subscription ID.
        pub subscription_id: pulumi_gestalt_rust::Output<String>,
        /// is set to the Azure Tenant ID.
        pub tenant_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(context: &pulumi_gestalt_rust::Context) -> GetClientConfigResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:core/getClientConfig:getClientConfig".into(),
            version: super::super::super::get_version(),
            object: &[],
        };
        let o = context.invoke_resource(request);
        GetClientConfigResult {
            client_id: o.get_field("clientId"),
            id: o.get_field("id"),
            object_id: o.get_field("objectId"),
            subscription_id: o.get_field("subscriptionId"),
            tenant_id: o.get_field("tenantId"),
        }
    }
}
