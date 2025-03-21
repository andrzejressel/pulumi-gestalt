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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod spring_cloud_active_deployment {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SpringCloudActiveDeploymentArgs {
        /// Specifies the name of Spring Cloud Deployment which is going to be active.
        #[builder(into)]
        pub deployment_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the id of the Spring Cloud Application. Changing this forces a new resource to be created.
        #[builder(into)]
        pub spring_cloud_app_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct SpringCloudActiveDeploymentResult {
        /// Specifies the name of Spring Cloud Deployment which is going to be active.
        pub deployment_name: pulumi_gestalt_rust::Output<String>,
        /// Specifies the id of the Spring Cloud Application. Changing this forces a new resource to be created.
        pub spring_cloud_app_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SpringCloudActiveDeploymentArgs,
    ) -> SpringCloudActiveDeploymentResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let deployment_name_binding = args.deployment_name.get_output(context);
        let spring_cloud_app_id_binding = args.spring_cloud_app_id.get_output(context);
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
        };
        let o = context.register_resource(request);
        SpringCloudActiveDeploymentResult {
            deployment_name: o.get_field("deploymentName"),
            spring_cloud_app_id: o.get_field("springCloudAppId"),
        }
    }
}
