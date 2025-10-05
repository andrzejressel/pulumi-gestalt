/// Provides an AWS App Mesh service mesh resource.
///
/// ## Example Usage
///
/// ### Basic
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let simple = mesh::create(
///         "simple",
///         MeshArgs::builder().name("simpleapp").build_struct(),
///     );
/// }
/// ```
///
/// ### Egress Filter
///
/// ```yaml
/// resources:
///   simple:
///     type: aws:appmesh:Mesh
///     properties:
///       name: simpleapp
///       spec:
///         egressFilter:
///           type: ALLOW_ALL
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import App Mesh service meshes using the `name`. For example:
///
/// ```sh
/// $ pulumi import aws:appmesh/mesh:Mesh simple simpleapp
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod mesh {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct MeshArgs {
        /// Name to use for the service mesh. Must be between 1 and 255 characters in length.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Service mesh specification to apply.
        #[builder(into, default)]
        pub spec: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::appmesh::MeshSpec>,
        >,
        /// Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct MeshResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// ARN of the service mesh.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Creation date of the service mesh.
        pub created_date: pulumi_gestalt_rust::Output<String>,
        /// Last update date of the service mesh.
        pub last_updated_date: pulumi_gestalt_rust::Output<String>,
        /// AWS account ID of the service mesh's owner.
        pub mesh_owner: pulumi_gestalt_rust::Output<String>,
        /// Name to use for the service mesh. Must be between 1 and 255 characters in length.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Resource owner's AWS account ID.
        pub resource_owner: pulumi_gestalt_rust::Output<String>,
        /// Service mesh specification to apply.
        pub spec: pulumi_gestalt_rust::Output<
            Option<super::super::types::appmesh::MeshSpec>,
        >,
        /// Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: MeshArgs,
    ) -> MeshResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let spec_binding = args.spec.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:appmesh/mesh:Mesh".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "spec".into(),
                    value: &spec_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        MeshResult {
            id: o.get_field("id"),
            arn: o.get_field("arn"),
            created_date: o.get_field("createdDate"),
            last_updated_date: o.get_field("lastUpdatedDate"),
            mesh_owner: o.get_field("meshOwner"),
            name: o.get_field("name"),
            resource_owner: o.get_field("resourceOwner"),
            spec: o.get_field("spec"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}
