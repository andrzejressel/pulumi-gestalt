/// Manages a Spring Cloud Storage.
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
///             .account_replication_type("GRS")
///             .account_tier("Standard")
///             .location("${example.location}")
///             .name("example")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleSpringCloudService = spring_cloud_service::create(
///         "exampleSpringCloudService",
///         SpringCloudServiceArgs::builder()
///             .location("${example.location}")
///             .name("example")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleSpringCloudStorage = spring_cloud_storage::create(
///         "exampleSpringCloudStorage",
///         SpringCloudStorageArgs::builder()
///             .name("example")
///             .spring_cloud_service_id("${exampleSpringCloudService.id}")
///             .storage_account_key("${exampleAccount.primaryAccessKey}")
///             .storage_account_name("${exampleAccount.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Spring Cloud Storages can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:appplatform/springCloudStorage:SpringCloudStorage example /subscriptions/12345678-1234-9876-4563-123456789012/resourceGroups/resourceGroup1/providers/Microsoft.AppPlatform/spring/service1/storages/storage1
/// ```
///
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod spring_cloud_storage {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SpringCloudStorageArgs {
        /// The name which should be used for this Spring Cloud Storage. Changing this forces a new Spring Cloud Storage to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::Input<Option<String>>,
        /// The ID of the Spring Cloud Service where the Spring Cloud Storage should exist. Changing this forces a new Spring Cloud Storage to be created.
        #[builder(into)]
        pub spring_cloud_service_id: pulumi_gestalt_rust::Input<String>,
        /// The access key of the Azure Storage Account.
        #[builder(into)]
        pub storage_account_key: pulumi_gestalt_rust::Input<String>,
        /// The account name of the Azure Storage Account.
        #[builder(into)]
        pub storage_account_name: pulumi_gestalt_rust::Input<String>,
    }
    #[allow(dead_code)]
    pub struct SpringCloudStorageResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// The name which should be used for this Spring Cloud Storage. Changing this forces a new Spring Cloud Storage to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Spring Cloud Service where the Spring Cloud Storage should exist. Changing this forces a new Spring Cloud Storage to be created.
        pub spring_cloud_service_id: pulumi_gestalt_rust::Output<String>,
        /// The access key of the Azure Storage Account.
        pub storage_account_key: pulumi_gestalt_rust::Output<String>,
        /// The account name of the Azure Storage Account.
        pub storage_account_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SpringCloudStorageArgs,
    ) -> SpringCloudStorageResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SpringCloudStorageArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> SpringCloudStorageResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SpringCloudStorageArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> SpringCloudStorageResult {
        let name_binding = args.name.get_output(ctx);
        let spring_cloud_service_id_binding = args
            .spring_cloud_service_id
            .get_output(ctx);
        let storage_account_key_binding = args.storage_account_key.get_output(ctx);
        let storage_account_name_binding = args.storage_account_name.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:appplatform/springCloudStorage:SpringCloudStorage".into(),
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
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "storageAccountKey".into(),
                    value: &storage_account_key_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "storageAccountName".into(),
                    value: &storage_account_name_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        SpringCloudStorageResult {
            id: o.get_id(),
            urn: o.get_urn(),
            name: o.get_field("name"),
            spring_cloud_service_id: o.get_field("springCloudServiceId"),
            storage_account_key: o.get_field("storageAccountKey"),
            storage_account_name: o.get_field("storageAccountName"),
        }
    }
}
