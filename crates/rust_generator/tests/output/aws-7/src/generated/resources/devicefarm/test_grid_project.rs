/// Provides a resource to manage AWS Device Farm Test Grid Projects.
///
/// > **NOTE:** AWS currently has limited regional support for Device Farm (e.g., `us-west-2`). See [AWS Device Farm endpoints and quotas](https://docs.aws.amazon.com/general/latest/gr/devicefarm.html) for information on supported regions.
///
/// ## Example Usage
///
///
/// ## Import
///
/// Using `pulumi import`, import DeviceFarm Test Grid Projects using their ARN. For example:
///
/// ```sh
/// $ pulumi import aws:devicefarm/testGridProject:TestGridProject example arn:aws:devicefarm:us-west-2:123456789012:testgrid-project:4fa784c7-ccb4-4dbf-ba4f-02198320daa1
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod test_grid_project {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TestGridProjectArgs {
        /// Human-readable description of the project.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the Selenium testing project.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The VPC security groups and subnets that are attached to a project. See VPC Config below.
        #[builder(into, default)]
        pub vpc_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::devicefarm::TestGridProjectVpcConfig>,
        >,
    }
    #[allow(dead_code)]
    pub struct TestGridProjectResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The Amazon Resource Name of this Test Grid Project.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Human-readable description of the project.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the Selenium testing project.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The VPC security groups and subnets that are attached to a project. See VPC Config below.
        pub vpc_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::devicefarm::TestGridProjectVpcConfig>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: TestGridProjectArgs,
    ) -> TestGridProjectResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let name_binding = args.name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let vpc_config_binding = args.vpc_config.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:devicefarm/testGridProject:TestGridProject".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "vpcConfig".into(),
                    value: &vpc_config_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        TestGridProjectResult {
            id: o.get_field("id"),
            arn: o.get_field("arn"),
            description: o.get_field("description"),
            name: o.get_field("name"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            vpc_config: o.get_field("vpcConfig"),
        }
    }
}
