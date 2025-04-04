/// Provides a AWS WorkLink Fleet resource.
///
/// !> **WARNING:** The `aws.worklink.Fleet` resource has been deprecated and will be removed in a future version. Use Amazon WorkSpaces Secure Browser instead.
///
/// ## Example Usage
///
/// Basic usage:
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = fleet::create(
///         "example",
///         FleetArgs::builder().name("example").build_struct(),
///     );
/// }
/// ```
///
/// Network Configuration Usage:
///
///
/// Identity Provider Configuration Usage:
///
/// ```yaml
/// resources:
///   test:
///     type: aws:worklink:Fleet
///     properties:
///       name: tf-worklink-fleet
///       identityProvider:
///         type: SAML
///         samlMetadata:
///           fn::invoke:
///             function: std:file
///             arguments:
///               input: saml-metadata.xml
///             return: result
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import WorkLink using the ARN. For example:
///
/// ```sh
/// $ pulumi import aws:worklink/fleet:Fleet test arn:aws:worklink::123456789012:fleet/example
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod fleet {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FleetArgs {
        /// The ARN of the Amazon Kinesis data stream that receives the audit events. Kinesis data stream name must begin with `"AmazonWorkLink-"`.
        #[builder(into, default)]
        pub audit_stream_arn: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The certificate chain, including intermediate certificates and the root certificate authority certificate used to issue device certificates.
        #[builder(into, default)]
        pub device_ca_certificate: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the fleet.
        #[builder(into, default)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Provide this to allow manage the identity provider configuration for the fleet. Fields documented below.
        #[builder(into, default)]
        pub identity_provider: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::worklink::FleetIdentityProvider>,
        >,
        /// A region-unique name for the AMI.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Provide this to allow manage the company network configuration for the fleet. Fields documented below.
        #[builder(into, default)]
        pub network: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::worklink::FleetNetwork>,
        >,
        /// The option to optimize for better performance by routing traffic through the closest AWS Region to users, which may be outside of your home Region. Defaults to `true`.
        ///
        /// **network** requires the following:
        ///
        /// > **NOTE:** `network` is cannot removed without force recreating.
        #[builder(into, default)]
        pub optimize_for_end_user_location: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
    }
    #[allow(dead_code)]
    pub struct FleetResult {
        /// The ARN of the created WorkLink Fleet.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The ARN of the Amazon Kinesis data stream that receives the audit events. Kinesis data stream name must begin with `"AmazonWorkLink-"`.
        pub audit_stream_arn: pulumi_gestalt_rust::Output<Option<String>>,
        /// The identifier used by users to sign in to the Amazon WorkLink app.
        pub company_code: pulumi_gestalt_rust::Output<String>,
        /// The time that the fleet was created.
        pub created_time: pulumi_gestalt_rust::Output<String>,
        /// The certificate chain, including intermediate certificates and the root certificate authority certificate used to issue device certificates.
        pub device_ca_certificate: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the fleet.
        pub display_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// Provide this to allow manage the identity provider configuration for the fleet. Fields documented below.
        pub identity_provider: pulumi_gestalt_rust::Output<
            Option<super::super::types::worklink::FleetIdentityProvider>,
        >,
        /// The time that the fleet was last updated.
        pub last_updated_time: pulumi_gestalt_rust::Output<String>,
        /// A region-unique name for the AMI.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Provide this to allow manage the company network configuration for the fleet. Fields documented below.
        pub network: pulumi_gestalt_rust::Output<
            Option<super::super::types::worklink::FleetNetwork>,
        >,
        /// The option to optimize for better performance by routing traffic through the closest AWS Region to users, which may be outside of your home Region. Defaults to `true`.
        ///
        /// **network** requires the following:
        ///
        /// > **NOTE:** `network` is cannot removed without force recreating.
        pub optimize_for_end_user_location: pulumi_gestalt_rust::Output<Option<bool>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: FleetArgs,
    ) -> FleetResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let audit_stream_arn_binding = args.audit_stream_arn.get_output(context);
        let device_ca_certificate_binding = args
            .device_ca_certificate
            .get_output(context);
        let display_name_binding = args.display_name.get_output(context);
        let identity_provider_binding = args.identity_provider.get_output(context);
        let name_binding = args.name.get_output(context);
        let network_binding = args.network.get_output(context);
        let optimize_for_end_user_location_binding = args
            .optimize_for_end_user_location
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:worklink/fleet:Fleet".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "auditStreamArn".into(),
                    value: &audit_stream_arn_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "deviceCaCertificate".into(),
                    value: &device_ca_certificate_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "identityProvider".into(),
                    value: &identity_provider_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "network".into(),
                    value: &network_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "optimizeForEndUserLocation".into(),
                    value: &optimize_for_end_user_location_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        FleetResult {
            arn: o.get_field("arn"),
            audit_stream_arn: o.get_field("auditStreamArn"),
            company_code: o.get_field("companyCode"),
            created_time: o.get_field("createdTime"),
            device_ca_certificate: o.get_field("deviceCaCertificate"),
            display_name: o.get_field("displayName"),
            identity_provider: o.get_field("identityProvider"),
            last_updated_time: o.get_field("lastUpdatedTime"),
            name: o.get_field("name"),
            network: o.get_field("network"),
            optimize_for_end_user_location: o.get_field("optimizeForEndUserLocation"),
        }
    }
}
