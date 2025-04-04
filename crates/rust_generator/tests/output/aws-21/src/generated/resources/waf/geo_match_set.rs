/// Provides a WAF Geo Match Set Resource
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let geoMatchSet = geo_match_set::create(
///         "geoMatchSet",
///         GeoMatchSetArgs::builder()
///             .geo_match_constraints(
///                 vec![
///                     GeoMatchSetGeoMatchConstraint::builder(). type ("Country")
///                     .value("US").build_struct(), GeoMatchSetGeoMatchConstraint::builder()
///                     . type ("Country").value("CA").build_struct(),
///                 ],
///             )
///             .name("geo_match_set")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import WAF Geo Match Set using their ID. For example:
///
/// ```sh
/// $ pulumi import aws:waf/geoMatchSet:GeoMatchSet example a1b2c3d4-d5f6-7777-8888-9999aaaabbbbcccc
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod geo_match_set {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GeoMatchSetArgs {
        /// The GeoMatchConstraint objects which contain the country that you want AWS WAF to search for.
        #[builder(into, default)]
        pub geo_match_constraints: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::waf::GeoMatchSetGeoMatchConstraint>>,
        >,
        /// The name or description of the GeoMatchSet.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GeoMatchSetResult {
        /// Amazon Resource Name (ARN)
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The GeoMatchConstraint objects which contain the country that you want AWS WAF to search for.
        pub geo_match_constraints: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::waf::GeoMatchSetGeoMatchConstraint>>,
        >,
        /// The name or description of the GeoMatchSet.
        pub name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: GeoMatchSetArgs,
    ) -> GeoMatchSetResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let geo_match_constraints_binding = args
            .geo_match_constraints
            .get_output(context);
        let name_binding = args.name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:waf/geoMatchSet:GeoMatchSet".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "geoMatchConstraints".into(),
                    value: &geo_match_constraints_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        GeoMatchSetResult {
            arn: o.get_field("arn"),
            geo_match_constraints: o.get_field("geoMatchConstraints"),
            name: o.get_field("name"),
        }
    }
}
