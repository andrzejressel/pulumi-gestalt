/// The resource `random.RandomPet` generates random pet names that are intended to be used as unique identifiers for other resources.
///
/// This resource can be used in conjunction with resources that have the `create_before_destroy` lifecycle flag set, to avoid conflicts with unique names during the brief period where both the old and new resources exist concurrently.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   # The following example shows how to generate a unique pet name
///   # for an AWS EC2 instance that changes each time a new AMI id is
///   # selected.
///   serverRandomPet:
///     type: random:RandomPet
///     properties:
///       keepers:
///         ami_id: ${var.ami_id}
///   serverInstance:
///     type: aws:ec2:Instance
///     properties:
///       tags:
///         Name: web-server-${serverRandomPet.id}
///       # Read the AMI id "through" the random_pet resource to ensure that
///       #   # both will change together.
///       ami: ${serverRandomPet.keepers.amiId}
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod random_pet {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RandomPetArgs {
        /// Arbitrary map of values that, when changed, will trigger recreation of resource. See the main provider documentation for more information.
        #[builder(into, default)]
        pub keepers: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The length (in words) of the pet name. Defaults to 2
        #[builder(into, default)]
        pub length: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// A string to prefix the name with.
        #[builder(into, default)]
        pub prefix: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The character to separate words in the pet name. Defaults to "-"
        #[builder(into, default)]
        pub separator: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct RandomPetResult {
        /// Arbitrary map of values that, when changed, will trigger recreation of resource. See the main provider documentation for more information.
        pub keepers: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The length (in words) of the pet name. Defaults to 2
        pub length: pulumi_gestalt_rust::Output<i32>,
        /// A string to prefix the name with.
        pub prefix: pulumi_gestalt_rust::Output<Option<String>>,
        /// The character to separate words in the pet name. Defaults to "-"
        pub separator: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: RandomPetArgs,
    ) -> RandomPetResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let keepers_binding = args.keepers.get_output(context);
        let length_binding = args.length.get_output(context);
        let prefix_binding = args.prefix.get_output(context);
        let separator_binding = args.separator.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "random:index/randomPet:RandomPet".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "keepers".into(),
                    value: &keepers_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "length".into(),
                    value: &length_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "prefix".into(),
                    value: &prefix_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "separator".into(),
                    value: &separator_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        RandomPetResult {
            keepers: o.get_field("keepers"),
            length: o.get_field("length"),
            prefix: o.get_field("prefix"),
            separator: o.get_field("separator"),
        }
    }
}
