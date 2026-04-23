/// Provides a resource to manage an [Amazon Macie Invitation Accepter](https://docs.aws.amazon.com/macie/latest/APIReference/invitations-accept.html).
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let member = account::create("member", AccountArgs::builder().build_struct());
///     let memberInvitationAccepter = invitation_accepter::create(
///         "memberInvitationAccepter",
///         InvitationAccepterArgs::builder()
///             .administrator_account_id("ADMINISTRATOR ACCOUNT ID")
///             .build_struct(),
///     );
///     let primary = account::create("primary", AccountArgs::builder().build_struct());
///     let primaryMember = member::create(
///         "primaryMember",
///         MemberArgs::builder()
///             .account_id("ACCOUNT ID")
///             .email("EMAIL")
///             .invitation_message("Message of the invite")
///             .invite(true)
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_macie2_invitation_accepter` using the admin account ID. For example:
///
/// ```sh
/// $ pulumi import aws:macie2/invitationAccepter:InvitationAccepter example 123456789012
/// ```
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod invitation_accepter {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct InvitationAccepterArgs {
        /// The AWS account ID for the account that sent the invitation.
        #[builder(into)]
        pub administrator_account_id: pulumi_gestalt_rust::Input<String>,
    }
    #[allow(dead_code)]
    pub struct InvitationAccepterResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// The AWS account ID for the account that sent the invitation.
        pub administrator_account_id: pulumi_gestalt_rust::Output<String>,
        /// The unique identifier for the invitation.
        pub invitation_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: InvitationAccepterArgs,
    ) -> InvitationAccepterResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: InvitationAccepterArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> InvitationAccepterResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: InvitationAccepterArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> InvitationAccepterResult {
        let administrator_account_id_binding = args
            .administrator_account_id
            .get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:macie2/invitationAccepter:InvitationAccepter".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "administratorAccountId".into(),
                    value: &administrator_account_id_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        InvitationAccepterResult {
            id: o.get_id(),
            urn: o.get_urn(),
            administrator_account_id: o.get_field("administratorAccountId"),
            invitation_id: o.get_field("invitationId"),
        }
    }
}
