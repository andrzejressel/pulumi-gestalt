/// Manages an Active Azure Spring Cloud Deployment.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-resources
///       location: West Europe
///   exampleSpringCloudService:
///     type: azure:appplatform:SpringCloudService
///     name: example
///     properties:
///       name: example-springcloud
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///   exampleSpringCloudApp:
///     type: azure:appplatform:SpringCloudApp
///     name: example
///     properties:
///       name: example-springcloudapp
///       resourceGroupName: ${example.name}
///       serviceName: ${exampleSpringCloudService.name}
///       identity:
///         type: SystemAssigned
///   exampleSpringCloudJavaDeployment:
///     type: azure:appplatform:SpringCloudJavaDeployment
///     name: example
///     properties:
///       name: deploy1
///       springCloudAppId: ${exampleSpringCloudApp.id}
///       instanceCount: 2
///       jvmOptions: -XX:+PrintGC
///       runtimeVersion: Java_11
///       quota:
///         cpu: '2'
///         memory: 4Gi
///       environmentVariables:
///         Env: Staging
///   exampleSpringCloudActiveDeployment:
///     type: azure:appplatform:SpringCloudActiveDeployment
///     name: example
///     properties:
///       springCloudAppId: ${exampleSpringCloudApp.id}
///       deploymentName: ${exampleSpringCloudJavaDeployment.name}
/// ```
///
/// ## Import
///
/// Spring Cloud Active Deployment can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:appplatform/springCloudActiveDeployment:SpringCloudActiveDeployment example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/resourcegroup1/providers/Microsoft.AppPlatform/spring/service1/apps/app1
/// ```
///
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod spring_cloud_active_deployment {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SpringCloudActiveDeploymentArgs {
        /// Specifies the name of Spring Cloud Deployment which is going to be active.
        #[builder(into)]
        pub deployment_name: pulumi_gestalt_rust::Input<String>,
        /// Specifies the id of the Spring Cloud Application. Changing this forces a new resource to be created.
        #[builder(into)]
        pub spring_cloud_app_id: pulumi_gestalt_rust::Input<String>,
    }
    #[allow(dead_code)]
    pub struct SpringCloudActiveDeploymentResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name of Spring Cloud Deployment which is going to be active.
        pub deployment_name: pulumi_gestalt_rust::Output<String>,
        /// Specifies the id of the Spring Cloud Application. Changing this forces a new resource to be created.
        pub spring_cloud_app_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SpringCloudActiveDeploymentArgs,
    ) -> SpringCloudActiveDeploymentResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SpringCloudActiveDeploymentArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> SpringCloudActiveDeploymentResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SpringCloudActiveDeploymentArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> SpringCloudActiveDeploymentResult {
        let deployment_name_binding = args.deployment_name.get_output(ctx);
        let spring_cloud_app_id_binding = args.spring_cloud_app_id.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:appplatform/springCloudActiveDeployment:SpringCloudActiveDeployment"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "deploymentName".into(),
                    value: &deployment_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "springCloudAppId".into(),
                    value: &spring_cloud_app_id_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        SpringCloudActiveDeploymentResult {
            id: o.get_id(),
            urn: o.get_urn(),
            deployment_name: o.get_field("deploymentName"),
            spring_cloud_app_id: o.get_field("springCloudAppId"),
        }
    }
}
