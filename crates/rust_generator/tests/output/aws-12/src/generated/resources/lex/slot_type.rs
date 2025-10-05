/// Provides an Amazon Lex Slot Type resource. For more information see
/// [Amazon Lex: How It Works](https://docs.aws.amazon.com/lex/latest/dg/how-it-works.html)
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let flowerTypes = slot_type::create(
///         "flowerTypes",
///         SlotTypeArgs::builder()
///             .create_version(true)
///             .description("Types of flowers to order")
///             .enumeration_values(
///                 vec![
///                     SlotTypeEnumerationValue::builder().synonyms(vec!["Lirium",
///                     "Martagon",]).value("lilies").build_struct(),
///                     SlotTypeEnumerationValue::builder().synonyms(vec!["Eduardoregelia",
///                     "Podonix",]).value("tulips").build_struct(),
///                 ],
///             )
///             .name("FlowerTypes")
///             .value_selection_strategy("ORIGINAL_VALUE")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import slot types using their name. For example:
///
/// ```sh
/// $ pulumi import aws:lex/slotType:SlotType flower_types FlowerTypes
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod slot_type {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SlotTypeArgs {
        /// Determines if a new slot type version is created when the initial resource is created and on each
        /// update. Defaults to `false`.
        #[builder(into, default)]
        pub create_version: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// A description of the slot type. Must be less than or equal to 200 characters in length.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A list of EnumerationValue objects that defines the values that
        /// the slot type can take. Each value can have a list of synonyms, which are additional values that help
        /// train the machine learning model about the values that it resolves for a slot. Attributes are
        /// documented under enumeration_value.
        #[builder(into)]
        pub enumeration_values: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::lex::SlotTypeEnumerationValue>,
        >,
        /// The name of the slot type. The name is not case sensitive. Must be less than or equal to 100 characters in length.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Determines the slot resolution strategy that Amazon Lex
        /// uses to return slot type values. `ORIGINAL_VALUE` returns the value entered by the user if the user
        /// value is similar to the slot value. `TOP_RESOLUTION` returns the first value in the resolution list
        /// if there is a resolution list for the slot, otherwise null is returned. Defaults to `ORIGINAL_VALUE`.
        #[builder(into, default)]
        pub value_selection_strategy: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct SlotTypeResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Checksum identifying the version of the slot type that was created. The checksum is
        /// not included as an argument because the resource will add it automatically when updating the slot type.
        pub checksum: pulumi_gestalt_rust::Output<String>,
        /// Determines if a new slot type version is created when the initial resource is created and on each
        /// update. Defaults to `false`.
        pub create_version: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The date when the slot type version was created.
        pub created_date: pulumi_gestalt_rust::Output<String>,
        /// A description of the slot type. Must be less than or equal to 200 characters in length.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// A list of EnumerationValue objects that defines the values that
        /// the slot type can take. Each value can have a list of synonyms, which are additional values that help
        /// train the machine learning model about the values that it resolves for a slot. Attributes are
        /// documented under enumeration_value.
        pub enumeration_values: pulumi_gestalt_rust::Output<
            Vec<super::super::types::lex::SlotTypeEnumerationValue>,
        >,
        /// The date when the `$LATEST` version of this slot type was updated.
        pub last_updated_date: pulumi_gestalt_rust::Output<String>,
        /// The name of the slot type. The name is not case sensitive. Must be less than or equal to 100 characters in length.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Determines the slot resolution strategy that Amazon Lex
        /// uses to return slot type values. `ORIGINAL_VALUE` returns the value entered by the user if the user
        /// value is similar to the slot value. `TOP_RESOLUTION` returns the first value in the resolution list
        /// if there is a resolution list for the slot, otherwise null is returned. Defaults to `ORIGINAL_VALUE`.
        pub value_selection_strategy: pulumi_gestalt_rust::Output<Option<String>>,
        /// The version of the slot type.
        pub version: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SlotTypeArgs,
    ) -> SlotTypeResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let create_version_binding = args.create_version.get_output(context);
        let description_binding = args.description.get_output(context);
        let enumeration_values_binding = args.enumeration_values.get_output(context);
        let name_binding = args.name.get_output(context);
        let value_selection_strategy_binding = args
            .value_selection_strategy
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:lex/slotType:SlotType".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "createVersion".into(),
                    value: &create_version_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enumerationValues".into(),
                    value: &enumeration_values_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "valueSelectionStrategy".into(),
                    value: &value_selection_strategy_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        SlotTypeResult {
            id: o.get_field("id"),
            checksum: o.get_field("checksum"),
            create_version: o.get_field("createVersion"),
            created_date: o.get_field("createdDate"),
            description: o.get_field("description"),
            enumeration_values: o.get_field("enumerationValues"),
            last_updated_date: o.get_field("lastUpdatedDate"),
            name: o.get_field("name"),
            value_selection_strategy: o.get_field("valueSelectionStrategy"),
            version: o.get_field("version"),
        }
    }
}
