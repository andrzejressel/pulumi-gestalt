/// Provides an IAM Service Specific Credential.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = user::create(
///         "example",
///         UserArgs::builder().name("example").build_struct(),
///     );
///     let exampleServiceSpecificCredential = service_specific_credential::create(
///         "exampleServiceSpecificCredential",
///         ServiceSpecificCredentialArgs::builder()
///             .service_name("codecommit.amazonaws.com")
///             .user_name("${example.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import IAM Service Specific Credentials using the `service_name:user_name:service_specific_credential_id`. For example:
///
/// ```sh
/// $ pulumi import aws:iam/serviceSpecificCredential:ServiceSpecificCredential default `codecommit.amazonaws.com:example:some-id`
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod service_specific_credential {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ServiceSpecificCredentialArgs {
        /// The name of the AWS service that is to be associated with the credentials. The service you specify here is the only service that can be accessed using these credentials.
        #[builder(into)]
        pub service_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The status to be assigned to the service-specific credential. Valid values are `Active` and `Inactive`. Default value is `Active`.
        #[builder(into, default)]
        pub status: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the IAM user that is to be associated with the credentials. The new service-specific credentials have the same permissions as the associated user except that they can be used only to access the specified service.
        #[builder(into)]
        pub user_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ServiceSpecificCredentialResult {
        /// The name of the AWS service that is to be associated with the credentials. The service you specify here is the only service that can be accessed using these credentials.
        pub service_name: pulumi_gestalt_rust::Output<String>,
        /// The generated password for the service-specific credential.
        pub service_password: pulumi_gestalt_rust::Output<String>,
        /// The unique identifier for the service-specific credential.
        pub service_specific_credential_id: pulumi_gestalt_rust::Output<String>,
        /// The generated user name for the service-specific credential. This value is generated by combining the IAM user's name combined with the ID number of the AWS account, as in `jane-at-123456789012`, for example.
        pub service_user_name: pulumi_gestalt_rust::Output<String>,
        /// The status to be assigned to the service-specific credential. Valid values are `Active` and `Inactive`. Default value is `Active`.
        pub status: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the IAM user that is to be associated with the credentials. The new service-specific credentials have the same permissions as the associated user except that they can be used only to access the specified service.
        pub user_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ServiceSpecificCredentialArgs,
    ) -> ServiceSpecificCredentialResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let service_name_binding = args.service_name.get_output(context);
        let status_binding = args.status.get_output(context);
        let user_name_binding = args.user_name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:iam/serviceSpecificCredential:ServiceSpecificCredential".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "serviceName".into(),
                    value: &service_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "status".into(),
                    value: &status_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "userName".into(),
                    value: &user_name_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        ServiceSpecificCredentialResult {
            service_name: o.get_field("serviceName"),
            service_password: o.get_field("servicePassword"),
            service_specific_credential_id: o.get_field("serviceSpecificCredentialId"),
            service_user_name: o.get_field("serviceUserName"),
            status: o.get_field("status"),
            user_name: o.get_field("userName"),
        }
    }
}
