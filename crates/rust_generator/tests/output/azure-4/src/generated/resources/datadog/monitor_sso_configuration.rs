/// Manages SingleSignOn on the datadog Monitor.
///
/// ## Example Usage
///
/// ### Enabling SSO on monitor
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-datadog
///       location: West US 2
///   exampleMonitor:
///     type: azure:datadog:Monitor
///     name: example
///     properties:
///       name: example-monitor
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       datadogOrganization:
///         apiKey: XXXX
///         applicationKey: XXXX
///       user:
///         name: Example
///         email: abc@xyz.com
///       skuName: Linked
///       identity:
///         type: SystemAssigned
///   exampleMonitorSsoConfiguration:
///     type: azure:datadog:MonitorSsoConfiguration
///     name: example
///     properties:
///       datadogMonitorId: ${exampleMonitor.id}
///       singleSignOnEnabled: Enable
///       enterpriseApplicationId: XXXX
/// ```
///
/// ## Import
///
/// SingleSignOn on the Datadog Monitor can be imported using the `signle sign on resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:datadog/monitorSsoConfiguration:MonitorSsoConfiguration example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Datadog/monitors/monitor1/singleSignOnConfigurations/default
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod monitor_sso_configuration {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct MonitorSsoConfigurationArgs {
        /// The Datadog Monitor Id which should be used for this Datadog Monitor SSO Configuration. Changing this forces a new Datadog Monitor SSO Configuration to be created.
        #[builder(into)]
        pub datadog_monitor_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The application Id to perform SSO operation.
        #[builder(into)]
        pub enterprise_application_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the SingleSignOn configuration. Defaults to `default`.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The state of SingleSignOn configuration. Possible values are `Enable` and `Disable`.
        #[builder(into)]
        pub single_sign_on_enabled: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct MonitorSsoConfigurationResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The Datadog Monitor Id which should be used for this Datadog Monitor SSO Configuration. Changing this forces a new Datadog Monitor SSO Configuration to be created.
        pub datadog_monitor_id: pulumi_gestalt_rust::Output<String>,
        /// The application Id to perform SSO operation.
        pub enterprise_application_id: pulumi_gestalt_rust::Output<String>,
        /// The SingleSignOn URL to login to Datadog org.
        pub login_url: pulumi_gestalt_rust::Output<String>,
        /// The name of the SingleSignOn configuration. Defaults to `default`.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The state of SingleSignOn configuration. Possible values are `Enable` and `Disable`.
        pub single_sign_on_enabled: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: MonitorSsoConfigurationArgs,
    ) -> MonitorSsoConfigurationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let datadog_monitor_id_binding = args.datadog_monitor_id.get_output(context);
        let enterprise_application_id_binding = args
            .enterprise_application_id
            .get_output(context);
        let name_binding = args.name.get_output(context);
        let single_sign_on_enabled_binding = args
            .single_sign_on_enabled
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:datadog/monitorSsoConfiguration:MonitorSsoConfiguration"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "datadogMonitorId".into(),
                    value: &datadog_monitor_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enterpriseApplicationId".into(),
                    value: &enterprise_application_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "singleSignOnEnabled".into(),
                    value: &single_sign_on_enabled_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        MonitorSsoConfigurationResult {
            id: o.get_field("id"),
            datadog_monitor_id: o.get_field("datadogMonitorId"),
            enterprise_application_id: o.get_field("enterpriseApplicationId"),
            login_url: o.get_field("loginUrl"),
            name: o.get_field("name"),
            single_sign_on_enabled: o.get_field("singleSignOnEnabled"),
        }
    }
}
