/// Manages a Function App Hybrid Connection.
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
///             .name("example-rg")
///             .build_struct(),
///     );
///     let exampleAccount = account::create(
///         "exampleAccount",
///         AccountArgs::builder()
///             .account_replication_type("GRS")
///             .account_tier("Standard")
///             .location("${example.location}")
///             .name("storageaccountname")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleFunctionAppHybridConnection = function_app_hybrid_connection::create(
///         "exampleFunctionAppHybridConnection",
///         FunctionAppHybridConnectionArgs::builder()
///             .function_app_id("${exampleWindowsWebApp.id}")
///             .hostname("myhostname.example")
///             .port(8081)
///             .relay_id("${exampleHybridConnection.id}")
///             .build_struct(),
///     );
///     let exampleHybridConnection = hybrid_connection::create(
///         "exampleHybridConnection",
///         HybridConnectionArgs::builder()
///             .name("examplerhc1")
///             .relay_namespace_name("${exampleNamespace.name}")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleNamespace = namespace::create(
///         "exampleNamespace",
///         NamespaceArgs::builder()
///             .location("${example.location}")
///             .name("example-relay")
///             .resource_group_name("${example.name}")
///             .sku_name("Standard")
///             .build_struct(),
///     );
///     let exampleServicePlan = service_plan::create(
///         "exampleServicePlan",
///         ServicePlanArgs::builder()
///             .location("${example.location}")
///             .name("example-plan")
///             .os_type("Windows")
///             .resource_group_name("${example.name}")
///             .sku_name("S1")
///             .build_struct(),
///     );
///     let exampleWindowsFunctionApp = windows_function_app::create(
///         "exampleWindowsFunctionApp",
///         WindowsFunctionAppArgs::builder()
///             .location("${example.location}")
///             .name("example-function-app")
///             .resource_group_name("${example.name}")
///             .service_plan_id("${exampleServicePlan.id}")
///             .site_config(WindowsFunctionAppSiteConfig::builder().build_struct())
///             .storage_account_access_key("${exampleAccount.primaryAccessKey}")
///             .storage_account_name("${exampleAccount.name}")
///             .build_struct(),
///     );
///     let exampleWindowsWebApp = windows_web_app::create(
///         "exampleWindowsWebApp",
///         WindowsWebAppArgs::builder()
///             .location("${example.location}")
///             .name("example")
///             .resource_group_name("${example.name}")
///             .service_plan_id("${exampleServicePlan.id}")
///             .site_config(WindowsWebAppSiteConfig::builder().build_struct())
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// a Function App Hybrid Connection can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:appservice/functionAppHybridConnection:FunctionAppHybridConnection example "/subscriptions/12345678-1234-9876-4563-123456789012/resourceGroups/resGroup1/providers/Microsoft.Web/sites/site1/hybridConnectionNamespaces/hybridConnectionNamespace1/relays/relay1"
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod function_app_hybrid_connection {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FunctionAppHybridConnectionArgs {
        /// The ID of the Function App for this Hybrid Connection. Changing this forces a new resource to be created.
        #[builder(into)]
        pub function_app_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The hostname of the endpoint.
        #[builder(into)]
        pub hostname: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The port to use for the endpoint
        #[builder(into)]
        pub port: pulumi_gestalt_rust::InputOrOutput<i32>,
        /// The ID of the Relay Hybrid Connection to use. Changing this forces a new resource to be created.
        #[builder(into)]
        pub relay_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Relay key with `Send` permission to use. Defaults to `RootManageSharedAccessKey`
        #[builder(into, default)]
        pub send_key_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct FunctionAppHybridConnectionResult {
        /// The ID of the Function App for this Hybrid Connection. Changing this forces a new resource to be created.
        pub function_app_id: pulumi_gestalt_rust::Output<String>,
        /// The hostname of the endpoint.
        pub hostname: pulumi_gestalt_rust::Output<String>,
        /// The name of the Relay Namespace.
        pub namespace_name: pulumi_gestalt_rust::Output<String>,
        /// The port to use for the endpoint
        pub port: pulumi_gestalt_rust::Output<i32>,
        /// The ID of the Relay Hybrid Connection to use. Changing this forces a new resource to be created.
        pub relay_id: pulumi_gestalt_rust::Output<String>,
        /// The name of the Relay in use.
        pub relay_name: pulumi_gestalt_rust::Output<String>,
        /// The name of the Relay key with `Send` permission to use. Defaults to `RootManageSharedAccessKey`
        pub send_key_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// The Primary Access Key for the `send_key_name`
        pub send_key_value: pulumi_gestalt_rust::Output<String>,
        /// The Service Bus Namespace.
        pub service_bus_namespace: pulumi_gestalt_rust::Output<String>,
        /// The suffix for the endpoint.
        pub service_bus_suffix: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: FunctionAppHybridConnectionArgs,
    ) -> FunctionAppHybridConnectionResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let function_app_id_binding = args.function_app_id.get_output(context);
        let hostname_binding = args.hostname.get_output(context);
        let port_binding = args.port.get_output(context);
        let relay_id_binding = args.relay_id.get_output(context);
        let send_key_name_binding = args.send_key_name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:appservice/functionAppHybridConnection:FunctionAppHybridConnection"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "functionAppId".into(),
                    value: &function_app_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "hostname".into(),
                    value: &hostname_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "port".into(),
                    value: &port_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "relayId".into(),
                    value: &relay_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sendKeyName".into(),
                    value: &send_key_name_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        FunctionAppHybridConnectionResult {
            function_app_id: o.get_field("functionAppId"),
            hostname: o.get_field("hostname"),
            namespace_name: o.get_field("namespaceName"),
            port: o.get_field("port"),
            relay_id: o.get_field("relayId"),
            relay_name: o.get_field("relayName"),
            send_key_name: o.get_field("sendKeyName"),
            send_key_value: o.get_field("sendKeyValue"),
            service_bus_namespace: o.get_field("serviceBusNamespace"),
            service_bus_suffix: o.get_field("serviceBusSuffix"),
        }
    }
}
