/// ## Example Usage
///
/// ### Gemini Code Repository Index Basic
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = code_repository_index::create(
///         "example",
///         CodeRepositoryIndexArgs::builder()
///             .code_repository_index_id("")
///             .kms_key(
///                 "projects/projectExample/locations/locationExample/keyRings/keyRingExample/cryptoKeys/cryptoKeyExample",
///             )
///             .location("us-central1")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// CodeRepositoryIndex can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/codeRepositoryIndexes/{{code_repository_index_id}}`
///
/// * `{{project}}/{{location}}/{{code_repository_index_id}}`
///
/// * `{{location}}/{{code_repository_index_id}}`
///
/// When using the `pulumi import` command, CodeRepositoryIndex can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:gemini/codeRepositoryIndex:CodeRepositoryIndex default projects/{{project}}/locations/{{location}}/codeRepositoryIndexes/{{code_repository_index_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:gemini/codeRepositoryIndex:CodeRepositoryIndex default {{project}}/{{location}}/{{code_repository_index_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:gemini/codeRepositoryIndex:CodeRepositoryIndex default {{location}}/{{code_repository_index_id}}
/// ```
///
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod code_repository_index {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CodeRepositoryIndexArgs {
        /// Required. Id of the Code Repository Index.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub code_repository_index_id: pulumi_gestalt_rust::Input<String>,
        /// Optional. Immutable. Customer-managed encryption key name, in the format
        /// projects/*/locations/*/keyRings/*/cryptoKeys/*.
        #[builder(into, default)]
        pub kms_key: pulumi_gestalt_rust::Input<Option<String>>,
        /// Optional. Labels as key value pairs.
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::Input<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The location of the Code Repository Index, for example `us-central1`.
        #[builder(into)]
        pub location: pulumi_gestalt_rust::Input<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::Input<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct CodeRepositoryIndexResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// Required. Id of the Code Repository Index.
        ///
        ///
        /// - - -
        pub code_repository_index_id: pulumi_gestalt_rust::Output<String>,
        /// Output only. Create time stamp.
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Optional. Immutable. Customer-managed encryption key name, in the format
        /// projects/*/locations/*/keyRings/*/cryptoKeys/*.
        pub kms_key: pulumi_gestalt_rust::Output<Option<String>>,
        /// Optional. Labels as key value pairs.
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The location of the Code Repository Index, for example `us-central1`.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Immutable. Identifier. Name of Code Repository Index.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Output only. Code Repository Index instance State.
        /// Possible values:
        /// STATE_UNSPECIFIED
        /// CREATING
        /// ACTIVE
        /// DELETING
        /// SUSPENDED
        pub state: pulumi_gestalt_rust::Output<String>,
        /// Output only. Update time stamp.
        pub update_time: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: CodeRepositoryIndexArgs,
    ) -> CodeRepositoryIndexResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: CodeRepositoryIndexArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> CodeRepositoryIndexResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: CodeRepositoryIndexArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> CodeRepositoryIndexResult {
        let code_repository_index_id_binding = args
            .code_repository_index_id
            .get_output(ctx);
        let kms_key_binding = args.kms_key.get_output(ctx);
        let labels_binding = args.labels.get_output(ctx);
        let location_binding = args.location.get_output(ctx);
        let project_binding = args.project.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:gemini/codeRepositoryIndex:CodeRepositoryIndex".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "codeRepositoryIndexId".into(),
                    value: &code_repository_index_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "kmsKey".into(),
                    value: &kms_key_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "labels".into(),
                    value: &labels_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: &location_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: &project_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        CodeRepositoryIndexResult {
            id: o.get_id(),
            urn: o.get_urn(),
            code_repository_index_id: o.get_field("codeRepositoryIndexId"),
            create_time: o.get_field("createTime"),
            effective_labels: o.get_field("effectiveLabels"),
            kms_key: o.get_field("kmsKey"),
            labels: o.get_field("labels"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            pulumi_labels: o.get_field("pulumiLabels"),
            state: o.get_field("state"),
            update_time: o.get_field("updateTime"),
        }
    }
}
