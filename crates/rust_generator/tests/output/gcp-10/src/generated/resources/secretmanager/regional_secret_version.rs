/// A regional secret version resource.
///
///
///
///
///
/// ## Example Usage
///
/// ### Regional Secret Version Basic
///
///
/// ```yaml
/// resources:
///   secret-basic:
///     type: gcp:secretmanager:RegionalSecret
///     properties:
///       secretId: secret-version
///       location: us-central1
///   regionalSecretVersionBasic:
///     type: gcp:secretmanager:RegionalSecretVersion
///     name: regional_secret_version_basic
///     properties:
///       secret: ${["secret-basic"].id}
///       secretData: secret-data
/// ```
/// ### Regional Secret Version With Base64 Data
///
///
/// ```yaml
/// resources:
///   secret-basic:
///     type: gcp:secretmanager:RegionalSecret
///     properties:
///       secretId: secret-version
///       location: us-central1
///   regionalSecretVersionBase64:
///     type: gcp:secretmanager:RegionalSecretVersion
///     name: regional_secret_version_base64
///     properties:
///       secret: ${["secret-basic"].id}
///       secretData:
///         fn::invoke:
///           function: std:filebase64
///           arguments:
///             input: secret-data.pfx
///           return: result
///       isSecretDataBase64: true
/// ```
/// ### Regional Secret Version Disabled
///
///
/// ```yaml
/// resources:
///   secret-basic:
///     type: gcp:secretmanager:RegionalSecret
///     properties:
///       secretId: secret-version
///       location: us-central1
///   regionalSecretVersionDisabled:
///     type: gcp:secretmanager:RegionalSecretVersion
///     name: regional_secret_version_disabled
///     properties:
///       secret: ${["secret-basic"].id}
///       secretData: secret-data
///       enabled: false
/// ```
/// ### Regional Secret Version Deletion Policy Abandon
///
///
/// ```yaml
/// resources:
///   secret-basic:
///     type: gcp:secretmanager:RegionalSecret
///     properties:
///       secretId: secret-version
///       location: us-central1
///   regionalSecretVersionDeletionPolicy:
///     type: gcp:secretmanager:RegionalSecretVersion
///     name: regional_secret_version_deletion_policy
///     properties:
///       secret: ${["secret-basic"].id}
///       secretData: secret-data
///       deletionPolicy: ABANDON
/// ```
/// ### Regional Secret Version Deletion Policy Disable
///
///
/// ```yaml
/// resources:
///   secret-basic:
///     type: gcp:secretmanager:RegionalSecret
///     properties:
///       secretId: secret-version
///       location: us-central1
///   regionalSecretVersionDeletionPolicy:
///     type: gcp:secretmanager:RegionalSecretVersion
///     name: regional_secret_version_deletion_policy
///     properties:
///       secret: ${["secret-basic"].id}
///       secretData: secret-data
///       deletionPolicy: DISABLE
/// ```
///
/// ## Import
///
/// RegionalSecretVersion can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/secrets/{{secret_id}}/versions/{{version}}`
///
/// When using the `pulumi import` command, RegionalSecretVersion can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:secretmanager/regionalSecretVersion:RegionalSecretVersion default projects/{{project}}/locations/{{location}}/secrets/{{secret_id}}/versions/{{version}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod regional_secret_version {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RegionalSecretVersionArgs {
        /// The deletion policy for the regional secret version. Setting `ABANDON` allows the resource
        /// to be abandoned rather than deleted. Setting `DISABLE` allows the resource to be
        /// disabled rather than deleted. Default is `DELETE`. Possible values are:
        /// * DELETE
        /// * DISABLE
        /// * ABANDON
        #[builder(into, default)]
        pub deletion_policy: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The current state of the regional secret version.
        #[builder(into, default)]
        pub enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// If set to 'true', the secret data is expected to be base64-encoded string and would be sent as is.
        #[builder(into, default)]
        pub is_secret_data_base64: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Secret Manager regional secret resource.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub secret: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The secret data. Must be no larger than 64KiB.
        /// **Note**: This property is sensitive and will not be displayed in the plan.
        #[builder(into)]
        pub secret_data: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct RegionalSecretVersionResult {
        /// The time at which the regional secret version was created.
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// The customer-managed encryption configuration of the regional secret.
        /// Structure is documented below.
        pub customer_managed_encryptions: pulumi_gestalt_rust::Output<
            Vec<
                super::super::types::secretmanager::RegionalSecretVersionCustomerManagedEncryption,
            >,
        >,
        /// The deletion policy for the regional secret version. Setting `ABANDON` allows the resource
        /// to be abandoned rather than deleted. Setting `DISABLE` allows the resource to be
        /// disabled rather than deleted. Default is `DELETE`. Possible values are:
        /// * DELETE
        /// * DISABLE
        /// * ABANDON
        pub deletion_policy: pulumi_gestalt_rust::Output<Option<String>>,
        /// The time at which the regional secret version was destroyed. Only present if state is DESTROYED.
        pub destroy_time: pulumi_gestalt_rust::Output<String>,
        /// The current state of the regional secret version.
        pub enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// If set to 'true', the secret data is expected to be base64-encoded string and would be sent as is.
        pub is_secret_data_base64: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Location of Secret Manager regional secret resource.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The resource name of the regional secret version. Format:
        /// `projects/{{project}}/locations/{{location}}/secrets/{{secret_id}}/versions/{{version}}`
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Secret Manager regional secret resource.
        ///
        ///
        /// - - -
        pub secret: pulumi_gestalt_rust::Output<String>,
        /// The secret data. Must be no larger than 64KiB.
        /// **Note**: This property is sensitive and will not be displayed in the plan.
        pub secret_data: pulumi_gestalt_rust::Output<String>,
        /// The version of the Regional Secret.
        pub version: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: RegionalSecretVersionArgs,
    ) -> RegionalSecretVersionResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let deletion_policy_binding = args.deletion_policy.get_output(context);
        let enabled_binding = args.enabled.get_output(context);
        let is_secret_data_base64_binding = args
            .is_secret_data_base64
            .get_output(context);
        let secret_binding = args.secret.get_output(context);
        let secret_data_binding = args.secret_data.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:secretmanager/regionalSecretVersion:RegionalSecretVersion"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "deletionPolicy".into(),
                    value: &deletion_policy_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enabled".into(),
                    value: &enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "isSecretDataBase64".into(),
                    value: &is_secret_data_base64_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "secret".into(),
                    value: &secret_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "secretData".into(),
                    value: &secret_data_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        RegionalSecretVersionResult {
            create_time: o.get_field("createTime"),
            customer_managed_encryptions: o.get_field("customerManagedEncryptions"),
            deletion_policy: o.get_field("deletionPolicy"),
            destroy_time: o.get_field("destroyTime"),
            enabled: o.get_field("enabled"),
            is_secret_data_base64: o.get_field("isSecretDataBase64"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            secret: o.get_field("secret"),
            secret_data: o.get_field("secretData"),
            version: o.get_field("version"),
        }
    }
}
