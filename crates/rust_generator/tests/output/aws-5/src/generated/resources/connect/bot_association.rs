/// Allows the specified Amazon Connect instance to access the specified Amazon Lex (V1) bot. For more information see
/// [Amazon Connect: Getting Started](https://docs.aws.amazon.com/connect/latest/adminguide/amazon-connect-get-started.html) and [Add an Amazon Lex bot](https://docs.aws.amazon.com/connect/latest/adminguide/amazon-lex.html).
///
/// > **NOTE:** This resource only currently supports Amazon Lex (V1) Associations.
///
/// ## Example Usage
///
/// ### Basic
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = bot_association::create(
///         "example",
///         BotAssociationArgs::builder()
///             .instance_id("${exampleAwsConnectInstance.id}")
///             .lex_bot(
///                 BotAssociationLexBot::builder()
///                     .lexRegion("us-west-2")
///                     .name("Test")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Including a sample Lex bot
///
/// ```yaml
/// resources:
///   example:
///     type: aws:lex:Intent
///     properties:
///       createVersion: true
///       name: connect_lex_intent
///       fulfillmentActivity:
///         type: ReturnIntent
///       sampleUtterances:
///         - I would like to pick up flowers.
///   exampleBot:
///     type: aws:lex:Bot
///     name: example
///     properties:
///       abortStatement:
///         messages:
///           - content: Sorry, I am not able to assist at this time.
///             contentType: PlainText
///       clarificationPrompt:
///         maxAttempts: 2
///         messages:
///           - content: I didn't understand you, what would you like to do?
///             contentType: PlainText
///       intents:
///         - intentName: ${example.name}
///           intentVersion: '1'
///       childDirected: false
///       name: connect_lex_bot
///       processBehavior: BUILD
///   exampleBotAssociation:
///     type: aws:connect:BotAssociation
///     name: example
///     properties:
///       instanceId: ${exampleAwsConnectInstance.id}
///       lexBot:
///         lexRegion: ${current.name}
///         name: ${exampleBot.name}
/// variables:
///   current:
///     fn::invoke:
///       function: aws:getRegion
///       arguments: {}
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_connect_bot_association` using the Amazon Connect instance ID, Lex (V1) bot name, and Lex (V1) bot region separated by colons (`:`). For example:
///
/// ```sh
/// $ pulumi import aws:connect/botAssociation:BotAssociation example aaaaaaaa-bbbb-cccc-dddd-111111111111:Example:us-west-2
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod bot_association {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct BotAssociationArgs {
        /// The identifier of the Amazon Connect instance. You can find the instanceId in the ARN of the instance.
        #[builder(into)]
        pub instance_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Configuration information of an Amazon Lex (V1) bot. Detailed below.
        #[builder(into)]
        pub lex_bot: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::connect::BotAssociationLexBot,
        >,
    }
    #[allow(dead_code)]
    pub struct BotAssociationResult {
        /// The identifier of the Amazon Connect instance. You can find the instanceId in the ARN of the instance.
        pub instance_id: pulumi_gestalt_rust::Output<String>,
        /// Configuration information of an Amazon Lex (V1) bot. Detailed below.
        pub lex_bot: pulumi_gestalt_rust::Output<
            super::super::types::connect::BotAssociationLexBot,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: BotAssociationArgs,
    ) -> BotAssociationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let instance_id_binding = args.instance_id.get_output(context);
        let lex_bot_binding = args.lex_bot.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:connect/botAssociation:BotAssociation".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "instanceId".into(),
                    value: &instance_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "lexBot".into(),
                    value: &lex_bot_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        BotAssociationResult {
            instance_id: o.get_field("instanceId"),
            lex_bot: o.get_field("lexBot"),
        }
    }
}
