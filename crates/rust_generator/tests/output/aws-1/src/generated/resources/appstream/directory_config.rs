/// Provides an AppStream Directory Config.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = directory_config::create(
///         "example",
///         DirectoryConfigArgs::builder()
///             .directory_name("NAME OF DIRECTORY")
///             .organizational_unit_distinguished_names(vec!["DISTINGUISHED NAME",])
///             .service_account_credentials(
///                 DirectoryConfigServiceAccountCredentials::builder()
///                     .accountName("NAME OF ACCOUNT")
///                     .accountPassword("PASSWORD OF ACCOUNT")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_appstream_directory_config` using the id. For example:
///
/// ```sh
/// $ pulumi import aws:appstream/directoryConfig:DirectoryConfig example directoryNameExample
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod directory_config {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DirectoryConfigArgs {
        /// Fully qualified name of the directory.
        #[builder(into)]
        pub directory_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Distinguished names of the organizational units for computer accounts.
        #[builder(into)]
        pub organizational_unit_distinguished_names: pulumi_gestalt_rust::InputOrOutput<
            Vec<String>,
        >,
        /// Configuration block for the name of the directory and organizational unit (OU) to use to join the directory config to a Microsoft Active Directory domain. See `service_account_credentials` below.
        #[builder(into)]
        pub service_account_credentials: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::appstream::DirectoryConfigServiceAccountCredentials,
        >,
    }
    #[allow(dead_code)]
    pub struct DirectoryConfigResult {
        /// Date and time, in UTC and extended RFC 3339 format, when the directory config was created.
        pub created_time: pulumi_gestalt_rust::Output<String>,
        /// Fully qualified name of the directory.
        pub directory_name: pulumi_gestalt_rust::Output<String>,
        /// Distinguished names of the organizational units for computer accounts.
        pub organizational_unit_distinguished_names: pulumi_gestalt_rust::Output<
            Vec<String>,
        >,
        /// Configuration block for the name of the directory and organizational unit (OU) to use to join the directory config to a Microsoft Active Directory domain. See `service_account_credentials` below.
        pub service_account_credentials: pulumi_gestalt_rust::Output<
            super::super::types::appstream::DirectoryConfigServiceAccountCredentials,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: DirectoryConfigArgs,
    ) -> DirectoryConfigResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let directory_name_binding = args.directory_name.get_output(context);
        let organizational_unit_distinguished_names_binding = args
            .organizational_unit_distinguished_names
            .get_output(context);
        let service_account_credentials_binding = args
            .service_account_credentials
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:appstream/directoryConfig:DirectoryConfig".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "directoryName".into(),
                    value: &directory_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "organizationalUnitDistinguishedNames".into(),
                    value: &organizational_unit_distinguished_names_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "serviceAccountCredentials".into(),
                    value: &service_account_credentials_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        DirectoryConfigResult {
            created_time: o.get_field("createdTime"),
            directory_name: o.get_field("directoryName"),
            organizational_unit_distinguished_names: o
                .get_field("organizationalUnitDistinguishedNames"),
            service_account_credentials: o.get_field("serviceAccountCredentials"),
        }
    }
}
