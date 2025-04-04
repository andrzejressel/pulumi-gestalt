#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_network {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetNetworkArgs {
        /// Specifies the name which should be used for this Mobile Network.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the name of the Resource Group where the Mobile Network should exist.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetNetworkResult {
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The Azure Region where the Mobile Network should exist. Changing this forces a new Mobile Network to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Mobile country code (MCC), defined in https://www.itu.int/rec/T-REC-E.212 .
        pub mobile_country_code: pulumi_gestalt_rust::Output<String>,
        /// Mobile network code (MNC), defined in https://www.itu.int/rec/T-REC-E.212 .
        pub mobile_network_code: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The mobile network resource identifier.
        pub service_key: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags which should be assigned to the Mobile Network.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetNetworkArgs,
    ) -> GetNetworkResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:mobile/getNetwork:getNetwork".into(),
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
        GetNetworkResult {
            id: o.get_field("id"),
            location: o.get_field("location"),
            mobile_country_code: o.get_field("mobileCountryCode"),
            mobile_network_code: o.get_field("mobileNetworkCode"),
            name: o.get_field("name"),
            resource_group_name: o.get_field("resourceGroupName"),
            service_key: o.get_field("serviceKey"),
            tags: o.get_field("tags"),
        }
    }
}
