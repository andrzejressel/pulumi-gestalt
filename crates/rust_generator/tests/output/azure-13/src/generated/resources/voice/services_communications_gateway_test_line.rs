/// Manages a Voice Services Communications Gateway Test Line.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-resources
///       location: West Central US
///   exampleServicesCommunicationsGateway:
///     type: azure:voice:ServicesCommunicationsGateway
///     name: example
///     properties:
///       name: example-vcg
///       resourceGroupName: ${example.name}
///   exampleServicesCommunicationsGatewayTestLine:
///     type: azure:voice:ServicesCommunicationsGatewayTestLine
///     name: example
///     properties:
///       name: example-vtl
///       location: West Central US
///       voiceServicesCommunicationsGatewayId: ${exampleServicesCommunicationsGateway.id}
///       phoneNumber: '123456789'
///       purpose: Automated
///       tags:
///         key: value
/// ```
///
/// ## Import
///
/// Voice Services Communications Gateway Test Line can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:voice/servicesCommunicationsGatewayTestLine:ServicesCommunicationsGatewayTestLine example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/resourceGroup1/providers/Microsoft.VoiceServices/communicationsGateways/communicationsGateway1/testLines/testLine1
/// ```
///
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod services_communications_gateway_test_line {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ServicesCommunicationsGatewayTestLineArgs {
        /// Specifies the Azure Region where the Voice Services Communications Gateway Test Line should exist. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::Input<Option<String>>,
        /// Specifies the name which should be used for this Voice Services Communications Gateway Test Line. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::Input<Option<String>>,
        /// Specifies the phone number.
        #[builder(into)]
        pub phone_number: pulumi_gestalt_rust::Input<String>,
        /// The purpose of the Voice Services Communications Gateway Test Line. Possible values are `Automated` or `Manual`.
        #[builder(into)]
        pub purpose: pulumi_gestalt_rust::Input<String>,
        /// A mapping of tags which should be assigned to the Voice Services Communications Gateway Test Line.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::Input<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies the ID of the Voice Services Communications Gateway. Changing this forces a new resource to be created.
        #[builder(into)]
        pub voice_services_communications_gateway_id: pulumi_gestalt_rust::Input<String>,
    }
    #[allow(dead_code)]
    pub struct ServicesCommunicationsGatewayTestLineResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// Specifies the Azure Region where the Voice Services Communications Gateway Test Line should exist. Changing this forces a new resource to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name which should be used for this Voice Services Communications Gateway Test Line. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Specifies the phone number.
        pub phone_number: pulumi_gestalt_rust::Output<String>,
        /// The purpose of the Voice Services Communications Gateway Test Line. Possible values are `Automated` or `Manual`.
        pub purpose: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags which should be assigned to the Voice Services Communications Gateway Test Line.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies the ID of the Voice Services Communications Gateway. Changing this forces a new resource to be created.
        pub voice_services_communications_gateway_id: pulumi_gestalt_rust::Output<
            String,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ServicesCommunicationsGatewayTestLineArgs,
    ) -> ServicesCommunicationsGatewayTestLineResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ServicesCommunicationsGatewayTestLineArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> ServicesCommunicationsGatewayTestLineResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ServicesCommunicationsGatewayTestLineArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> ServicesCommunicationsGatewayTestLineResult {
        let location_binding = args.location.get_output(ctx);
        let name_binding = args.name.get_output(ctx);
        let phone_number_binding = args.phone_number.get_output(ctx);
        let purpose_binding = args.purpose.get_output(ctx);
        let tags_binding = args.tags.get_output(ctx);
        let voice_services_communications_gateway_id_binding = args
            .voice_services_communications_gateway_id
            .get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:voice/servicesCommunicationsGatewayTestLine:ServicesCommunicationsGatewayTestLine"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: &location_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "phoneNumber".into(),
                    value: &phone_number_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "purpose".into(),
                    value: &purpose_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "voiceServicesCommunicationsGatewayId".into(),
                    value: &voice_services_communications_gateway_id_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        ServicesCommunicationsGatewayTestLineResult {
            id: o.get_id(),
            urn: o.get_urn(),
            location: o.get_field("location"),
            name: o.get_field("name"),
            phone_number: o.get_field("phoneNumber"),
            purpose: o.get_field("purpose"),
            tags: o.get_field("tags"),
            voice_services_communications_gateway_id: o
                .get_field("voiceServicesCommunicationsGatewayId"),
        }
    }
}
