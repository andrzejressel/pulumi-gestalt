/// Manages an IotHub Device Provisioning Service Certificate.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-resources
///       location: West Europe
///   exampleIotHubDps:
///     type: azure:iot:IotHubDps
///     name: example
///     properties:
///       name: example
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       sku:
///         name: S1
///         capacity: '1'
///   exampleIotHubCertificate:
///     type: azure:iot:IotHubCertificate
///     name: example
///     properties:
///       name: example
///       resourceGroupName: ${example.name}
///       iotDpsName: ${exampleIotHubDps.name}
///       certificateContent:
///         fn::invoke:
///           function: std:filebase64
///           arguments:
///             input: example.cer
///           return: result
/// ```
///
/// ## Import
///
/// IoTHub Device Provisioning Service Certificates can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:iot/iotHubCertificate:IotHubCertificate example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.Devices/provisioningServices/example/certificates/example
/// ```
///
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod iot_hub_certificate {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct IotHubCertificateArgs {
        /// The Base-64 representation of the X509 leaf certificate .cer file or just a .pem file content.
        #[builder(into)]
        pub certificate_content: pulumi_gestalt_rust::Input<String>,
        /// The name of the IoT Device Provisioning Service that this certificate will be attached to. Changing this forces a new resource to be created.
        #[builder(into)]
        pub iot_dps_name: pulumi_gestalt_rust::Input<String>,
        /// Specifies if the certificate is created in verified state. Defaults to `false`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub is_verified: pulumi_gestalt_rust::Input<Option<bool>>,
        /// Specifies the name of the Iot Device Provisioning Service Certificate resource. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::Input<Option<String>>,
        /// The name of the resource group under which the Iot Device Provisioning Service Certificate resource has to be created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::Input<String>,
    }
    #[allow(dead_code)]
    pub struct IotHubCertificateResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// The Base-64 representation of the X509 leaf certificate .cer file or just a .pem file content.
        pub certificate_content: pulumi_gestalt_rust::Output<String>,
        /// The name of the IoT Device Provisioning Service that this certificate will be attached to. Changing this forces a new resource to be created.
        pub iot_dps_name: pulumi_gestalt_rust::Output<String>,
        /// Specifies if the certificate is created in verified state. Defaults to `false`. Changing this forces a new resource to be created.
        pub is_verified: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Specifies the name of the Iot Device Provisioning Service Certificate resource. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The name of the resource group under which the Iot Device Provisioning Service Certificate resource has to be created. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: IotHubCertificateArgs,
    ) -> IotHubCertificateResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: IotHubCertificateArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> IotHubCertificateResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: IotHubCertificateArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> IotHubCertificateResult {
        let certificate_content_binding = args.certificate_content.get_output(ctx);
        let iot_dps_name_binding = args.iot_dps_name.get_output(ctx);
        let is_verified_binding = args.is_verified.get_output(ctx);
        let name_binding = args.name.get_output(ctx);
        let resource_group_name_binding = args.resource_group_name.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:iot/iotHubCertificate:IotHubCertificate".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "certificateContent".into(),
                    value: &certificate_content_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "iotDpsName".into(),
                    value: &iot_dps_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "isVerified".into(),
                    value: &is_verified_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        IotHubCertificateResult {
            id: o.get_id(),
            urn: o.get_urn(),
            certificate_content: o.get_field("certificateContent"),
            iot_dps_name: o.get_field("iotDpsName"),
            is_verified: o.get_field("isVerified"),
            name: o.get_field("name"),
            resource_group_name: o.get_field("resourceGroupName"),
        }
    }
}
