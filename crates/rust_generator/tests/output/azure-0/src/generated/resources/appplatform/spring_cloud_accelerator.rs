/// > **NOTE:** This resource is applicable only for Spring Cloud Service with enterprise tier.
///
/// Manages a Spring Cloud Accelerator.
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
///             .name("example")
///             .build_struct(),
///     );
///     let exampleSpringCloudAccelerator = spring_cloud_accelerator::create(
///         "exampleSpringCloudAccelerator",
///         SpringCloudAcceleratorArgs::builder()
///             .name("default")
///             .spring_cloud_service_id("${exampleSpringCloudService.id}")
///             .build_struct(),
///     );
///     let exampleSpringCloudService = spring_cloud_service::create(
///         "exampleSpringCloudService",
///         SpringCloudServiceArgs::builder()
///             .location("${example.location}")
///             .name("example")
///             .resource_group_name("${example.name}")
///             .sku_name("E0")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Spring Cloud Accelerators can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:appplatform/springCloudAccelerator:SpringCloudAccelerator example /subscriptions/12345678-1234-9876-4563-123456789012/resourceGroups/resourceGroup1/providers/Microsoft.AppPlatform/spring/service1/applicationAccelerators/default
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod spring_cloud_accelerator {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SpringCloudAcceleratorArgs {
        /// The name which should be used for this Spring Cloud Accelerator. Changing this forces a new Spring Cloud Accelerator to be created. The only possible value is `default`.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the Spring Cloud Service. Changing this forces a new Spring Cloud Accelerator to be created.
        #[builder(into)]
        pub spring_cloud_service_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct SpringCloudAcceleratorResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The name which should be used for this Spring Cloud Accelerator. Changing this forces a new Spring Cloud Accelerator to be created. The only possible value is `default`.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Spring Cloud Service. Changing this forces a new Spring Cloud Accelerator to be created.
        pub spring_cloud_service_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SpringCloudAcceleratorArgs,
    ) -> SpringCloudAcceleratorResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let spring_cloud_service_id_binding = args
            .spring_cloud_service_id
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:appplatform/springCloudAccelerator:SpringCloudAccelerator"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "springCloudServiceId".into(),
                    value: &spring_cloud_service_id_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        SpringCloudAcceleratorResult {
            id: o.get_field("id"),
            name: o.get_field("name"),
            spring_cloud_service_id: o.get_field("springCloudServiceId"),
        }
    }
}
