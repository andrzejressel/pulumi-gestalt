/// Provides a CodeDeploy deployment config for an application
///
/// ## Example Usage
///
/// ### Server Usage
///
/// ```yaml
/// resources:
///   foo:
///     type: aws:codedeploy:DeploymentConfig
///     properties:
///       deploymentConfigName: test-deployment-config
///       minimumHealthyHosts:
///         type: HOST_COUNT
///         value: 2
///   fooDeploymentGroup:
///     type: aws:codedeploy:DeploymentGroup
///     name: foo
///     properties:
///       appName: ${fooApp.name}
///       deploymentGroupName: bar
///       serviceRoleArn: ${fooRole.arn}
///       deploymentConfigName: ${foo.id}
///       ec2TagFilters:
///         - key: filterkey
///           type: KEY_AND_VALUE
///           value: filtervalue
///       triggerConfigurations:
///         - triggerEvents:
///             - DeploymentFailure
///           triggerName: foo-trigger
///           triggerTargetArn: foo-topic-arn
///       autoRollbackConfiguration:
///         enabled: true
///         events:
///           - DEPLOYMENT_FAILURE
///       alarmConfiguration:
///         alarms:
///           - my-alarm-name
///         enabled: true
/// ```
///
/// ### Lambda Usage
///
/// ```yaml
/// resources:
///   foo:
///     type: aws:codedeploy:DeploymentConfig
///     properties:
///       deploymentConfigName: test-deployment-config
///       computePlatform: Lambda
///       trafficRoutingConfig:
///         type: TimeBasedLinear
///         timeBasedLinear:
///           interval: 10
///           percentage: 10
///   fooDeploymentGroup:
///     type: aws:codedeploy:DeploymentGroup
///     name: foo
///     properties:
///       appName: ${fooApp.name}
///       deploymentGroupName: bar
///       serviceRoleArn: ${fooRole.arn}
///       deploymentConfigName: ${foo.id}
///       autoRollbackConfiguration:
///         enabled: true
///         events:
///           - DEPLOYMENT_STOP_ON_ALARM
///       alarmConfiguration:
///         alarms:
///           - my-alarm-name
///         enabled: true
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import CodeDeploy Deployment Configurations using the `deployment_config_name`. For example:
///
/// ```sh
/// $ pulumi import aws:codedeploy/deploymentConfig:DeploymentConfig example my-deployment-config
/// ```
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod deployment_config {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DeploymentConfigArgs {
        /// The compute platform can be `Server`, `Lambda`, or `ECS`. Default is `Server`.
        #[builder(into, default)]
        pub compute_platform: pulumi_gestalt_rust::Input<Option<String>>,
        /// The name of the deployment config.
        #[builder(into, default)]
        pub deployment_config_name: pulumi_gestalt_rust::Input<Option<String>>,
        /// A minimum_healthy_hosts block. Required for `Server` compute platform. Minimum Healthy Hosts are documented below.
        #[builder(into, default)]
        pub minimum_healthy_hosts: pulumi_gestalt_rust::Input<
            Option<super::super::types::codedeploy::DeploymentConfigMinimumHealthyHosts>,
        >,
        /// A traffic_routing_config block. Traffic Routing Config is documented below.
        #[builder(into, default)]
        pub traffic_routing_config: pulumi_gestalt_rust::Input<
            Option<super::super::types::codedeploy::DeploymentConfigTrafficRoutingConfig>,
        >,
        /// A zonal_config block. Zonal Config is documented below.
        #[builder(into, default)]
        pub zonal_config: pulumi_gestalt_rust::Input<
            Option<super::super::types::codedeploy::DeploymentConfigZonalConfig>,
        >,
    }
    #[allow(dead_code)]
    pub struct DeploymentConfigResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// The ARN of the deployment config.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The compute platform can be `Server`, `Lambda`, or `ECS`. Default is `Server`.
        pub compute_platform: pulumi_gestalt_rust::Output<Option<String>>,
        /// The AWS Assigned deployment config id
        pub deployment_config_id: pulumi_gestalt_rust::Output<String>,
        /// The name of the deployment config.
        pub deployment_config_name: pulumi_gestalt_rust::Output<String>,
        /// A minimum_healthy_hosts block. Required for `Server` compute platform. Minimum Healthy Hosts are documented below.
        pub minimum_healthy_hosts: pulumi_gestalt_rust::Output<
            Option<super::super::types::codedeploy::DeploymentConfigMinimumHealthyHosts>,
        >,
        /// A traffic_routing_config block. Traffic Routing Config is documented below.
        pub traffic_routing_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::codedeploy::DeploymentConfigTrafficRoutingConfig>,
        >,
        /// A zonal_config block. Zonal Config is documented below.
        pub zonal_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::codedeploy::DeploymentConfigZonalConfig>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: DeploymentConfigArgs,
    ) -> DeploymentConfigResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: DeploymentConfigArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> DeploymentConfigResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: DeploymentConfigArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> DeploymentConfigResult {
        let compute_platform_binding = args.compute_platform.get_output(ctx);
        let deployment_config_name_binding = args.deployment_config_name.get_output(ctx);
        let minimum_healthy_hosts_binding = args.minimum_healthy_hosts.get_output(ctx);
        let traffic_routing_config_binding = args.traffic_routing_config.get_output(ctx);
        let zonal_config_binding = args.zonal_config.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:codedeploy/deploymentConfig:DeploymentConfig".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "computePlatform".into(),
                    value: &compute_platform_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "deploymentConfigName".into(),
                    value: &deployment_config_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "minimumHealthyHosts".into(),
                    value: &minimum_healthy_hosts_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "trafficRoutingConfig".into(),
                    value: &traffic_routing_config_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "zonalConfig".into(),
                    value: &zonal_config_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        DeploymentConfigResult {
            id: o.get_id(),
            urn: o.get_urn(),
            arn: o.get_field("arn"),
            compute_platform: o.get_field("computePlatform"),
            deployment_config_id: o.get_field("deploymentConfigId"),
            deployment_config_name: o.get_field("deploymentConfigName"),
            minimum_healthy_hosts: o.get_field("minimumHealthyHosts"),
            traffic_routing_config: o.get_field("trafficRoutingConfig"),
            zonal_config: o.get_field("zonalConfig"),
        }
    }
}
