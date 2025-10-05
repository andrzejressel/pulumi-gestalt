/// Provides a WAF Regional Regex Match Set Resource
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = regex_match_set::create(
///         "example",
///         RegexMatchSetArgs::builder()
///             .name("example")
///             .regex_match_tuples(
///                 vec![
///                     RegexMatchSetRegexMatchTuple::builder()
///                     .fieldToMatch(RegexMatchSetRegexMatchTupleFieldToMatch::builder()
///                     .data("User-Agent"). type ("HEADER").build_struct())
///                     .regexPatternSetId("${exampleRegexPatternSet.id}")
///                     .textTransformation("NONE").build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
///     let exampleRegexPatternSet = regex_pattern_set::create(
///         "exampleRegexPatternSet",
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
/// Using `pulumi import`, import WAF Regional Regex Match Set using the id. For example:
///
/// ```sh
/// $ pulumi import aws:wafregional/regexMatchSet:RegexMatchSet example a1b2c3d4-d5f6-7777-8888-9999aaaabbbbcccc
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod regex_match_set {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RegexMatchSetArgs {
        /// The name or description of the Regex Match Set.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The regular expression pattern that you want AWS WAF to search for in web requests, the location in requests that you want AWS WAF to search, and other settings. See below.
        #[builder(into, default)]
        pub regex_match_tuples: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::wafregional::RegexMatchSetRegexMatchTuple>>,
        >,
    }
    #[allow(dead_code)]
    pub struct RegexMatchSetResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The name or description of the Regex Match Set.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The regular expression pattern that you want AWS WAF to search for in web requests, the location in requests that you want AWS WAF to search, and other settings. See below.
        pub regex_match_tuples: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::wafregional::RegexMatchSetRegexMatchTuple>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: RegexMatchSetArgs,
    ) -> RegexMatchSetResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let regex_match_tuples_binding = args.regex_match_tuples.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:wafregional/regexMatchSet:RegexMatchSet".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "regexMatchTuples".into(),
                    value: &regex_match_tuples_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        RegexMatchSetResult {
            id: o.get_field("id"),
            name: o.get_field("name"),
            regex_match_tuples: o.get_field("regexMatchTuples"),
        }
    }
}
