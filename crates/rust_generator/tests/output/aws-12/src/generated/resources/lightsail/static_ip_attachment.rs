/// Provides a static IP address attachment - relationship between a Lightsail static IP & Lightsail instance.
///
/// > **Note:** Lightsail is currently only supported in a limited number of AWS Regions, please see ["Regions and Availability Zones in Amazon Lightsail"](https://lightsail.aws.amazon.com/ls/docs/overview/article/understanding-regions-and-availability-zones-in-amazon-lightsail) for more details
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let test = static_ip_attachment::create(
///         "test",
///         StaticIpAttachmentArgs::builder()
///             .instance_name("${testInstance.id}")
///             .static_ip_name("${testStaticIp.id}")
///             .build_struct(),
///     );
///     let testInstance = instance::create(
///         "testInstance",
///         InstanceArgs::builder()
///             .availability_zone("us-east-1b")
///             .blueprint_id("string")
///             .bundle_id("string")
///             .key_pair_name("some_key_name")
///             .name("example")
///             .build_struct(),
///     );
///     let testStaticIp = static_ip::create(
///         "testStaticIp",
///         StaticIpArgs::builder().name("example").build_struct(),
///     );
/// }
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod static_ip_attachment {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct StaticIpAttachmentArgs {
        /// The name of the Lightsail instance to attach the IP to
        #[builder(into)]
        pub instance_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the allocated static IP
        #[builder(into)]
        pub static_ip_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct StaticIpAttachmentResult {
        /// The name of the Lightsail instance to attach the IP to
        pub instance_name: pulumi_gestalt_rust::Output<String>,
        /// The allocated static IP address
        pub ip_address: pulumi_gestalt_rust::Output<String>,
        /// The name of the allocated static IP
        pub static_ip_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: StaticIpAttachmentArgs,
    ) -> StaticIpAttachmentResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let instance_name_binding = args.instance_name.get_output(context);
        let static_ip_name_binding = args.static_ip_name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:lightsail/staticIpAttachment:StaticIpAttachment".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "instanceName".into(),
                    value: &instance_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "staticIpName".into(),
                    value: &static_ip_name_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        StaticIpAttachmentResult {
            instance_name: o.get_field("instanceName"),
            ip_address: o.get_field("ipAddress"),
            static_ip_name: o.get_field("staticIpName"),
        }
    }
}
