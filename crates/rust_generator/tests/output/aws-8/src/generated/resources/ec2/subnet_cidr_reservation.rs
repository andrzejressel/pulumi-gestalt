/// Provides a subnet CIDR reservation resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = subnet_cidr_reservation::create(
///         "example",
///         SubnetCidrReservationArgs::builder()
///             .cidr_block("10.0.0.16/28")
///             .reservation_type("prefix")
///             .subnet_id("${exampleAwsSubnet.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Existing CIDR reservations using `SUBNET_ID:RESERVATION_ID`. For example:
///
/// ```sh
/// $ pulumi import aws:ec2/subnetCidrReservation:SubnetCidrReservation example subnet-01llsxvsxabqiymcz:scr-4mnvz6wb7otksjcs9
/// ```
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod subnet_cidr_reservation {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SubnetCidrReservationArgs {
        /// The CIDR block for the reservation.
        #[builder(into)]
        pub cidr_block: pulumi_gestalt_rust::Input<String>,
        /// A brief description of the reservation.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::Input<Option<String>>,
        /// The type of reservation to create. Valid values: `explicit`, `prefix`
        #[builder(into)]
        pub reservation_type: pulumi_gestalt_rust::Input<String>,
        /// The ID of the subnet to create the reservation for.
        #[builder(into)]
        pub subnet_id: pulumi_gestalt_rust::Input<String>,
    }
    #[allow(dead_code)]
    pub struct SubnetCidrReservationResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// The CIDR block for the reservation.
        pub cidr_block: pulumi_gestalt_rust::Output<String>,
        /// A brief description of the reservation.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// ID of the AWS account that owns this CIDR reservation.
        pub owner_id: pulumi_gestalt_rust::Output<String>,
        /// The type of reservation to create. Valid values: `explicit`, `prefix`
        pub reservation_type: pulumi_gestalt_rust::Output<String>,
        /// The ID of the subnet to create the reservation for.
        pub subnet_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SubnetCidrReservationArgs,
    ) -> SubnetCidrReservationResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SubnetCidrReservationArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> SubnetCidrReservationResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SubnetCidrReservationArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> SubnetCidrReservationResult {
        let cidr_block_binding = args.cidr_block.get_output(ctx);
        let description_binding = args.description.get_output(ctx);
        let reservation_type_binding = args.reservation_type.get_output(ctx);
        let subnet_id_binding = args.subnet_id.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:ec2/subnetCidrReservation:SubnetCidrReservation".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "cidrBlock".into(),
                    value: &cidr_block_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "reservationType".into(),
                    value: &reservation_type_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "subnetId".into(),
                    value: &subnet_id_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        SubnetCidrReservationResult {
            id: o.get_id(),
            urn: o.get_urn(),
            cidr_block: o.get_field("cidrBlock"),
            description: o.get_field("description"),
            owner_id: o.get_field("ownerId"),
            reservation_type: o.get_field("reservationType"),
            subnet_id: o.get_field("subnetId"),
        }
    }
}
