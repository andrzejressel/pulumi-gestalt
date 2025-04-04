#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_folders {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetFoldersArgs {
        /// A string parent as defined in the [REST API](https://cloud.google.com/resource-manager/reference/rest/v3/folders/list#query-parameters).
        #[builder(into)]
        pub parent_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetFoldersResult {
        /// A list of folders matching the provided filter. Structure is defined below.
        pub folders: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::organizations::GetFoldersFolder>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub parent_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetFoldersArgs,
    ) -> GetFoldersResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let parent_id_binding = args.parent_id.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "gcp:organizations/getFolders:getFolders".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "parentId".into(),
                    value: &parent_id_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetFoldersResult {
            folders: o.get_field("folders"),
            id: o.get_field("id"),
            parent_id: o.get_field("parentId"),
        }
    }
}
