/// Manages the specified alternate contact attached to an AWS Account.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let operations = alternative_contact::create(
///         "operations",
///         AlternativeContactArgs::builder()
///             .alternate_contact_type("OPERATIONS")
///             .email_address("test@example.com")
///             .name("Example")
///             .phone_number("+1234567890")
///             .title("Example")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Import the Alternate Contact for another account using the `account_id` and `alternate_contact_type` separated by a forward slash (`/`):
///
/// __Using `pulumi import` to import__ the Alternate Contact for the current or another account using the `alternate_contact_type`. For example:
///
/// Import the Alternate Contact for the current account:
///
/// ```sh
/// $ pulumi import aws:account/alternativeContact:AlternativeContact operations OPERATIONS
/// ```
/// Import the Alternate Contact for another account using the `account_id` and `alternate_contact_type` separated by a forward slash (`/`):
///
/// ```sh
/// $ pulumi import aws:account/alternativeContact:AlternativeContact operations 1234567890/OPERATIONS
/// ```
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod alternative_contact {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AlternativeContactArgs {
        /// ID of the target account when managing member accounts. Will manage current user's account by default if omitted.
        #[builder(into, default)]
        pub account_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Type of the alternate contact. Allowed values are: `BILLING`, `OPERATIONS`, `SECURITY`.
        #[builder(into)]
        pub alternate_contact_type: pulumi_gestalt_rust::InputOrOutput<String>,
        /// An email address for the alternate contact.
        #[builder(into)]
        pub email_address: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Name of the alternate contact.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Phone number for the alternate contact.
        #[builder(into)]
        pub phone_number: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Title for the alternate contact.
        #[builder(into)]
        pub title: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct AlternativeContactResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// ID of the target account when managing member accounts. Will manage current user's account by default if omitted.
        pub account_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// Type of the alternate contact. Allowed values are: `BILLING`, `OPERATIONS`, `SECURITY`.
        pub alternate_contact_type: pulumi_gestalt_rust::Output<String>,
        /// An email address for the alternate contact.
        pub email_address: pulumi_gestalt_rust::Output<String>,
        /// Name of the alternate contact.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Phone number for the alternate contact.
        pub phone_number: pulumi_gestalt_rust::Output<String>,
        /// Title for the alternate contact.
        pub title: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: AlternativeContactArgs,
    ) -> AlternativeContactResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: AlternativeContactArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> AlternativeContactResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: AlternativeContactArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> AlternativeContactResult {
        let account_id_binding = args.account_id.get_output(ctx);
        let alternate_contact_type_binding = args.alternate_contact_type.get_output(ctx);
        let email_address_binding = args.email_address.get_output(ctx);
        let name_binding = args.name.get_output(ctx);
        let phone_number_binding = args.phone_number.get_output(ctx);
        let title_binding = args.title.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:account/alternativeContact:AlternativeContact".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accountId".into(),
                    value: &account_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "alternateContactType".into(),
                    value: &alternate_contact_type_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "emailAddress".into(),
                    value: &email_address_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "phoneNumber".into(),
                    value: &phone_number_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "title".into(),
                    value: &title_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        AlternativeContactResult {
            id: o.get_id(),
            urn: o.get_urn(),
            account_id: o.get_field("accountId"),
            alternate_contact_type: o.get_field("alternateContactType"),
            email_address: o.get_field("emailAddress"),
            name: o.get_field("name"),
            phone_number: o.get_field("phoneNumber"),
            title: o.get_field("title"),
        }
    }
}
