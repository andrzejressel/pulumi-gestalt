/// A named resource representing a shared pool of capacity.
///
///
/// To get more information about Reservation, see:
///
/// * [API documentation](https://cloud.google.com/pubsub/lite/docs/reference/rest/v1/admin.projects.locations.reservations)
/// * How-to Guides
///     * [Managing Reservations](https://cloud.google.com/pubsub/lite/docs/reservations)
///
/// ## Example Usage
///
/// ### Pubsub Lite Reservation Basic
///
///
/// ```yaml
/// resources:
///   example:
///     type: gcp:pubsub:LiteReservation
///     properties:
///       name: example-reservation
///       project: ${project.number}
///       throughputCapacity: 2
/// variables:
///   project:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments: {}
/// ```
///
/// ## Import
///
/// Reservation can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{region}}/reservations/{{name}}`
///
/// * `{{project}}/{{region}}/{{name}}`
///
/// * `{{region}}/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, Reservation can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:pubsub/liteReservation:LiteReservation default projects/{{project}}/locations/{{region}}/reservations/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:pubsub/liteReservation:LiteReservation default {{project}}/{{region}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:pubsub/liteReservation:LiteReservation default {{region}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:pubsub/liteReservation:LiteReservation default {{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod lite_reservation {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LiteReservationArgs {
        /// Name of the reservation.
        ///
        ///
        /// - - -
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The region of the pubsub lite reservation.
        #[builder(into, default)]
        pub region: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The reserved throughput capacity. Every unit of throughput capacity is
        /// equivalent to 1 MiB/s of published messages or 2 MiB/s of subscribed
        /// messages.
        #[builder(into)]
        pub throughput_capacity: pulumi_gestalt_rust::InputOrOutput<i32>,
    }
    #[allow(dead_code)]
    pub struct LiteReservationResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Name of the reservation.
        ///
        ///
        /// - - -
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The region of the pubsub lite reservation.
        pub region: pulumi_gestalt_rust::Output<Option<String>>,
        /// The reserved throughput capacity. Every unit of throughput capacity is
        /// equivalent to 1 MiB/s of published messages or 2 MiB/s of subscribed
        /// messages.
        pub throughput_capacity: pulumi_gestalt_rust::Output<i32>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: LiteReservationArgs,
    ) -> LiteReservationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let project_binding = args.project.get_output(context);
        let region_binding = args.region.get_output(context);
        let throughput_capacity_binding = args.throughput_capacity.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:pubsub/liteReservation:LiteReservation".into(),
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
                    name: "region".into(),
                    value: &region_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "throughputCapacity".into(),
                    value: &throughput_capacity_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        LiteReservationResult {
            id: o.get_field("id"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            region: o.get_field("region"),
            throughput_capacity: o.get_field("throughputCapacity"),
        }
    }
}
