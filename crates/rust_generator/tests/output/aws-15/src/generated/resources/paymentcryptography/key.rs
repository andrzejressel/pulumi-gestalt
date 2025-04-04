/// Resource for managing an AWS Payment Cryptography Control Plane Key.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```yaml
/// resources:
///   test:
///     type: aws:paymentcryptography:Key
///     properties:
///       exportable: true
///       keyAttributes:
///         - keyAlgorithm: TDES_3KEY
///           keyClass: SYMMETRIC_KEY
///           keyUsage: TR31_P0_PIN_ENCRYPTION_KEY
///           keyModesOfUse:
///             - decrypt: true
///               encrypt: true
///               wrap: true
///               unwrap: true
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Payment Cryptography Control Plane Key using the `arn:aws:payment-cryptography:us-east-1:123456789012:key/qtbojf64yshyvyzf`. For example:
///
/// ```sh
/// $ pulumi import aws:paymentcryptography/key:Key example arn:aws:payment-cryptography:us-east-1:123456789012:key/qtbojf64yshyvyzf
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod key {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct KeyArgs {
        #[builder(into, default)]
        pub deletion_window_in_days: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Whether to enable the key.
        #[builder(into, default)]
        pub enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Whether the key is exportable from the service.
        #[builder(into)]
        pub exportable: pulumi_gestalt_rust::InputOrOutput<bool>,
        /// Role of the key, the algorithm it supports, and the cryptographic operations allowed with the key.
        ///
        /// The following arguments are optional:
        #[builder(into, default)]
        pub key_attributes: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::paymentcryptography::KeyKeyAttributes>,
        >,
        /// Algorithm that AWS Payment Cryptography uses to calculate the key check value (KCV).
        #[builder(into, default)]
        pub key_check_value_algorithm: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Map of tags assigned to the WorkSpaces Connection Alias. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        #[builder(into, default)]
        pub timeouts: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::paymentcryptography::KeyTimeouts>,
        >,
    }
    #[allow(dead_code)]
    pub struct KeyResult {
        /// ARN of the key.
        pub arn: pulumi_gestalt_rust::Output<String>,
        pub deletion_window_in_days: pulumi_gestalt_rust::Output<i32>,
        /// Whether to enable the key.
        pub enabled: pulumi_gestalt_rust::Output<bool>,
        /// Whether the key is exportable from the service.
        pub exportable: pulumi_gestalt_rust::Output<bool>,
        /// Role of the key, the algorithm it supports, and the cryptographic operations allowed with the key.
        ///
        /// The following arguments are optional:
        pub key_attributes: pulumi_gestalt_rust::Output<
            Option<super::super::types::paymentcryptography::KeyKeyAttributes>,
        >,
        /// Key check value (KCV) is used to check if all parties holding a given key have the same key or to detect that a key has changed.
        pub key_check_value: pulumi_gestalt_rust::Output<String>,
        /// Algorithm that AWS Payment Cryptography uses to calculate the key check value (KCV).
        pub key_check_value_algorithm: pulumi_gestalt_rust::Output<String>,
        /// Source of the key material.
        pub key_origin: pulumi_gestalt_rust::Output<String>,
        /// State of key that is being created or deleted.
        pub key_state: pulumi_gestalt_rust::Output<String>,
        /// Map of tags assigned to the WorkSpaces Connection Alias. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub timeouts: pulumi_gestalt_rust::Output<
            Option<super::super::types::paymentcryptography::KeyTimeouts>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: KeyArgs,
    ) -> KeyResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let deletion_window_in_days_binding = args
            .deletion_window_in_days
            .get_output(context);
        let enabled_binding = args.enabled.get_output(context);
        let exportable_binding = args.exportable.get_output(context);
        let key_attributes_binding = args.key_attributes.get_output(context);
        let key_check_value_algorithm_binding = args
            .key_check_value_algorithm
            .get_output(context);
        let tags_binding = args.tags.get_output(context);
        let timeouts_binding = args.timeouts.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:paymentcryptography/key:Key".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "deletionWindowInDays".into(),
                    value: &deletion_window_in_days_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enabled".into(),
                    value: &enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "exportable".into(),
                    value: &exportable_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "keyAttributes".into(),
                    value: &key_attributes_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "keyCheckValueAlgorithm".into(),
                    value: &key_check_value_algorithm_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "timeouts".into(),
                    value: &timeouts_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        KeyResult {
            arn: o.get_field("arn"),
            deletion_window_in_days: o.get_field("deletionWindowInDays"),
            enabled: o.get_field("enabled"),
            exportable: o.get_field("exportable"),
            key_attributes: o.get_field("keyAttributes"),
            key_check_value: o.get_field("keyCheckValue"),
            key_check_value_algorithm: o.get_field("keyCheckValueAlgorithm"),
            key_origin: o.get_field("keyOrigin"),
            key_state: o.get_field("keyState"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            timeouts: o.get_field("timeouts"),
        }
    }
}
