/// Provides a resource to manage an S3 Access Grants instance resource policy.
/// Use a resource policy to manage cross-account access to your S3 Access Grants instance.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = access_grants_instance::create(
///         "example",
///         AccessGrantsInstanceArgs::builder().build_struct(),
///     );
///     let exampleAccessGrantsInstanceResourcePolicy = access_grants_instance_resource_policy::create(
///         "exampleAccessGrantsInstanceResourcePolicy",
///         AccessGrantsInstanceResourcePolicyArgs::builder()
///             .policy(
///                 "{\n  \"Version\": \"2012-10-17\",\n  \"Id\": \"S3AccessGrantsPolicy\",\n  \"Statement\": [{\n    \"Sid\": \"AllowAccessToS3AccessGrants\",\n    \"Effect\": \"Allow\",\n    \"Principal\": {\n      \"AWS\": \"123456789456\"\n    },\n    \"Action\": [\n      \"s3:ListAccessGrants\",\n      \"s3:ListAccessGrantsLocations\",\n      \"s3:GetDataAccess\"\n    ],\n    \"Resource\": \"${example.accessGrantsInstanceArn}\"\n  }]\n}",
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import S3 Access Grants instance resource policies using the `account_id`. For example:
///
/// ```sh
/// $ pulumi import aws:s3control/accessGrantsInstanceResourcePolicy:AccessGrantsInstanceResourcePolicy example 123456789012
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod access_grants_instance_resource_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AccessGrantsInstanceResourcePolicyArgs {
        #[builder(into, default)]
        pub account_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The policy document.
        #[builder(into)]
        pub policy: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct AccessGrantsInstanceResourcePolicyResult {
        pub account_id: pulumi_gestalt_rust::Output<String>,
        /// The policy document.
        pub policy: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: AccessGrantsInstanceResourcePolicyArgs,
    ) -> AccessGrantsInstanceResourcePolicyResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let account_id_binding = args.account_id.get_output(context);
        let policy_binding = args.policy.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:s3control/accessGrantsInstanceResourcePolicy:AccessGrantsInstanceResourcePolicy"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accountId".into(),
                    value: &account_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "policy".into(),
                    value: &policy_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        AccessGrantsInstanceResourcePolicyResult {
            account_id: o.get_field("accountId"),
            policy: o.get_field("policy"),
        }
    }
}
