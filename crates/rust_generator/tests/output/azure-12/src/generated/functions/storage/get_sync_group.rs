#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_sync_group {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetSyncGroupArgs {
        /// The name of this Storage Sync Group.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The resource ID of the Storage Sync where this Storage Sync Group is.
        #[builder(into)]
        pub storage_sync_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetSyncGroupResult {
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        pub storage_sync_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetSyncGroupArgs,
    ) -> GetSyncGroupResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let storage_sync_id_binding = args.storage_sync_id.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:storage/getSyncGroup:getSyncGroup".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "storageSyncId".into(),
                    value: &storage_sync_id_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetSyncGroupResult {
            id: o.get_field("id"),
            name: o.get_field("name"),
            storage_sync_id: o.get_field("storageSyncId"),
        }
    }
}
