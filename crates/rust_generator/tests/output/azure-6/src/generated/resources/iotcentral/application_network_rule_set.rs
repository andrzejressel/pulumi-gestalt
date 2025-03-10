/// Manages an IoT Central Application Network Rule Set.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-resource
///       location: West Europe
///   exampleApplication:
///     type: azure:iotcentral:Application
///     name: example
///     properties:
///       name: example-iotcentral-app
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       subDomain: example-iotcentral-app-subdomain
///       displayName: example-iotcentral-app-display-name
///       sku: ST1
///       tags:
///         Foo: Bar
///   exampleApplicationNetworkRuleSet:
///     type: azure:iotcentral:ApplicationNetworkRuleSet
///     name: example
///     properties:
///       iotcentralApplicationId: ${exampleApplication.id}
///       ipRules:
///         - name: rule1
///           ipMask: 10.0.1.0/24
///         - name: rule2
///           ipMask: 10.1.1.0/24
/// ```
///
/// ## Import
///
/// IoT Central Application Network Rule Sets can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:iotcentral/applicationNetworkRuleSet:ApplicationNetworkRuleSet example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.IoTCentral/iotApps/app1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod application_network_rule_set {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ApplicationNetworkRuleSetArgs {
        /// Whether these IP Rules apply for device connectivity to IoT Hub and Device Provisioning Service associated with this IoT Central Application. Possible values are `true`, `false`. Defaults to `true`
        #[builder(into, default)]
        pub apply_to_device: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Specifies the default action for the IoT Central Application Network Rule Set. Possible values are `Allow` and `Deny`. Defaults to `Deny`.
        #[builder(into, default)]
        pub default_action: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the IoT Central Application. Changing this forces a new resource to be created.
        #[builder(into)]
        pub iotcentral_application_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// One or more `ip_rule` blocks as defined below.
        #[builder(into, default)]
        pub ip_rules: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::iotcentral::ApplicationNetworkRuleSetIpRule>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ApplicationNetworkRuleSetResult {
        /// Whether these IP Rules apply for device connectivity to IoT Hub and Device Provisioning Service associated with this IoT Central Application. Possible values are `true`, `false`. Defaults to `true`
        pub apply_to_device: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Specifies the default action for the IoT Central Application Network Rule Set. Possible values are `Allow` and `Deny`. Defaults to `Deny`.
        pub default_action: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ID of the IoT Central Application. Changing this forces a new resource to be created.
        pub iotcentral_application_id: pulumi_gestalt_rust::Output<String>,
        /// One or more `ip_rule` blocks as defined below.
        pub ip_rules: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::iotcentral::ApplicationNetworkRuleSetIpRule>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ApplicationNetworkRuleSetArgs,
    ) -> ApplicationNetworkRuleSetResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let apply_to_device_binding = args.apply_to_device.get_output(context);
        let default_action_binding = args.default_action.get_output(context);
        let iotcentral_application_id_binding = args
            .iotcentral_application_id
            .get_output(context);
        let ip_rules_binding = args.ip_rules.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:iotcentral/applicationNetworkRuleSet:ApplicationNetworkRuleSet"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "applyToDevice".into(),
                    value: &apply_to_device_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "defaultAction".into(),
                    value: &default_action_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "iotcentralApplicationId".into(),
                    value: &iotcentral_application_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ipRules".into(),
                    value: &ip_rules_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        ApplicationNetworkRuleSetResult {
            apply_to_device: o.get_field("applyToDevice"),
            default_action: o.get_field("defaultAction"),
            iotcentral_application_id: o.get_field("iotcentralApplicationId"),
            ip_rules: o.get_field("ipRules"),
        }
    }
}
