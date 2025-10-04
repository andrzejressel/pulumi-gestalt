/// Provides an IAM Virtual MFA Device.
///
/// > **Note:** All attributes will be stored in the raw state as plain-text.
/// > **Note:** A virtual MFA device cannot be directly associated with an IAM User from the provider.
///   To associate the virtual MFA device with a user and enable it, use the code returned in either `base_32_string_seed` or `qr_code_png` to generate TOTP authentication codes.
///   The authentication codes can then be used with the AWS CLI command [`aws iam enable-mfa-device`](https://docs.aws.amazon.com/cli/latest/reference/iam/enable-mfa-device.html) or the AWS API call [`EnableMFADevice`](https://docs.aws.amazon.com/IAM/latest/APIReference/API_EnableMFADevice.html).
///
/// ## Example Usage
///
/// **Using certs on file:**
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = virtual_mfa_device::create(
///         "example",
///         VirtualMfaDeviceArgs::builder().virtual_mfa_device_name("example").build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import IAM Virtual MFA Devices using the `arn`. For example:
///
/// ```sh
/// $ pulumi import aws:iam/virtualMfaDevice:VirtualMfaDevice example arn:aws:iam::123456789012:mfa/example
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod virtual_mfa_device {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VirtualMfaDeviceArgs {
        /// The path for the virtual MFA device.
        #[builder(into, default)]
        pub path: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Map of resource tags for the virtual mfa device. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The name of the virtual MFA device. Use with path to uniquely identify a virtual MFA device.
        #[builder(into)]
        pub virtual_mfa_device_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct VirtualMfaDeviceResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The Amazon Resource Name (ARN) specifying the virtual mfa device.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The base32 seed defined as specified in [RFC3548](https://tools.ietf.org/html/rfc3548.txt). The `base_32_string_seed` is base64-encoded.
        pub base32_string_seed: pulumi_gestalt_rust::Output<String>,
        /// The date and time when the virtual MFA device was enabled.
        pub enable_date: pulumi_gestalt_rust::Output<String>,
        /// The path for the virtual MFA device.
        pub path: pulumi_gestalt_rust::Output<Option<String>>,
        /// A QR code PNG image that encodes `otpauth://totp/$virtualMFADeviceName@$AccountName?secret=$Base32String` where `$virtualMFADeviceName` is one of the create call arguments. AccountName is the user name if set (otherwise, the account ID), and Base32String is the seed in base32 format.
        pub qr_code_png: pulumi_gestalt_rust::Output<String>,
        /// Map of resource tags for the virtual mfa device. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The associated IAM User name if the virtual MFA device is enabled.
        pub user_name: pulumi_gestalt_rust::Output<String>,
        /// The name of the virtual MFA device. Use with path to uniquely identify a virtual MFA device.
        pub virtual_mfa_device_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: VirtualMfaDeviceArgs,
    ) -> VirtualMfaDeviceResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let path_binding = args.path.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let virtual_mfa_device_name_binding = args
            .virtual_mfa_device_name
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:iam/virtualMfaDevice:VirtualMfaDevice".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "path".into(),
                    value: &path_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "virtualMfaDeviceName".into(),
                    value: &virtual_mfa_device_name_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        VirtualMfaDeviceResult {
            id: o.get_field("id"),
            arn: o.get_field("arn"),
            base32_string_seed: o.get_field("base32StringSeed"),
            enable_date: o.get_field("enableDate"),
            path: o.get_field("path"),
            qr_code_png: o.get_field("qrCodePng"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            user_name: o.get_field("userName"),
            virtual_mfa_device_name: o.get_field("virtualMfaDeviceName"),
        }
    }
}
