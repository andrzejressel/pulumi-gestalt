/// Provides a resource to manage an [Amazon Macie Member](https://docs.aws.amazon.com/macie/latest/APIReference/members-id.html).
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = account::create("example", AccountArgs::builder().build_struct());
///     let exampleMember = member::create(
///         "exampleMember",
///         MemberArgs::builder()
///             .account_id("AWS ACCOUNT ID")
///             .email("EMAIL")
///             .invitation_disable_email_notification(true)
///             .invitation_message("Message of the invitation")
///             .invite(true)
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_macie2_member` using the account ID of the member account. For example:
///
/// ```sh
/// $ pulumi import aws:macie2/member:Member example 123456789012
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod member {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct MemberArgs {
        /// The AWS account ID for the account.
        #[builder(into)]
        pub account_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The email address for the account.
        #[builder(into)]
        pub email: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies whether to send an email notification to the root user of each account that the invitation will be sent to. This notification is in addition to an alert that the root user receives in AWS Personal Health Dashboard. To send an email notification to the root user of each account, set this value to `true`.
        #[builder(into, default)]
        pub invitation_disable_email_notification: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// A custom message to include in the invitation. Amazon Macie adds this message to the standard content that it sends for an invitation.
        #[builder(into, default)]
        pub invitation_message: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Send an invitation to a member
        #[builder(into, default)]
        pub invite: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Specifies the status for the account. To enable Amazon Macie and start all Macie activities for the account, set this value to `ENABLED`. Valid values are `ENABLED` or `PAUSED`.
        #[builder(into, default)]
        pub status: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A map of key-value pairs that specifies the tags to associate with the account in Amazon Macie.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct MemberResult {
        /// The AWS account ID for the account.
        pub account_id: pulumi_gestalt_rust::Output<String>,
        /// The AWS account ID for the administrator account.
        pub administrator_account_id: pulumi_gestalt_rust::Output<String>,
        /// The Amazon Resource Name (ARN) of the account.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The email address for the account.
        pub email: pulumi_gestalt_rust::Output<String>,
        /// Specifies whether to send an email notification to the root user of each account that the invitation will be sent to. This notification is in addition to an alert that the root user receives in AWS Personal Health Dashboard. To send an email notification to the root user of each account, set this value to `true`.
        pub invitation_disable_email_notification: pulumi_gestalt_rust::Output<
            Option<bool>,
        >,
        /// A custom message to include in the invitation. Amazon Macie adds this message to the standard content that it sends for an invitation.
        pub invitation_message: pulumi_gestalt_rust::Output<Option<String>>,
        /// Send an invitation to a member
        pub invite: pulumi_gestalt_rust::Output<bool>,
        /// The date and time, in UTC and extended RFC 3339 format, when an Amazon Macie membership invitation was last sent to the account. This value is null if a Macie invitation hasn't been sent to the account.
        pub invited_at: pulumi_gestalt_rust::Output<String>,
        pub master_account_id: pulumi_gestalt_rust::Output<String>,
        /// The current status of the relationship between the account and the administrator account.
        pub relationship_status: pulumi_gestalt_rust::Output<String>,
        /// Specifies the status for the account. To enable Amazon Macie and start all Macie activities for the account, set this value to `ENABLED`. Valid values are `ENABLED` or `PAUSED`.
        pub status: pulumi_gestalt_rust::Output<String>,
        /// A map of key-value pairs that specifies the tags to associate with the account in Amazon Macie.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The date and time, in UTC and extended RFC 3339 format, of the most recent change to the status of the relationship between the account and the administrator account.
        pub updated_at: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: MemberArgs,
    ) -> MemberResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let account_id_binding = args.account_id.get_output(context);
        let email_binding = args.email.get_output(context);
        let invitation_disable_email_notification_binding = args
            .invitation_disable_email_notification
            .get_output(context);
        let invitation_message_binding = args.invitation_message.get_output(context);
        let invite_binding = args.invite.get_output(context);
        let status_binding = args.status.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:macie2/member:Member".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accountId".into(),
                    value: &account_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "email".into(),
                    value: &email_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "invitationDisableEmailNotification".into(),
                    value: &invitation_disable_email_notification_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "invitationMessage".into(),
                    value: &invitation_message_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "invite".into(),
                    value: &invite_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "status".into(),
                    value: &status_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        MemberResult {
            account_id: o.get_field("accountId"),
            administrator_account_id: o.get_field("administratorAccountId"),
            arn: o.get_field("arn"),
            email: o.get_field("email"),
            invitation_disable_email_notification: o
                .get_field("invitationDisableEmailNotification"),
            invitation_message: o.get_field("invitationMessage"),
            invite: o.get_field("invite"),
            invited_at: o.get_field("invitedAt"),
            master_account_id: o.get_field("masterAccountId"),
            relationship_status: o.get_field("relationshipStatus"),
            status: o.get_field("status"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            updated_at: o.get_field("updatedAt"),
        }
    }
}
