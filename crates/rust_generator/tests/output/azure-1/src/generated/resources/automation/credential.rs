/// Manages a Automation Credential.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resource_group::create(
///         "example",
///         ResourceGroupArgs::builder()
///             .location("West Europe")
///             .name("example-resources")
///             .build_struct(),
///     );
///     let exampleAccount = account::create(
///         "exampleAccount",
///         AccountArgs::builder()
///             .location("${example.location}")
///             .name("account1")
///             .resource_group_name("${example.name}")
///             .sku_name("Basic")
///             .build_struct(),
///     );
///     let exampleCredential = credential::create(
///         "exampleCredential",
///         CredentialArgs::builder()
///             .automation_account_name("${exampleAccount.name}")
///             .description("This is an example credential")
///             .name("credential1")
///             .password("example_pwd")
///             .resource_group_name("${example.name}")
///             .username("example_user")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Automation Credentials can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:automation/credential:Credential credential1 /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Automation/automationAccounts/account1/credentials/credential1
/// ```
///
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod credential {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CredentialArgs {
        /// The name of the automation account in which the Credential is created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub automation_account_name: pulumi_gestalt_rust::Input<String>,
        /// The description associated with this Automation Credential.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::Input<Option<String>>,
        /// Specifies the name of the Credential. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::Input<Option<String>>,
        /// The password associated with this Automation Credential.
        #[builder(into)]
        pub password: pulumi_gestalt_rust::Input<String>,
        /// The name of the resource group in which the Credential is created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::Input<String>,
        /// The username associated with this Automation Credential.
        #[builder(into)]
        pub username: pulumi_gestalt_rust::Input<String>,
    }
    #[allow(dead_code)]
    pub struct CredentialResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// The name of the automation account in which the Credential is created. Changing this forces a new resource to be created.
        pub automation_account_name: pulumi_gestalt_rust::Output<String>,
        /// The description associated with this Automation Credential.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the name of the Credential. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The password associated with this Automation Credential.
        pub password: pulumi_gestalt_rust::Output<String>,
        /// The name of the resource group in which the Credential is created. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The username associated with this Automation Credential.
        pub username: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: CredentialArgs,
    ) -> CredentialResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: CredentialArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> CredentialResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: CredentialArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> CredentialResult {
        let automation_account_name_binding = args
            .automation_account_name
            .get_output(ctx);
        let description_binding = args.description.get_output(ctx);
        let name_binding = args.name.get_output(ctx);
        let password_binding = args.password.get_output(ctx);
        let resource_group_name_binding = args.resource_group_name.get_output(ctx);
        let username_binding = args.username.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:automation/credential:Credential".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "automationAccountName".into(),
                    value: &automation_account_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "password".into(),
                    value: &password_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "username".into(),
                    value: &username_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        CredentialResult {
            id: o.get_id(),
            urn: o.get_urn(),
            automation_account_name: o.get_field("automationAccountName"),
            description: o.get_field("description"),
            name: o.get_field("name"),
            password: o.get_field("password"),
            resource_group_name: o.get_field("resourceGroupName"),
            username: o.get_field("username"),
        }
    }
}
