/// Resource for managing an AWS DataZone Environment Blueprint Configuration.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:datazone:Domain
///     properties:
///       name: example_domain
///       domainExecutionRole: ${domainExecutionRole.arn}
///   exampleEnvironmentBlueprintConfiguration:
///     type: aws:datazone:EnvironmentBlueprintConfiguration
///     name: example
///     properties:
///       domainId: ${example.id}
///       environmentBlueprintId: ${defaultDataLake.id}
///       enabledRegions:
///         - us-east-1
///       regionalParameters:
///         us-east-1:
///           s3Location: s3://my-amazon-datazone-bucket
/// variables:
///   defaultDataLake:
///     fn::invoke:
///       function: aws:datazone:getEnvironmentBlueprint
///       arguments:
///         domainId: ${example.id}
///         name: DefaultDataLake
///         managed: true
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import DataZone Environment Blueprint Configuration using the `domain_id` and `environment_blueprint_id`, separated by a `/`. For example:
///
/// ```sh
/// $ pulumi import aws:datazone/environmentBlueprintConfiguration:EnvironmentBlueprintConfiguration example domain-id-12345/environment-blueprint-id-54321
/// ```
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod environment_blueprint_configuration {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EnvironmentBlueprintConfigurationArgs {
        /// ID of the Domain.
        #[builder(into)]
        pub domain_id: pulumi_gestalt_rust::Input<String>,
        /// Regions in which the blueprint is enabled
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub enabled_regions: pulumi_gestalt_rust::Input<Vec<String>>,
        /// ID of the Environment Blueprint
        #[builder(into)]
        pub environment_blueprint_id: pulumi_gestalt_rust::Input<String>,
        /// ARN of the manage access role with which this blueprint is created.
        #[builder(into, default)]
        pub manage_access_role_arn: pulumi_gestalt_rust::Input<Option<String>>,
        /// ARN of the provisioning role with which this blueprint is created.
        #[builder(into, default)]
        pub provisioning_role_arn: pulumi_gestalt_rust::Input<Option<String>>,
        /// Parameters for each region in which the blueprint is enabled
        #[builder(into, default)]
        pub regional_parameters: pulumi_gestalt_rust::Input<
            Option<
                std::collections::HashMap<
                    String,
                    std::collections::HashMap<String, String>,
                >,
            >,
        >,
    }
    #[allow(dead_code)]
    pub struct EnvironmentBlueprintConfigurationResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// ID of the Domain.
        pub domain_id: pulumi_gestalt_rust::Output<String>,
        /// Regions in which the blueprint is enabled
        ///
        /// The following arguments are optional:
        pub enabled_regions: pulumi_gestalt_rust::Output<Vec<String>>,
        /// ID of the Environment Blueprint
        pub environment_blueprint_id: pulumi_gestalt_rust::Output<String>,
        /// ARN of the manage access role with which this blueprint is created.
        pub manage_access_role_arn: pulumi_gestalt_rust::Output<Option<String>>,
        /// ARN of the provisioning role with which this blueprint is created.
        pub provisioning_role_arn: pulumi_gestalt_rust::Output<Option<String>>,
        /// Parameters for each region in which the blueprint is enabled
        pub regional_parameters: pulumi_gestalt_rust::Output<
            Option<
                std::collections::HashMap<
                    String,
                    std::collections::HashMap<String, String>,
                >,
            >,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: EnvironmentBlueprintConfigurationArgs,
    ) -> EnvironmentBlueprintConfigurationResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: EnvironmentBlueprintConfigurationArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> EnvironmentBlueprintConfigurationResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: EnvironmentBlueprintConfigurationArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> EnvironmentBlueprintConfigurationResult {
        let domain_id_binding = args.domain_id.get_output(ctx);
        let enabled_regions_binding = args.enabled_regions.get_output(ctx);
        let environment_blueprint_id_binding = args
            .environment_blueprint_id
            .get_output(ctx);
        let manage_access_role_arn_binding = args.manage_access_role_arn.get_output(ctx);
        let provisioning_role_arn_binding = args.provisioning_role_arn.get_output(ctx);
        let regional_parameters_binding = args.regional_parameters.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:datazone/environmentBlueprintConfiguration:EnvironmentBlueprintConfiguration"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "domainId".into(),
                    value: &domain_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enabledRegions".into(),
                    value: &enabled_regions_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "environmentBlueprintId".into(),
                    value: &environment_blueprint_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "manageAccessRoleArn".into(),
                    value: &manage_access_role_arn_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "provisioningRoleArn".into(),
                    value: &provisioning_role_arn_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "regionalParameters".into(),
                    value: &regional_parameters_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        EnvironmentBlueprintConfigurationResult {
            id: o.get_id(),
            urn: o.get_urn(),
            domain_id: o.get_field("domainId"),
            enabled_regions: o.get_field("enabledRegions"),
            environment_blueprint_id: o.get_field("environmentBlueprintId"),
            manage_access_role_arn: o.get_field("manageAccessRoleArn"),
            provisioning_role_arn: o.get_field("provisioningRoleArn"),
            regional_parameters: o.get_field("regionalParameters"),
        }
    }
}
