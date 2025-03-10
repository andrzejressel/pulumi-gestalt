/// Agents are best described as Natural Language Understanding (NLU) modules that transform user requests into actionable data. You can include agents in your app, product, or service to determine user intent and respond to the user in a natural way.
///
///
/// To get more information about Agent, see:
///
/// * [API documentation](https://cloud.google.com/dialogflow/cx/docs/reference/rest/v3/projects.locations.agents)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/dialogflow/cx/docs)
///
///
///
/// ## Example Usage
///
/// ### Dialogflowcx Agent Full
///
///
/// ```yaml
/// resources:
///   bucket:
///     type: gcp:storage:Bucket
///     properties:
///       name: dialogflowcx-bucket
///       location: US
///       uniformBucketLevelAccess: true
///   fullAgent:
///     type: gcp:diagflow:CxAgent
///     name: full_agent
///     properties:
///       displayName: dialogflowcx-agent
///       location: global
///       defaultLanguageCode: en
///       supportedLanguageCodes:
///         - fr
///         - de
///         - es
///       timeZone: America/New_York
///       description: Example description.
///       avatarUri: https://cloud.google.com/_static/images/cloud/icons/favicons/onecloud/super_cloud.png
///       enableStackdriverLogging: true
///       enableSpellCorrection: true
///       speechToTextSettings:
///         enableSpeechAdaptation: true
///       advancedSettings:
///         audioExportGcsDestination:
///           uri: ${bucket.url}/prefix-
///         speechSettings:
///           endpointerSensitivity: 30
///           noSpeechTimeout: 3.500s
///           useTimeoutBasedEndpointing: true
///           models:
///             name: wrench
///             mass: 1.3kg
///             count: '3'
///         dtmfSettings:
///           enabled: true
///           maxDigits: 1
///           finishDigit: '#'
///         loggingSettings:
///           enableStackdriverLogging: true
///           enableInteractionLogging: true
///           enableConsentBasedRedaction: true
///       gitIntegrationSettings:
///         githubSettings:
///           displayName: Github Repo
///           repositoryUri: https://api.github.com/repos/githubtraining/hellogitworld
///           trackingBranch: main
///           accessToken: secret-token
///           branches:
///             - main
///       textToSpeechSettings:
///         synthesizeSpeechConfigs:
///           fn::toJSON:
///             en:
///               voice:
///                 name: en-US-Neural2-A
///             fr:
///               voice:
///                 name: fr-CA-Neural2-A
/// ```
///
/// ## Import
///
/// Agent can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/agents/{{name}}`
///
/// * `{{project}}/{{location}}/{{name}}`
///
/// * `{{location}}/{{name}}`
///
/// When using the `pulumi import` command, Agent can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:diagflow/cxAgent:CxAgent default projects/{{project}}/locations/{{location}}/agents/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:diagflow/cxAgent:CxAgent default {{project}}/{{location}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:diagflow/cxAgent:CxAgent default {{location}}/{{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod cx_agent {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CxAgentArgs {
        /// Hierarchical advanced settings for this agent. The settings exposed at the lower level overrides the settings exposed at the higher level.
        /// Hierarchy: Agent->Flow->Page->Fulfillment/Parameter.
        /// Structure is documented below.
        #[builder(into, default)]
        pub advanced_settings: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::diagflow::CxAgentAdvancedSettings>,
        >,
        /// The URI of the agent's avatar. Avatars are used throughout the Dialogflow console and in the self-hosted Web Demo integration.
        #[builder(into, default)]
        pub avatar_uri: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The default language of the agent as a language tag. [See Language Support](https://cloud.google.com/dialogflow/cx/docs/reference/language)
        /// for a list of the currently supported language codes. This field cannot be updated after creation.
        #[builder(into)]
        pub default_language_code: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The description of this agent. The maximum length is 500 characters. If exceeded, the request is rejected.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The human-readable name of the agent, unique within the location.
        #[builder(into)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Indicates if automatic spell correction is enabled in detect intent requests.
        #[builder(into, default)]
        pub enable_spell_correction: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// (Optional, Deprecated)
        /// Determines whether this agent should log conversation queries.
        ///
        /// > **Warning:** `enable_stackdriver_logging` is deprecated and will be removed in a future major release. Please use `advanced_settings.logging_settings.enable_stackdriver_logging`instead.
        #[builder(into, default)]
        pub enable_stackdriver_logging: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Git integration settings for this agent.
        /// Structure is documented below.
        #[builder(into, default)]
        pub git_integration_settings: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::diagflow::CxAgentGitIntegrationSettings>,
        >,
        /// The name of the location this agent is located in.
        /// > **Note:** The first time you are deploying an Agent in your project you must configure location settings.
        /// This is a one time step but at the moment you can only [configure location settings](https://cloud.google.com/dialogflow/cx/docs/concept/region#location-settings) via the Dialogflow CX console.
        /// Another options is to use global location so you don't need to manually configure location settings.
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Name of the SecuritySettings reference for the agent. Format: projects/<Project ID>/locations/<Location ID>/securitySettings/<Security Settings ID>.
        #[builder(into, default)]
        pub security_settings: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Settings related to speech recognition.
        /// Structure is documented below.
        #[builder(into, default)]
        pub speech_to_text_settings: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::diagflow::CxAgentSpeechToTextSettings>,
        >,
        /// The list of all languages supported by this agent (except for the default_language_code).
        #[builder(into, default)]
        pub supported_language_codes: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// Settings related to speech synthesizing.
        /// Structure is documented below.
        #[builder(into, default)]
        pub text_to_speech_settings: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::diagflow::CxAgentTextToSpeechSettings>,
        >,
        /// The time zone of this agent from the [time zone database](https://www.iana.org/time-zones), e.g., America/New_York,
        /// Europe/Paris.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub time_zone: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct CxAgentResult {
        /// Hierarchical advanced settings for this agent. The settings exposed at the lower level overrides the settings exposed at the higher level.
        /// Hierarchy: Agent->Flow->Page->Fulfillment/Parameter.
        /// Structure is documented below.
        pub advanced_settings: pulumi_gestalt_rust::Output<
            super::super::types::diagflow::CxAgentAdvancedSettings,
        >,
        /// The URI of the agent's avatar. Avatars are used throughout the Dialogflow console and in the self-hosted Web Demo integration.
        pub avatar_uri: pulumi_gestalt_rust::Output<Option<String>>,
        /// The default language of the agent as a language tag. [See Language Support](https://cloud.google.com/dialogflow/cx/docs/reference/language)
        /// for a list of the currently supported language codes. This field cannot be updated after creation.
        pub default_language_code: pulumi_gestalt_rust::Output<String>,
        /// The description of this agent. The maximum length is 500 characters. If exceeded, the request is rejected.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The human-readable name of the agent, unique within the location.
        pub display_name: pulumi_gestalt_rust::Output<String>,
        /// Indicates if automatic spell correction is enabled in detect intent requests.
        pub enable_spell_correction: pulumi_gestalt_rust::Output<Option<bool>>,
        /// (Optional, Deprecated)
        /// Determines whether this agent should log conversation queries.
        ///
        /// > **Warning:** `enable_stackdriver_logging` is deprecated and will be removed in a future major release. Please use `advanced_settings.logging_settings.enable_stackdriver_logging`instead.
        pub enable_stackdriver_logging: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Git integration settings for this agent.
        /// Structure is documented below.
        pub git_integration_settings: pulumi_gestalt_rust::Output<
            Option<super::super::types::diagflow::CxAgentGitIntegrationSettings>,
        >,
        /// The name of the location this agent is located in.
        /// > **Note:** The first time you are deploying an Agent in your project you must configure location settings.
        /// This is a one time step but at the moment you can only [configure location settings](https://cloud.google.com/dialogflow/cx/docs/concept/region#location-settings) via the Dialogflow CX console.
        /// Another options is to use global location so you don't need to manually configure location settings.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The unique identifier of the agent.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// Name of the SecuritySettings reference for the agent. Format: projects/<Project ID>/locations/<Location ID>/securitySettings/<Security Settings ID>.
        pub security_settings: pulumi_gestalt_rust::Output<Option<String>>,
        /// Settings related to speech recognition.
        /// Structure is documented below.
        pub speech_to_text_settings: pulumi_gestalt_rust::Output<
            Option<super::super::types::diagflow::CxAgentSpeechToTextSettings>,
        >,
        /// Name of the start flow in this agent. A start flow will be automatically created when the agent is created, and can only be deleted by deleting the agent. Format: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/flows/<Flow ID>.
        pub start_flow: pulumi_gestalt_rust::Output<String>,
        /// The list of all languages supported by this agent (except for the default_language_code).
        pub supported_language_codes: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// Settings related to speech synthesizing.
        /// Structure is documented below.
        pub text_to_speech_settings: pulumi_gestalt_rust::Output<
            Option<super::super::types::diagflow::CxAgentTextToSpeechSettings>,
        >,
        /// The time zone of this agent from the [time zone database](https://www.iana.org/time-zones), e.g., America/New_York,
        /// Europe/Paris.
        ///
        ///
        /// - - -
        pub time_zone: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: CxAgentArgs,
    ) -> CxAgentResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let advanced_settings_binding = args.advanced_settings.get_output(context);
        let avatar_uri_binding = args.avatar_uri.get_output(context);
        let default_language_code_binding = args
            .default_language_code
            .get_output(context);
        let description_binding = args.description.get_output(context);
        let display_name_binding = args.display_name.get_output(context);
        let enable_spell_correction_binding = args
            .enable_spell_correction
            .get_output(context);
        let enable_stackdriver_logging_binding = args
            .enable_stackdriver_logging
            .get_output(context);
        let git_integration_settings_binding = args
            .git_integration_settings
            .get_output(context);
        let location_binding = args.location.get_output(context);
        let project_binding = args.project.get_output(context);
        let security_settings_binding = args.security_settings.get_output(context);
        let speech_to_text_settings_binding = args
            .speech_to_text_settings
            .get_output(context);
        let supported_language_codes_binding = args
            .supported_language_codes
            .get_output(context);
        let text_to_speech_settings_binding = args
            .text_to_speech_settings
            .get_output(context);
        let time_zone_binding = args.time_zone.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:diagflow/cxAgent:CxAgent".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "advancedSettings".into(),
                    value: &advanced_settings_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "avatarUri".into(),
                    value: &avatar_uri_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "defaultLanguageCode".into(),
                    value: &default_language_code_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enableSpellCorrection".into(),
                    value: &enable_spell_correction_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enableStackdriverLogging".into(),
                    value: &enable_stackdriver_logging_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "gitIntegrationSettings".into(),
                    value: &git_integration_settings_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: &location_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: &project_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "securitySettings".into(),
                    value: &security_settings_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "speechToTextSettings".into(),
                    value: &speech_to_text_settings_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "supportedLanguageCodes".into(),
                    value: &supported_language_codes_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "textToSpeechSettings".into(),
                    value: &text_to_speech_settings_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "timeZone".into(),
                    value: &time_zone_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        CxAgentResult {
            advanced_settings: o.get_field("advancedSettings"),
            avatar_uri: o.get_field("avatarUri"),
            default_language_code: o.get_field("defaultLanguageCode"),
            description: o.get_field("description"),
            display_name: o.get_field("displayName"),
            enable_spell_correction: o.get_field("enableSpellCorrection"),
            enable_stackdriver_logging: o.get_field("enableStackdriverLogging"),
            git_integration_settings: o.get_field("gitIntegrationSettings"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            security_settings: o.get_field("securitySettings"),
            speech_to_text_settings: o.get_field("speechToTextSettings"),
            start_flow: o.get_field("startFlow"),
            supported_language_codes: o.get_field("supportedLanguageCodes"),
            text_to_speech_settings: o.get_field("textToSpeechSettings"),
            time_zone: o.get_field("timeZone"),
        }
    }
}
