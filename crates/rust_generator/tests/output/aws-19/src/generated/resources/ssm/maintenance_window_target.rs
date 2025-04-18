/// Provides an SSM Maintenance Window Target resource
///
/// ## Example Usage
///
/// ### Instance Target
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let target1 = maintenance_window_target::create(
///         "target1",
///         MaintenanceWindowTargetArgs::builder()
///             .description("This is a maintenance window target")
///             .name("maintenance-window-target")
///             .resource_type("INSTANCE")
///             .targets(
///                 vec![
///                     MaintenanceWindowTargetTarget::builder().key("tag:Name")
///                     .values(vec!["acceptance_test",]).build_struct(),
///                 ],
///             )
///             .window_id("${window.id}")
///             .build_struct(),
///     );
///     let window = maintenance_window::create(
///         "window",
///         MaintenanceWindowArgs::builder()
///             .cutoff(1)
///             .duration(3)
///             .name("maintenance-window-webapp")
///             .schedule("cron(0 16 ? * TUE *)")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Resource Group Target
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let target1 = maintenance_window_target::create(
///         "target1",
///         MaintenanceWindowTargetArgs::builder()
///             .description("This is a maintenance window target")
///             .name("maintenance-window-target")
///             .resource_type("RESOURCE_GROUP")
///             .targets(
///                 vec![
///                     MaintenanceWindowTargetTarget::builder()
///                     .key("resource-groups:ResourceTypeFilters")
///                     .values(vec!["AWS::EC2::Instance",]).build_struct(),
///                 ],
///             )
///             .window_id("${window.id}")
///             .build_struct(),
///     );
///     let window = maintenance_window::create(
///         "window",
///         MaintenanceWindowArgs::builder()
///             .cutoff(1)
///             .duration(3)
///             .name("maintenance-window-webapp")
///             .schedule("cron(0 16 ? * TUE *)")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import SSM Maintenance Window targets using `WINDOW_ID/WINDOW_TARGET_ID`. For example:
///
/// ```sh
/// $ pulumi import aws:ssm/maintenanceWindowTarget:MaintenanceWindowTarget example mw-0c50858d01EXAMPLE/23639a0b-ddbc-4bca-9e72-78d96EXAMPLE
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod maintenance_window_target {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct MaintenanceWindowTargetArgs {
        /// The description of the maintenance window target.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the maintenance window target.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// User-provided value that will be included in any CloudWatch events raised while running tasks for these targets in this Maintenance Window.
        #[builder(into, default)]
        pub owner_information: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The type of target being registered with the Maintenance Window. Possible values are `INSTANCE` and `RESOURCE_GROUP`.
        #[builder(into)]
        pub resource_type: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The targets to register with the maintenance window. In other words, the instances to run commands on when the maintenance window runs. You can specify targets using instance IDs, resource group names, or tags that have been applied to instances. For more information about these examples formats see
        /// (https://docs.aws.amazon.com/systems-manager/latest/userguide/mw-cli-tutorial-targets-examples.html)
        #[builder(into)]
        pub targets: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::ssm::MaintenanceWindowTargetTarget>,
        >,
        /// The Id of the maintenance window to register the target with.
        #[builder(into)]
        pub window_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct MaintenanceWindowTargetResult {
        /// The description of the maintenance window target.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the maintenance window target.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// User-provided value that will be included in any CloudWatch events raised while running tasks for these targets in this Maintenance Window.
        pub owner_information: pulumi_gestalt_rust::Output<Option<String>>,
        /// The type of target being registered with the Maintenance Window. Possible values are `INSTANCE` and `RESOURCE_GROUP`.
        pub resource_type: pulumi_gestalt_rust::Output<String>,
        /// The targets to register with the maintenance window. In other words, the instances to run commands on when the maintenance window runs. You can specify targets using instance IDs, resource group names, or tags that have been applied to instances. For more information about these examples formats see
        /// (https://docs.aws.amazon.com/systems-manager/latest/userguide/mw-cli-tutorial-targets-examples.html)
        pub targets: pulumi_gestalt_rust::Output<
            Vec<super::super::types::ssm::MaintenanceWindowTargetTarget>,
        >,
        /// The Id of the maintenance window to register the target with.
        pub window_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: MaintenanceWindowTargetArgs,
    ) -> MaintenanceWindowTargetResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let name_binding = args.name.get_output(context);
        let owner_information_binding = args.owner_information.get_output(context);
        let resource_type_binding = args.resource_type.get_output(context);
        let targets_binding = args.targets.get_output(context);
        let window_id_binding = args.window_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:ssm/maintenanceWindowTarget:MaintenanceWindowTarget".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ownerInformation".into(),
                    value: &owner_information_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceType".into(),
                    value: &resource_type_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "targets".into(),
                    value: &targets_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "windowId".into(),
                    value: &window_id_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        MaintenanceWindowTargetResult {
            description: o.get_field("description"),
            name: o.get_field("name"),
            owner_information: o.get_field("ownerInformation"),
            resource_type: o.get_field("resourceType"),
            targets: o.get_field("targets"),
            window_id: o.get_field("windowId"),
        }
    }
}
