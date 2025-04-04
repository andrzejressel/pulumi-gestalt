/// The Apikeys Key resource
///
/// ## Example Usage
///
/// ### Android_key
/// A basic example of a android api keys key
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let primary = api_key::create(
///         "primary",
///         ApiKeyArgs::builder()
///             .display_name("sample-key")
///             .name("key")
///             .restrictions(
///                 ApiKeyRestrictions::builder()
///                     .androidKeyRestrictions(
///                         ApiKeyRestrictionsAndroidKeyRestrictions::builder()
///                             .allowedApplications(
///                                 vec![
///                                     ApiKeyRestrictionsAndroidKeyRestrictionsAllowedApplication::builder()
///                                     .packageName("com.example.app123")
///                                     .sha1Fingerprint("1699466a142d4682a5f91b50fdf400f2358e2b0b")
///                                     .build_struct(),
///                                 ],
///                             )
///                             .build_struct(),
///                     )
///                     .apiTargets(
///                         vec![
///                             ApiKeyRestrictionsApiTarget::builder().methods(vec!["GET*",])
///                             .service("translate.googleapis.com").build_struct(),
///                         ],
///                     )
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
/// ### Basic_key
/// A basic example of a api keys key
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let primary = api_key::create(
///         "primary",
///         ApiKeyArgs::builder()
///             .display_name("sample-key")
///             .name("key")
///             .restrictions(
///                 ApiKeyRestrictions::builder()
///                     .apiTargets(
///                         vec![
///                             ApiKeyRestrictionsApiTarget::builder().methods(vec!["GET*",])
///                             .service("translate.googleapis.com").build_struct(),
///                         ],
///                     )
///                     .browserKeyRestrictions(
///                         ApiKeyRestrictionsBrowserKeyRestrictions::builder()
///                             .allowedReferrers(vec![".*",])
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
/// ### Ios_key
/// A basic example of a ios api keys key
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let primary = api_key::create(
///         "primary",
///         ApiKeyArgs::builder()
///             .display_name("sample-key")
///             .name("key")
///             .restrictions(
///                 ApiKeyRestrictions::builder()
///                     .apiTargets(
///                         vec![
///                             ApiKeyRestrictionsApiTarget::builder().methods(vec!["GET*",])
///                             .service("translate.googleapis.com").build_struct(),
///                         ],
///                     )
///                     .iosKeyRestrictions(
///                         ApiKeyRestrictionsIosKeyRestrictions::builder()
///                             .allowedBundleIds(vec!["com.google.app.macos",])
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
/// ### Minimal_key
/// A minimal example of a api keys key
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let primary = api_key::create(
///         "primary",
///         ApiKeyArgs::builder().display_name("sample-key").name("key").build_struct(),
///     );
/// }
/// ```
/// ### Server_key
/// A basic example of a server api keys key
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let primary = api_key::create(
///         "primary",
///         ApiKeyArgs::builder()
///             .display_name("sample-key")
///             .name("key")
///             .restrictions(
///                 ApiKeyRestrictions::builder()
///                     .apiTargets(
///                         vec![
///                             ApiKeyRestrictionsApiTarget::builder().methods(vec!["GET*",])
///                             .service("translate.googleapis.com").build_struct(),
///                         ],
///                     )
///                     .serverKeyRestrictions(
///                         ApiKeyRestrictionsServerKeyRestrictions::builder()
///                             .allowedIps(vec!["127.0.0.1",])
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Key can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/global/keys/{{name}}`
///
/// * `{{project}}/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, Key can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:projects/apiKey:ApiKey default projects/{{project}}/locations/global/keys/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:projects/apiKey:ApiKey default {{project}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:projects/apiKey:ApiKey default {{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod api_key {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ApiKeyArgs {
        /// Human-readable display name of this API key. Modifiable by user.
        #[builder(into, default)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The resource name of the key. The name must be unique within the project, must conform with RFC-1034, is restricted to lower-cased letters, and has a maximum length of 63 characters. In another word, the name must match the regular expression: `a-z?`.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The project for the resource
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Key restrictions.
        #[builder(into, default)]
        pub restrictions: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::projects::ApiKeyRestrictions>,
        >,
    }
    #[allow(dead_code)]
    pub struct ApiKeyResult {
        /// Human-readable display name of this API key. Modifiable by user.
        pub display_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// Output only. An encrypted and signed value held by this key. This field can be accessed only through the `GetKeyString` method.
        pub key_string: pulumi_gestalt_rust::Output<String>,
        /// The resource name of the key. The name must be unique within the project, must conform with RFC-1034, is restricted to lower-cased letters, and has a maximum length of 63 characters. In another word, the name must match the regular expression: `a-z?`.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The project for the resource
        pub project: pulumi_gestalt_rust::Output<String>,
        /// Key restrictions.
        pub restrictions: pulumi_gestalt_rust::Output<
            Option<super::super::types::projects::ApiKeyRestrictions>,
        >,
        /// Output only. Unique id in UUID4 format.
        pub uid: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ApiKeyArgs,
    ) -> ApiKeyResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let display_name_binding = args.display_name.get_output(context);
        let name_binding = args.name.get_output(context);
        let project_binding = args.project.get_output(context);
        let restrictions_binding = args.restrictions.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:projects/apiKey:ApiKey".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: &project_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "restrictions".into(),
                    value: &restrictions_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        ApiKeyResult {
            display_name: o.get_field("displayName"),
            key_string: o.get_field("keyString"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            restrictions: o.get_field("restrictions"),
            uid: o.get_field("uid"),
        }
    }
}
