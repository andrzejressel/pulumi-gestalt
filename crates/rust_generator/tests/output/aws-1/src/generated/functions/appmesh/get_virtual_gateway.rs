#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_virtual_gateway {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetVirtualGatewayArgs {
        /// Name of the service mesh in which the virtual gateway exists.
        #[builder(into)]
        pub mesh_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Name of the virtual gateway.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Map of tags.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetVirtualGatewayResult {
        /// ARN of the virtual gateway.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Creation date of the virtual gateway.
        pub created_date: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Last update date of the virtual gateway.
        pub last_updated_date: pulumi_gestalt_rust::Output<String>,
        pub mesh_name: pulumi_gestalt_rust::Output<String>,
        pub mesh_owner: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Resource owner's AWS account ID.
        pub resource_owner: pulumi_gestalt_rust::Output<String>,
        /// Virtual gateway specification. See the `aws.appmesh.VirtualGateway` resource for details.
        pub specs: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::appmesh::GetVirtualGatewaySpec>,
        >,
        /// Map of tags.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetVirtualGatewayArgs,
    ) -> GetVirtualGatewayResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let mesh_name_binding = args.mesh_name.get_output(context);
        let name_binding = args.name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:appmesh/getVirtualGateway:getVirtualGateway".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "meshName".into(),
                    value: &mesh_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetVirtualGatewayResult {
            arn: o.get_field("arn"),
            created_date: o.get_field("createdDate"),
            id: o.get_field("id"),
            last_updated_date: o.get_field("lastUpdatedDate"),
            mesh_name: o.get_field("meshName"),
            mesh_owner: o.get_field("meshOwner"),
            name: o.get_field("name"),
            resource_owner: o.get_field("resourceOwner"),
            specs: o.get_field("specs"),
            tags: o.get_field("tags"),
        }
    }
}
