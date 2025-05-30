/// For more information, see:
/// * [Get started with Firebase Security Rules](https://firebase.google.com/docs/rules/get-started)
/// ## Example Usage
///
/// ### Firestore_release
/// Creates a Firebase Rules Release to the default Cloud Firestore instance
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let firestore = ruleset::create(
///         "firestore",
///         RulesetArgs::builder()
///             .project("my-project-name")
///             .source(
///                 RulesetSource::builder()
///                     .files(
///                         vec![
///                             RulesetSourceFile::builder()
///                             .content("service cloud.firestore {match /databases/{database}/documents { match /{document=**} { allow read, write: if false; } } }")
///                             .name("firestore.rules").build_struct(),
///                         ],
///                     )
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
///     let primary = release::create(
///         "primary",
///         ReleaseArgs::builder()
///             .name("cloud.firestore")
///             .project("my-project-name")
///             .ruleset_name("projects/my-project-name/rulesets/${firestore.name}")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Firestore_release_additional
/// Creates a Firebase Rules Release to an additional Cloud Firestore instance
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let firestore = ruleset::create(
///         "firestore",
///         RulesetArgs::builder()
///             .project("my-project-name")
///             .source(
///                 RulesetSource::builder()
///                     .files(
///                         vec![
///                             RulesetSourceFile::builder()
///                             .content("service cloud.firestore {match /databases/{database}/documents { match /{document=**} { allow read, write: if false; } } }")
///                             .name("firestore.rules").build_struct(),
///                         ],
///                     )
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
///     let primary = release::create(
///         "primary",
///         ReleaseArgs::builder()
///             .name("cloud.firestore/database")
///             .project("my-project-name")
///             .ruleset_name("projects/my-project-name/rulesets/${firestore.name}")
///             .build_struct(),
///     );
/// }
/// ```
/// ## Import
///
/// Release can be imported using any of these accepted formats:
/// * `projects/{{project}}/releases/{{name}}`
///
/// When using the `pulumi import` command, Release can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:firebaserules/release:Release default projects/{{project}}/releases/{{name}}
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod release {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ReleaseArgs {
        /// Format: `projects/{project_id}/releases/{release_id}`\Firestore Rules Releases will **always** have the name 'cloud.firestore'
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The project for the resource
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Name of the `Ruleset` referred to by this `Release`. The `Ruleset` must exist for the `Release` to be created.
        ///
        ///
        ///
        /// - - -
        #[builder(into)]
        pub ruleset_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ReleaseResult {
        /// Output only. Time the release was created.
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// Disable the release to keep it from being served. The response code of NOT_FOUND will be given for executables generated from this Release.
        pub disabled: pulumi_gestalt_rust::Output<bool>,
        /// Format: `projects/{project_id}/releases/{release_id}`\Firestore Rules Releases will **always** have the name 'cloud.firestore'
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The project for the resource
        pub project: pulumi_gestalt_rust::Output<String>,
        /// Name of the `Ruleset` referred to by this `Release`. The `Ruleset` must exist for the `Release` to be created.
        ///
        ///
        ///
        /// - - -
        pub ruleset_name: pulumi_gestalt_rust::Output<String>,
        /// Output only. Time the release was updated.
        pub update_time: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ReleaseArgs,
    ) -> ReleaseResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let project_binding = args.project.get_output(context);
        let ruleset_name_binding = args.ruleset_name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:firebaserules/release:Release".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: &project_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "rulesetName".into(),
                    value: &ruleset_name_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        ReleaseResult {
            create_time: o.get_field("createTime"),
            disabled: o.get_field("disabled"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            ruleset_name: o.get_field("rulesetName"),
            update_time: o.get_field("updateTime"),
        }
    }
}
