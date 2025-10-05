/// Creates an Amazon Chime Voice Connector group under the administrator's AWS account. You can associate Amazon Chime Voice Connectors with the Amazon Chime Voice Connector group by including VoiceConnectorItems in the request.
///
/// You can include Amazon Chime Voice Connectors from different AWS Regions in your group. This creates a fault tolerant mechanism for fallback in case of availability events.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let group = voice_connector_group::create(
///         "group",
///         VoiceConnectorGroupArgs::builder()
///             .connectors(
///                 vec![
///                     VoiceConnectorGroupConnector::builder().priority(1)
///                     .voiceConnectorId("${vc1.id}").build_struct(),
///                     VoiceConnectorGroupConnector::builder().priority(3)
///                     .voiceConnectorId("${vc2.id}").build_struct(),
///                 ],
///             )
///             .name("test-group")
///             .build_struct(),
///     );
///     let vc1 = voice_connector::create(
///         "vc1",
///         VoiceConnectorArgs::builder()
///             .aws_region("us-east-1")
///             .name("connector-test-1")
///             .require_encryption(true)
///             .build_struct(),
///     );
///     let vc2 = voice_connector::create(
///         "vc2",
///         VoiceConnectorArgs::builder()
///             .aws_region("us-west-2")
///             .name("connector-test-2")
///             .require_encryption(true)
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Configuration Recorder using the name. For example:
///
/// ```sh
/// $ pulumi import aws:chime/voiceConnectorGroup:VoiceConnectorGroup default example
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod voice_connector_group {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VoiceConnectorGroupArgs {
        /// The Amazon Chime Voice Connectors to route inbound calls to.
        #[builder(into, default)]
        pub connectors: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::chime::VoiceConnectorGroupConnector>>,
        >,
        /// The name of the Amazon Chime Voice Connector group.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct VoiceConnectorGroupResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The Amazon Chime Voice Connectors to route inbound calls to.
        pub connectors: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::chime::VoiceConnectorGroupConnector>>,
        >,
        /// The name of the Amazon Chime Voice Connector group.
        pub name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: VoiceConnectorGroupArgs,
    ) -> VoiceConnectorGroupResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let connectors_binding = args.connectors.get_output(context);
        let name_binding = args.name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:chime/voiceConnectorGroup:VoiceConnectorGroup".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "connectors".into(),
                    value: &connectors_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        VoiceConnectorGroupResult {
            id: o.get_field("id"),
            connectors: o.get_field("connectors"),
            name: o.get_field("name"),
        }
    }
}
