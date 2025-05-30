/// Manages an Image Builder Image.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = image::create(
///         "example",
///         ImageArgs::builder()
///             .distribution_configuration_arn(
///                 "${exampleAwsImagebuilderDistributionConfiguration.arn}",
///             )
///             .image_recipe_arn("${exampleAwsImagebuilderImageRecipe.arn}")
///             .infrastructure_configuration_arn(
///                 "${exampleAwsImagebuilderInfrastructureConfiguration.arn}",
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_imagebuilder_image` resources using the Amazon Resource Name (ARN). For example:
///
/// ```sh
/// $ pulumi import aws:imagebuilder/image:Image example arn:aws:imagebuilder:us-east-1:123456789012:image/example/1.0.0/1
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod image {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ImageArgs {
        /// Amazon Resource Name (ARN) of the container recipe.
        #[builder(into, default)]
        pub container_recipe_arn: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Amazon Resource Name (ARN) of the Image Builder Distribution Configuration.
        #[builder(into, default)]
        pub distribution_configuration_arn: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Whether additional information about the image being created is collected. Defaults to `true`.
        #[builder(into, default)]
        pub enhanced_image_metadata_enabled: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// Amazon Resource Name (ARN) of the service-linked role to be used by Image Builder to [execute workflows](https://docs.aws.amazon.com/imagebuilder/latest/userguide/manage-image-workflows.html).
        #[builder(into, default)]
        pub execution_role: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Amazon Resource Name (ARN) of the image recipe.
        #[builder(into, default)]
        pub image_recipe_arn: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Configuration block with image scanning configuration. Detailed below.
        #[builder(into, default)]
        pub image_scanning_configuration: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::imagebuilder::ImageImageScanningConfiguration>,
        >,
        /// Configuration block with image tests configuration. Detailed below.
        #[builder(into, default)]
        pub image_tests_configuration: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::imagebuilder::ImageImageTestsConfiguration>,
        >,
        /// Amazon Resource Name (ARN) of the Image Builder Infrastructure Configuration.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub infrastructure_configuration_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Key-value map of resource tags for the Image Builder Image. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Configuration block with the workflow configuration. Detailed below.
        #[builder(into, default)]
        pub workflows: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::imagebuilder::ImageWorkflow>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ImageResult {
        /// Amazon Resource Name (ARN) of the image.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Amazon Resource Name (ARN) of the container recipe.
        pub container_recipe_arn: pulumi_gestalt_rust::Output<Option<String>>,
        /// Date the image was created.
        pub date_created: pulumi_gestalt_rust::Output<String>,
        /// Amazon Resource Name (ARN) of the Image Builder Distribution Configuration.
        pub distribution_configuration_arn: pulumi_gestalt_rust::Output<Option<String>>,
        /// Whether additional information about the image being created is collected. Defaults to `true`.
        pub enhanced_image_metadata_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Amazon Resource Name (ARN) of the service-linked role to be used by Image Builder to [execute workflows](https://docs.aws.amazon.com/imagebuilder/latest/userguide/manage-image-workflows.html).
        pub execution_role: pulumi_gestalt_rust::Output<String>,
        /// Amazon Resource Name (ARN) of the image recipe.
        pub image_recipe_arn: pulumi_gestalt_rust::Output<Option<String>>,
        /// Configuration block with image scanning configuration. Detailed below.
        pub image_scanning_configuration: pulumi_gestalt_rust::Output<
            super::super::types::imagebuilder::ImageImageScanningConfiguration,
        >,
        /// Configuration block with image tests configuration. Detailed below.
        pub image_tests_configuration: pulumi_gestalt_rust::Output<
            super::super::types::imagebuilder::ImageImageTestsConfiguration,
        >,
        /// Amazon Resource Name (ARN) of the Image Builder Infrastructure Configuration.
        ///
        /// The following arguments are optional:
        pub infrastructure_configuration_arn: pulumi_gestalt_rust::Output<String>,
        /// Name of the AMI.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Operating System version of the image.
        pub os_version: pulumi_gestalt_rust::Output<String>,
        /// List of objects with resources created by the image.
        pub output_resources: pulumi_gestalt_rust::Output<
            Vec<super::super::types::imagebuilder::ImageOutputResource>,
        >,
        /// Platform of the image.
        pub platform: pulumi_gestalt_rust::Output<String>,
        /// Key-value map of resource tags for the Image Builder Image. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Version of the image.
        pub version: pulumi_gestalt_rust::Output<String>,
        /// Configuration block with the workflow configuration. Detailed below.
        pub workflows: pulumi_gestalt_rust::Output<
            Vec<super::super::types::imagebuilder::ImageWorkflow>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ImageArgs,
    ) -> ImageResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let container_recipe_arn_binding = args.container_recipe_arn.get_output(context);
        let distribution_configuration_arn_binding = args
            .distribution_configuration_arn
            .get_output(context);
        let enhanced_image_metadata_enabled_binding = args
            .enhanced_image_metadata_enabled
            .get_output(context);
        let execution_role_binding = args.execution_role.get_output(context);
        let image_recipe_arn_binding = args.image_recipe_arn.get_output(context);
        let image_scanning_configuration_binding = args
            .image_scanning_configuration
            .get_output(context);
        let image_tests_configuration_binding = args
            .image_tests_configuration
            .get_output(context);
        let infrastructure_configuration_arn_binding = args
            .infrastructure_configuration_arn
            .get_output(context);
        let tags_binding = args.tags.get_output(context);
        let workflows_binding = args.workflows.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:imagebuilder/image:Image".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "containerRecipeArn".into(),
                    value: &container_recipe_arn_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "distributionConfigurationArn".into(),
                    value: &distribution_configuration_arn_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enhancedImageMetadataEnabled".into(),
                    value: &enhanced_image_metadata_enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "executionRole".into(),
                    value: &execution_role_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "imageRecipeArn".into(),
                    value: &image_recipe_arn_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "imageScanningConfiguration".into(),
                    value: &image_scanning_configuration_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "imageTestsConfiguration".into(),
                    value: &image_tests_configuration_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "infrastructureConfigurationArn".into(),
                    value: &infrastructure_configuration_arn_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "workflows".into(),
                    value: &workflows_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        ImageResult {
            arn: o.get_field("arn"),
            container_recipe_arn: o.get_field("containerRecipeArn"),
            date_created: o.get_field("dateCreated"),
            distribution_configuration_arn: o.get_field("distributionConfigurationArn"),
            enhanced_image_metadata_enabled: o.get_field("enhancedImageMetadataEnabled"),
            execution_role: o.get_field("executionRole"),
            image_recipe_arn: o.get_field("imageRecipeArn"),
            image_scanning_configuration: o.get_field("imageScanningConfiguration"),
            image_tests_configuration: o.get_field("imageTestsConfiguration"),
            infrastructure_configuration_arn: o
                .get_field("infrastructureConfigurationArn"),
            name: o.get_field("name"),
            os_version: o.get_field("osVersion"),
            output_resources: o.get_field("outputResources"),
            platform: o.get_field("platform"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            version: o.get_field("version"),
            workflows: o.get_field("workflows"),
        }
    }
}
