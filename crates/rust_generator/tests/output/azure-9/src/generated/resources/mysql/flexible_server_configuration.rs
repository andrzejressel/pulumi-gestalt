/// Sets a MySQL Flexible Server Configuration value on a MySQL Flexible Server.
///
/// ## Disclaimers
///
/// > **Note:** Since this resource is provisioned by default, the Azure Provider will not check for the presence of an existing resource prior to attempting to create it.
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
///     let exampleFlexibleServer = flexible_server::create(
///         "exampleFlexibleServer",
///         FlexibleServerArgs::builder()
///             .administrator_login("adminTerraform")
///             .administrator_password("H@Sh1CoR3!")
///             .location("${example.location}")
///             .name("example-fs")
///             .resource_group_name("${example.name}")
///             .sku_name("GP_Standard_D2ds_v4")
///             .build_struct(),
///     );
///     let exampleFlexibleServerConfiguration = flexible_server_configuration::create(
///         "exampleFlexibleServerConfiguration",
///         FlexibleServerConfigurationArgs::builder()
///             .name("interactive_timeout")
///             .resource_group_name("${example.name}")
///             .server_name("${exampleFlexibleServer.name}")
///             .value("600")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// MySQL Flexible Server Configurations can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:mysql/flexibleServerConfiguration:FlexibleServerConfiguration interactive_timeout /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.DBforMySQL/flexibleServers/flexibleServer1/configurations/interactive_timeout
/// ```
///
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod flexible_server_configuration {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FlexibleServerConfigurationArgs {
        /// Specifies the name of the MySQL Flexible Server Configuration, which needs [to be a valid MySQL configuration name](https://dev.mysql.com/doc/refman/5.7/en/server-configuration.html). Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::Input<Option<String>>,
        /// The name of the resource group in which the MySQL Flexible Server exists. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::Input<String>,
        /// Specifies the name of the MySQL Flexible Server. Changing this forces a new resource to be created.
        #[builder(into)]
        pub server_name: pulumi_gestalt_rust::Input<String>,
        /// Specifies the value of the MySQL Flexible Server Configuration. See the MySQL documentation for valid values.
        #[builder(into)]
        pub value: pulumi_gestalt_rust::Input<String>,
    }
    #[allow(dead_code)]
    pub struct FlexibleServerConfigurationResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name of the MySQL Flexible Server Configuration, which needs [to be a valid MySQL configuration name](https://dev.mysql.com/doc/refman/5.7/en/server-configuration.html). Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The name of the resource group in which the MySQL Flexible Server exists. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name of the MySQL Flexible Server. Changing this forces a new resource to be created.
        pub server_name: pulumi_gestalt_rust::Output<String>,
        /// Specifies the value of the MySQL Flexible Server Configuration. See the MySQL documentation for valid values.
        pub value: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: FlexibleServerConfigurationArgs,
    ) -> FlexibleServerConfigurationResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: FlexibleServerConfigurationArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> FlexibleServerConfigurationResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: FlexibleServerConfigurationArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> FlexibleServerConfigurationResult {
        let name_binding = args.name.get_output(ctx);
        let resource_group_name_binding = args.resource_group_name.get_output(ctx);
        let server_name_binding = args.server_name.get_output(ctx);
        let value_binding = args.value.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:mysql/flexibleServerConfiguration:FlexibleServerConfiguration"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "serverName".into(),
                    value: &server_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "value".into(),
                    value: &value_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        FlexibleServerConfigurationResult {
            id: o.get_id(),
            urn: o.get_urn(),
            name: o.get_field("name"),
            resource_group_name: o.get_field("resourceGroupName"),
            server_name: o.get_field("serverName"),
            value: o.get_field("value"),
        }
    }
}
