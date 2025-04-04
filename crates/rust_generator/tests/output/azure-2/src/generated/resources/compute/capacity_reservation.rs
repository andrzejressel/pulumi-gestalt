/// Manages a Capacity Reservation within a Capacity Reservation Group.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resource_group::create(
///         "example",
///         ResourceGroupArgs::builder()
///             .location("West Europe")
///             .name("example-rg")
///             .build_struct(),
///     );
///     let exampleCapacityReservation = capacity_reservation::create(
///         "exampleCapacityReservation",
///         CapacityReservationArgs::builder()
///             .capacity_reservation_group_id("${exampleCapacityReservationGroup.id}")
///             .name("example-capacity-reservation")
///             .sku(
///                 CapacityReservationSku::builder()
///                     .capacity(1)
///                     .name("Standard_D2s_v3")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
///     let exampleCapacityReservationGroup = capacity_reservation_group::create(
///         "exampleCapacityReservationGroup",
///         CapacityReservationGroupArgs::builder()
///             .location("${example.location}")
///             .name("example-capacity-reservation-group")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Capacity Reservations can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:compute/capacityReservation:CapacityReservation example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Compute/capacityReservationGroups/capacityReservationGroup1/capacityReservations/capacityReservation1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod capacity_reservation {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CapacityReservationArgs {
        /// The ID of the Capacity Reservation Group where the Capacity Reservation exists. Changing this forces a new resource to be created.
        #[builder(into)]
        pub capacity_reservation_group_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the name of this Capacity Reservation. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A `sku` block as defined below.
        #[builder(into)]
        pub sku: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::compute::CapacityReservationSku,
        >,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies the Availability Zone for this Capacity Reservation. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub zone: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct CapacityReservationResult {
        /// The ID of the Capacity Reservation Group where the Capacity Reservation exists. Changing this forces a new resource to be created.
        pub capacity_reservation_group_id: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name of this Capacity Reservation. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A `sku` block as defined below.
        pub sku: pulumi_gestalt_rust::Output<
            super::super::types::compute::CapacityReservationSku,
        >,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies the Availability Zone for this Capacity Reservation. Changing this forces a new resource to be created.
        pub zone: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: CapacityReservationArgs,
    ) -> CapacityReservationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let capacity_reservation_group_id_binding = args
            .capacity_reservation_group_id
            .get_output(context);
        let name_binding = args.name.get_output(context);
        let sku_binding = args.sku.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let zone_binding = args.zone.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:compute/capacityReservation:CapacityReservation".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "capacityReservationGroupId".into(),
                    value: &capacity_reservation_group_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sku".into(),
                    value: &sku_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "zone".into(),
                    value: &zone_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        CapacityReservationResult {
            capacity_reservation_group_id: o.get_field("capacityReservationGroupId"),
            name: o.get_field("name"),
            sku: o.get_field("sku"),
            tags: o.get_field("tags"),
            zone: o.get_field("zone"),
        }
    }
}
