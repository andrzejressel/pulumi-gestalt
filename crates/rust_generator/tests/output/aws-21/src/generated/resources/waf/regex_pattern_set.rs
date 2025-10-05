/// Provides a WAF Regex Pattern Set Resource
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
///             .name("my_waf_regex_pattern_set")
///             .regex_pattern_strings(vec!["one", "two",])
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import AWS WAF Regex Pattern Set using their ID. For example:
///
/// ```sh
/// $ pulumi import aws:waf/regexPatternSet:RegexPatternSet example a1b2c3d4-d5f6-7777-8888-9999aaaabbbbcccc
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod regex_pattern_set {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RegexPatternSetArgs {
        /// The name or description of the Regex Pattern Set.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A list of regular expression (regex) patterns that you want AWS WAF to search for, such as `B[a@]dB[o0]t`.
        #[builder(into, default)]
        pub regex_pattern_strings: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct RegexPatternSetResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Amazon Resource Name (ARN)
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The name or description of the Regex Pattern Set.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A list of regular expression (regex) patterns that you want AWS WAF to search for, such as `B[a@]dB[o0]t`.
        pub regex_pattern_strings: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: RegexPatternSetArgs,
    ) -> RegexPatternSetResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let regex_pattern_strings_binding = args
            .regex_pattern_strings
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:waf/regexPatternSet:RegexPatternSet".into(),
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
        };
        let o = context.register_resource(request);
        RegexPatternSetResult {
            id: o.get_field("id"),
            arn: o.get_field("arn"),
            name: o.get_field("name"),
            regex_pattern_strings: o.get_field("regexPatternStrings"),
        }
    }
}
