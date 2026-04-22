/// Provides a WAF Regional Regex Pattern Set Resource
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = regex_pattern_set::create(
///         "example",
///         RegexPatternSetArgs::builder()
///             .name("example")
///             .regex_pattern_strings(vec!["one", "two",])
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import WAF Regional Regex Pattern Set using the id. For example:
///
/// ```sh
/// $ pulumi import aws:wafregional/regexPatternSet:RegexPatternSet example a1b2c3d4-d5f6-7777-8888-9999aaaabbbbcccc
/// ```
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod regex_pattern_set {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RegexPatternSetArgs {
        /// The name or description of the Regex Pattern Set.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::Input<Option<String>>,
        /// A list of regular expression (regex) patterns that you want AWS WAF to search for, such as `B[a@]dB[o0]t`.
        #[builder(into, default)]
        pub regex_pattern_strings: pulumi_gestalt_rust::Input<
            Option<Vec<String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct RegexPatternSetResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// The name or description of the Regex Pattern Set.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A list of regular expression (regex) patterns that you want AWS WAF to search for, such as `B[a@]dB[o0]t`.
        pub regex_pattern_strings: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: RegexPatternSetArgs,
    ) -> RegexPatternSetResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: RegexPatternSetArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> RegexPatternSetResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: RegexPatternSetArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> RegexPatternSetResult {
        let name_binding = args.name.get_output(ctx);
        let regex_pattern_strings_binding = args.regex_pattern_strings.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:wafregional/regexPatternSet:RegexPatternSet".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "regexPatternStrings".into(),
                    value: &regex_pattern_strings_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        RegexPatternSetResult {
            id: o.get_id(),
            urn: o.get_urn(),
            name: o.get_field("name"),
            regex_pattern_strings: o.get_field("regexPatternStrings"),
        }
    }
}
