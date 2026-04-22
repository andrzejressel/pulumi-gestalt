/// Resource for managing AWS Cost Optimization Hub Preferences.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = preferences::create(
///         "example",
///         PreferencesArgs::builder().build_struct(),
///     );
/// }
/// ```
///
/// ### Usage with all the arguments
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = preferences::create(
///         "example",
///         PreferencesArgs::builder()
///             .member_account_discount_visibility("None")
///             .savings_estimation_mode("AfterDiscounts")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Cost Optimization Hub Preferences using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:costoptimizationhub/preferences:Preferences example 111222333444
/// ```
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod preferences {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PreferencesArgs {
        /// Customize whether the member accounts can see the "After Discounts" savings estimates. Valid values are `All` and `None`. Default value is `All`.
        #[builder(into, default)]
        pub member_account_discount_visibility: pulumi_gestalt_rust::Input<
            Option<String>,
        >,
        /// Customize how estimated monthly savings are calculated. Valid values are `BeforeDiscounts` and `AfterDiscounts`. Default value is `BeforeDiscounts`.
        #[builder(into, default)]
        pub savings_estimation_mode: pulumi_gestalt_rust::Input<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct PreferencesResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// Customize whether the member accounts can see the "After Discounts" savings estimates. Valid values are `All` and `None`. Default value is `All`.
        pub member_account_discount_visibility: pulumi_gestalt_rust::Output<String>,
        /// Customize how estimated monthly savings are calculated. Valid values are `BeforeDiscounts` and `AfterDiscounts`. Default value is `BeforeDiscounts`.
        pub savings_estimation_mode: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: PreferencesArgs,
    ) -> PreferencesResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: PreferencesArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> PreferencesResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: PreferencesArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> PreferencesResult {
        let member_account_discount_visibility_binding = args
            .member_account_discount_visibility
            .get_output(ctx);
        let savings_estimation_mode_binding = args
            .savings_estimation_mode
            .get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:costoptimizationhub/preferences:Preferences".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "memberAccountDiscountVisibility".into(),
                    value: &member_account_discount_visibility_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "savingsEstimationMode".into(),
                    value: &savings_estimation_mode_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        PreferencesResult {
            id: o.get_id(),
            urn: o.get_urn(),
            member_account_discount_visibility: o
                .get_field("memberAccountDiscountVisibility"),
            savings_estimation_mode: o.get_field("savingsEstimationMode"),
        }
    }
}
