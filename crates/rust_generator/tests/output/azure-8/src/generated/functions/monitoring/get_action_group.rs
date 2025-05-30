#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_action_group {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetActionGroupArgs {
        /// Specifies the name of the Action Group.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the name of the resource group the Action Group is located in.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetActionGroupResult {
        /// One or more `arm_role_receiver` blocks as defined below.
        pub arm_role_receivers: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::monitoring::GetActionGroupArmRoleReceiver>,
        >,
        /// One or more `automation_runbook_receiver` blocks as defined below.
        pub automation_runbook_receivers: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::monitoring::GetActionGroupAutomationRunbookReceiver,
            >,
        >,
        /// One or more `azure_app_push_receiver` blocks as defined below.
        pub azure_app_push_receivers: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::monitoring::GetActionGroupAzureAppPushReceiver,
            >,
        >,
        /// One or more `azure_function_receiver` blocks as defined below.
        pub azure_function_receivers: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::monitoring::GetActionGroupAzureFunctionReceiver,
            >,
        >,
        /// One or more `email_receiver` blocks as defined below.
        pub email_receivers: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::monitoring::GetActionGroupEmailReceiver>,
        >,
        /// Whether this action group is enabled.
        pub enabled: pulumi_gestalt_rust::Output<bool>,
        /// One or more `event_hub_receiver` blocks as defined below.
        pub event_hub_receivers: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::monitoring::GetActionGroupEventHubReceiver>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// One or more `itsm_receiver` blocks as defined below.
        pub itsm_receivers: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::monitoring::GetActionGroupItsmReceiver>,
        >,
        /// One or more `logic_app_receiver` blocks as defined below.
        pub logic_app_receivers: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::monitoring::GetActionGroupLogicAppReceiver>,
        >,
        /// The name of the webhook receiver.
        pub name: pulumi_gestalt_rust::Output<String>,
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The short name of the action group.
        pub short_name: pulumi_gestalt_rust::Output<String>,
        /// One or more `sms_receiver` blocks as defined below.
        pub sms_receivers: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::monitoring::GetActionGroupSmsReceiver>,
        >,
        /// One or more `voice_receiver` blocks as defined below.
        pub voice_receivers: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::monitoring::GetActionGroupVoiceReceiver>,
        >,
        /// One or more `webhook_receiver` blocks as defined below.
        pub webhook_receivers: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::monitoring::GetActionGroupWebhookReceiver>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetActionGroupArgs,
    ) -> GetActionGroupResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:monitoring/getActionGroup:getActionGroup".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetActionGroupResult {
            arm_role_receivers: o.get_field("armRoleReceivers"),
            automation_runbook_receivers: o.get_field("automationRunbookReceivers"),
            azure_app_push_receivers: o.get_field("azureAppPushReceivers"),
            azure_function_receivers: o.get_field("azureFunctionReceivers"),
            email_receivers: o.get_field("emailReceivers"),
            enabled: o.get_field("enabled"),
            event_hub_receivers: o.get_field("eventHubReceivers"),
            id: o.get_field("id"),
            itsm_receivers: o.get_field("itsmReceivers"),
            logic_app_receivers: o.get_field("logicAppReceivers"),
            name: o.get_field("name"),
            resource_group_name: o.get_field("resourceGroupName"),
            short_name: o.get_field("shortName"),
            sms_receivers: o.get_field("smsReceivers"),
            voice_receivers: o.get_field("voiceReceivers"),
            webhook_receivers: o.get_field("webhookReceivers"),
        }
    }
}
