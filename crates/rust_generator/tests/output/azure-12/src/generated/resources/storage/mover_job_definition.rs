/// Manages a Storage Mover Job Definition.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resource_group::create(
///         "example",
///         ResourceGroupArgs::builder()
///             .location("West Europe")
///             .name("example-resources")
///             .build_struct(),
///     );
///     let exampleAccount = account::create(
///         "exampleAccount",
///         AccountArgs::builder()
///             .account_replication_type("LRS")
///             .account_tier("Standard")
///             .allow_nested_items_to_be_public(true)
///             .location("${example.location}")
///             .name("examplesa")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleContainer = container::create(
///         "exampleContainer",
///         ContainerArgs::builder()
///             .container_access_type("blob")
///             .name("acccontainer")
///             .storage_account_name("${exampleAccount.name}")
///             .build_struct(),
///     );
///     let exampleMover = mover::create(
///         "exampleMover",
///         MoverArgs::builder()
///             .location("${example.location}")
///             .name("example-ssm")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleMoverAgent = mover_agent::create(
///         "exampleMoverAgent",
///         MoverAgentArgs::builder()
///             .arc_virtual_machine_id(
///                 "${example.id}/providers/Microsoft.HybridCompute/machines/examples-hybridComputeName",
///             )
///             .arc_virtual_machine_uuid("3bb2c024-eba9-4d18-9e7a-1d772fcc5fe9")
///             .name("example-agent")
///             .storage_mover_id("${exampleMover.id}")
///             .build_struct(),
///     );
///     let exampleMoverJobDefinition = mover_job_definition::create(
///         "exampleMoverJobDefinition",
///         MoverJobDefinitionArgs::builder()
///             .agent_name("${exampleMoverAgent.name}")
///             .copy_mode("Additive")
///             .description("Example Job Definition Description")
///             .name("example-sjd")
///             .source_name("${exampleMoverSourceEndpoint.name}")
///             .source_sub_path("/")
///             .storage_mover_project_id("${exampleMoverProject.id}")
///             .target_name("${exampleMoverTargetEndpoint.name}")
///             .target_sub_path("/")
///             .build_struct(),
///     );
///     let exampleMoverProject = mover_project::create(
///         "exampleMoverProject",
///         MoverProjectArgs::builder()
///             .name("example-sp")
///             .storage_mover_id("${exampleMover.id}")
///             .build_struct(),
///     );
///     let exampleMoverSourceEndpoint = mover_source_endpoint::create(
///         "exampleMoverSourceEndpoint",
///         MoverSourceEndpointArgs::builder()
///             .host("192.168.0.1")
///             .name("example-smse")
///             .storage_mover_id("${exampleMover.id}")
///             .build_struct(),
///     );
///     let exampleMoverTargetEndpoint = mover_target_endpoint::create(
///         "exampleMoverTargetEndpoint",
///         MoverTargetEndpointArgs::builder()
///             .name("example-smte")
///             .storage_account_id("${exampleAccount.id}")
///             .storage_container_name("${exampleContainer.name}")
///             .storage_mover_id("${exampleMover.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Storage Mover Job Definition can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:storage/moverJobDefinition:MoverJobDefinition example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/resourceGroup1/providers/Microsoft.StorageMover/storageMovers/storageMover1/projects/project1/jobDefinitions/jobDefinition1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod mover_job_definition {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct MoverJobDefinitionArgs {
        /// Specifies the name of the Storage Mover Agent to assign for new Job Runs of this Storage Mover Job Definition.
        #[builder(into, default)]
        pub agent_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the strategy to use for copy. Possible values are `Additive` and `Mirror`.
        #[builder(into)]
        pub copy_mode: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies a description for this Storage Mover Job Definition.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the name which should be used for this Storage Mover Job Definition. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the name of the Storage Mover Source Endpoint. Changing this forces a new resource to be created.
        #[builder(into)]
        pub source_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the sub path to use when reading from the Storage Mover Source Endpoint. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub source_sub_path: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the ID of the Storage Mover Project. Changing this forces a new resource to be created.
        #[builder(into)]
        pub storage_mover_project_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the name of the Storage Mover target Endpoint. Changing this forces a new resource to be created.
        #[builder(into)]
        pub target_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the sub path to use when writing to the Storage Mover Target Endpoint. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub target_sub_path: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct MoverJobDefinitionResult {
        /// Specifies the name of the Storage Mover Agent to assign for new Job Runs of this Storage Mover Job Definition.
        pub agent_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the strategy to use for copy. Possible values are `Additive` and `Mirror`.
        pub copy_mode: pulumi_gestalt_rust::Output<String>,
        /// Specifies a description for this Storage Mover Job Definition.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the name which should be used for this Storage Mover Job Definition. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name of the Storage Mover Source Endpoint. Changing this forces a new resource to be created.
        pub source_name: pulumi_gestalt_rust::Output<String>,
        /// Specifies the sub path to use when reading from the Storage Mover Source Endpoint. Changing this forces a new resource to be created.
        pub source_sub_path: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the ID of the Storage Mover Project. Changing this forces a new resource to be created.
        pub storage_mover_project_id: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name of the Storage Mover target Endpoint. Changing this forces a new resource to be created.
        pub target_name: pulumi_gestalt_rust::Output<String>,
        /// Specifies the sub path to use when writing to the Storage Mover Target Endpoint. Changing this forces a new resource to be created.
        pub target_sub_path: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: MoverJobDefinitionArgs,
    ) -> MoverJobDefinitionResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let agent_name_binding = args.agent_name.get_output(context);
        let copy_mode_binding = args.copy_mode.get_output(context);
        let description_binding = args.description.get_output(context);
        let name_binding = args.name.get_output(context);
        let source_name_binding = args.source_name.get_output(context);
        let source_sub_path_binding = args.source_sub_path.get_output(context);
        let storage_mover_project_id_binding = args
            .storage_mover_project_id
            .get_output(context);
        let target_name_binding = args.target_name.get_output(context);
        let target_sub_path_binding = args.target_sub_path.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:storage/moverJobDefinition:MoverJobDefinition".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "agentName".into(),
                    value: &agent_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "copyMode".into(),
                    value: &copy_mode_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sourceName".into(),
                    value: &source_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sourceSubPath".into(),
                    value: &source_sub_path_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "storageMoverProjectId".into(),
                    value: &storage_mover_project_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "targetName".into(),
                    value: &target_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "targetSubPath".into(),
                    value: &target_sub_path_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        MoverJobDefinitionResult {
            agent_name: o.get_field("agentName"),
            copy_mode: o.get_field("copyMode"),
            description: o.get_field("description"),
            name: o.get_field("name"),
            source_name: o.get_field("sourceName"),
            source_sub_path: o.get_field("sourceSubPath"),
            storage_mover_project_id: o.get_field("storageMoverProjectId"),
            target_name: o.get_field("targetName"),
            target_sub_path: o.get_field("targetSubPath"),
        }
    }
}
