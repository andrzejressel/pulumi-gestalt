/// Manages a Stack HCI Deployment Setting.
///
/// > Note: Completion of the prerequisites of deploying the Azure Stack HCI in your environment is outside the scope of this document. For more details refer to the [Azure Stack HCI deployment sequence](https://learn.microsoft.com/en-us/azure-stack/hci/deploy/deployment-introduction#deployment-sequence). If you encounter issues completing the prerequisites, we'd recommend opening a ticket with Microsoft Support.
///
/// > Note: During the deployment process, the service will generate additional resources, including a new Arc Bridge Appliance and a Custom Location containing several Stack HCI Storage Paths. The provider will attempt to remove these resources on the deletion or recreation of `azure.stack.HciDeploymentSetting`.
///
/// ## Import
///
/// Stack HCI Deployment Settings can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:stack/hciDeploymentSetting:HciDeploymentSetting example /subscriptions/12345678-1234-9876-4563-123456789012/resourceGroups/group1/providers/Microsoft.AzureStackHCI/clusters/clus1/deploymentSettings/default
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod hci_deployment_setting {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct HciDeploymentSettingArgs {
        /// Specifies a list of IDs of Azure ARC machine resource to be part of cluster. Changing this forces a new Stack HCI Deployment Setting to be created.
        #[builder(into)]
        pub arc_resource_ids: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
        /// One or more `scale_unit` blocks as defined below. Changing this forces a new Stack HCI Deployment Setting to be created.
        #[builder(into)]
        pub scale_units: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::stack::HciDeploymentSettingScaleUnit>,
        >,
        /// The ID of the Azure Stack HCI cluster. Changing this forces a new Stack HCI Deployment Setting to be created.
        #[builder(into)]
        pub stack_hci_cluster_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The deployment template version. The format must be a set of numbers separated by dots such as `10.0.0.0`. Changing this forces a new Stack HCI Deployment Setting to be created.
        #[builder(into)]
        pub version: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct HciDeploymentSettingResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Specifies a list of IDs of Azure ARC machine resource to be part of cluster. Changing this forces a new Stack HCI Deployment Setting to be created.
        pub arc_resource_ids: pulumi_gestalt_rust::Output<Vec<String>>,
        /// One or more `scale_unit` blocks as defined below. Changing this forces a new Stack HCI Deployment Setting to be created.
        pub scale_units: pulumi_gestalt_rust::Output<
            Vec<super::super::types::stack::HciDeploymentSettingScaleUnit>,
        >,
        /// The ID of the Azure Stack HCI cluster. Changing this forces a new Stack HCI Deployment Setting to be created.
        pub stack_hci_cluster_id: pulumi_gestalt_rust::Output<String>,
        /// The deployment template version. The format must be a set of numbers separated by dots such as `10.0.0.0`. Changing this forces a new Stack HCI Deployment Setting to be created.
        pub version: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: HciDeploymentSettingArgs,
    ) -> HciDeploymentSettingResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let arc_resource_ids_binding = args.arc_resource_ids.get_output(context);
        let scale_units_binding = args.scale_units.get_output(context);
        let stack_hci_cluster_id_binding = args.stack_hci_cluster_id.get_output(context);
        let version_binding = args.version.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:stack/hciDeploymentSetting:HciDeploymentSetting".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "arcResourceIds".into(),
                    value: &arc_resource_ids_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "scaleUnits".into(),
                    value: &scale_units_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "stackHciClusterId".into(),
                    value: &stack_hci_cluster_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "version".into(),
                    value: &version_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        HciDeploymentSettingResult {
            id: o.get_field("id"),
            arc_resource_ids: o.get_field("arcResourceIds"),
            scale_units: o.get_field("scaleUnits"),
            stack_hci_cluster_id: o.get_field("stackHciClusterId"),
            version: o.get_field("version"),
        }
    }
}
