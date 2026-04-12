/// Manages a Data Share.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-resources
///       location: West Europe
///   exampleAccount:
///     type: azure:datashare:Account
///     name: example
///     properties:
///       name: example-dsa
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       identity:
///         type: SystemAssigned
///       tags:
///         foo: bar
///   exampleShare:
///     type: azure:datashare:Share
///     name: example
///     properties:
///       name: example_dss
///       accountId: ${exampleAccount.id}
///       kind: CopyBased
///       description: example desc
///       terms: example terms
///       snapshotSchedule:
///         name: example-ss
///         recurrence: Day
///         startTime: 2020-04-17T04:47:52.9614956Z
/// ```
///
/// ## Import
///
/// Data Shares can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:datashare/share:Share example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.DataShare/accounts/account1/shares/share1
/// ```
///
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod share {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ShareArgs {
        /// The ID of the Data Share account in which the Data Share is created. Changing this forces a new Data Share to be created.
        #[builder(into)]
        pub account_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Data Share's description.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The kind of the Data Share. Possible values are `CopyBased` and `InPlace`. Changing this forces a new Data Share to be created.
        #[builder(into)]
        pub kind: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name which should be used for this Data Share. Changing this forces a new Data Share to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A `snapshot_schedule` block as defined below.
        #[builder(into, default)]
        pub snapshot_schedule: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::datashare::ShareSnapshotSchedule>,
        >,
        /// The terms of the Data Share.
        #[builder(into, default)]
        pub terms: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ShareResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Data Share account in which the Data Share is created. Changing this forces a new Data Share to be created.
        pub account_id: pulumi_gestalt_rust::Output<String>,
        /// The Data Share's description.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The kind of the Data Share. Possible values are `CopyBased` and `InPlace`. Changing this forces a new Data Share to be created.
        pub kind: pulumi_gestalt_rust::Output<String>,
        /// The name which should be used for this Data Share. Changing this forces a new Data Share to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A `snapshot_schedule` block as defined below.
        pub snapshot_schedule: pulumi_gestalt_rust::Output<
            Option<super::super::types::datashare::ShareSnapshotSchedule>,
        >,
        /// The terms of the Data Share.
        pub terms: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ShareArgs,
    ) -> ShareResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ShareArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> ShareResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ShareArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> ShareResult {
        let account_id_binding = args.account_id.get_output(ctx);
        let description_binding = args.description.get_output(ctx);
        let kind_binding = args.kind.get_output(ctx);
        let name_binding = args.name.get_output(ctx);
        let snapshot_schedule_binding = args.snapshot_schedule.get_output(ctx);
        let terms_binding = args.terms.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:datashare/share:Share".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accountId".into(),
                    value: &account_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "kind".into(),
                    value: &kind_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "snapshotSchedule".into(),
                    value: &snapshot_schedule_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "terms".into(),
                    value: &terms_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        ShareResult {
            id: o.get_id(),
            urn: o.get_urn(),
            account_id: o.get_field("accountId"),
            description: o.get_field("description"),
            kind: o.get_field("kind"),
            name: o.get_field("name"),
            snapshot_schedule: o.get_field("snapshotSchedule"),
            terms: o.get_field("terms"),
        }
    }
}
