#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_network_service {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetNetworkServiceArgs {
        /// Specifies the ID of the Mobile Network Service.
        #[builder(into)]
        pub mobile_network_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the name which should be used for this Mobile Network Service.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetNetworkServiceResult {
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The Azure Region where the Mobile Network Service should exist.
        pub location: pulumi_gestalt_rust::Output<String>,
        pub mobile_network_id: pulumi_gestalt_rust::Output<String>,
        /// The name of the data flow template. This must be unique within the parent data flow policy rule.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A `pcc_rule` block as defined below. The set of PCC Rules that make up this service.
        pub pcc_rules: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::mobile::GetNetworkServicePccRule>,
        >,
        /// A precedence value that is used to decide between services when identifying the QoS values to use for a particular SIM. A lower value means a higher priority.
        pub service_precedence: pulumi_gestalt_rust::Output<i32>,
        /// A `service_qos_policy` block as defined below. The QoS policy to use for packets matching this service.
        pub service_qos_policies: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::mobile::GetNetworkServiceServiceQosPolicy>,
        >,
        /// A mapping of tags which should be assigned to the Mobile Network Service.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetNetworkServiceArgs,
    ) -> GetNetworkServiceResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let mobile_network_id_binding = args.mobile_network_id.get_output(context);
        let name_binding = args.name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:mobile/getNetworkService:getNetworkService".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "mobileNetworkId".into(),
                    value: &mobile_network_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetNetworkServiceResult {
            id: o.get_field("id"),
            location: o.get_field("location"),
            mobile_network_id: o.get_field("mobileNetworkId"),
            name: o.get_field("name"),
            pcc_rules: o.get_field("pccRules"),
            service_precedence: o.get_field("servicePrecedence"),
            service_qos_policies: o.get_field("serviceQosPolicies"),
            tags: o.get_field("tags"),
        }
    }
}
