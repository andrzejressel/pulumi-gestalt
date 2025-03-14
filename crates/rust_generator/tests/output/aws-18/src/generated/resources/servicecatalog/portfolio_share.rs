/// Manages a Service Catalog Portfolio Share. Shares the specified portfolio with the specified account or organization node. You can share portfolios to an organization, an organizational unit, or a specific account.
///
/// If the portfolio share with the specified account or organization node already exists, using this resource to re-create the share will have no effect and will not return an error. You can then use this resource to update the share.
///
/// > **NOTE:** Shares to an organization node can only be created by the management account of an organization or by a delegated administrator. If a delegated admin is de-registered, they can no longer create portfolio shares.
///
/// > **NOTE:** AWSOrganizationsAccess must be enabled in order to create a portfolio share to an organization node.
///
/// > **NOTE:** You can't share a shared resource, including portfolios that contain a shared product.
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
///     let example = portfolio_share::create(
///         "example",
///         PortfolioShareArgs::builder()
///             .portfolio_id("${exampleAwsServicecatalogPortfolio.id}")
///             .principal_id("012128675309")
///             .type_("ACCOUNT")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_servicecatalog_portfolio_share` using the portfolio share ID. For example:
///
/// ```sh
/// $ pulumi import aws:servicecatalog/portfolioShare:PortfolioShare example port-12344321:ACCOUNT:123456789012
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod portfolio_share {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PortfolioShareArgs {
        /// Language code. Valid values: `en` (English), `jp` (Japanese), `zh` (Chinese). Default value is `en`.
        #[builder(into, default)]
        pub accept_language: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Portfolio identifier.
        #[builder(into)]
        pub portfolio_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Identifier of the principal with whom you will share the portfolio. Valid values AWS account IDs and ARNs of AWS Organizations and organizational units.
        #[builder(into)]
        pub principal_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Enables or disables Principal sharing when creating the portfolio share. If this flag is not provided, principal sharing is disabled.
        #[builder(into, default)]
        pub share_principals: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Whether to enable sharing of `aws.servicecatalog.TagOption` resources when creating the portfolio share.
        #[builder(into, default)]
        pub share_tag_options: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Type of portfolio share. Valid values are `ACCOUNT` (an external account), `ORGANIZATION` (a share to every account in an organization), `ORGANIZATIONAL_UNIT`, `ORGANIZATION_MEMBER_ACCOUNT` (a share to an account in an organization).
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub type_: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Whether to wait (up to the timeout) for the share to be accepted. Organizational shares are automatically accepted.
        #[builder(into, default)]
        pub wait_for_acceptance: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct PortfolioShareResult {
        /// Language code. Valid values: `en` (English), `jp` (Japanese), `zh` (Chinese). Default value is `en`.
        pub accept_language: pulumi_gestalt_rust::Output<Option<String>>,
        /// Whether the shared portfolio is imported by the recipient account. If the recipient is organizational, the share is automatically imported, and the field is always set to true.
        pub accepted: pulumi_gestalt_rust::Output<bool>,
        /// Portfolio identifier.
        pub portfolio_id: pulumi_gestalt_rust::Output<String>,
        /// Identifier of the principal with whom you will share the portfolio. Valid values AWS account IDs and ARNs of AWS Organizations and organizational units.
        pub principal_id: pulumi_gestalt_rust::Output<String>,
        /// Enables or disables Principal sharing when creating the portfolio share. If this flag is not provided, principal sharing is disabled.
        pub share_principals: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Whether to enable sharing of `aws.servicecatalog.TagOption` resources when creating the portfolio share.
        pub share_tag_options: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Type of portfolio share. Valid values are `ACCOUNT` (an external account), `ORGANIZATION` (a share to every account in an organization), `ORGANIZATIONAL_UNIT`, `ORGANIZATION_MEMBER_ACCOUNT` (a share to an account in an organization).
        ///
        /// The following arguments are optional:
        pub type_: pulumi_gestalt_rust::Output<String>,
        /// Whether to wait (up to the timeout) for the share to be accepted. Organizational shares are automatically accepted.
        pub wait_for_acceptance: pulumi_gestalt_rust::Output<Option<bool>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: PortfolioShareArgs,
    ) -> PortfolioShareResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let accept_language_binding = args.accept_language.get_output(context);
        let portfolio_id_binding = args.portfolio_id.get_output(context);
        let principal_id_binding = args.principal_id.get_output(context);
        let share_principals_binding = args.share_principals.get_output(context);
        let share_tag_options_binding = args.share_tag_options.get_output(context);
        let type__binding = args.type_.get_output(context);
        let wait_for_acceptance_binding = args.wait_for_acceptance.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:servicecatalog/portfolioShare:PortfolioShare".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "acceptLanguage".into(),
                    value: &accept_language_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "portfolioId".into(),
                    value: &portfolio_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "principalId".into(),
                    value: &principal_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sharePrincipals".into(),
                    value: &share_principals_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "shareTagOptions".into(),
                    value: &share_tag_options_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "type".into(),
                    value: &type__binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "waitForAcceptance".into(),
                    value: &wait_for_acceptance_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        PortfolioShareResult {
            accept_language: o.get_field("acceptLanguage"),
            accepted: o.get_field("accepted"),
            portfolio_id: o.get_field("portfolioId"),
            principal_id: o.get_field("principalId"),
            share_principals: o.get_field("sharePrincipals"),
            share_tag_options: o.get_field("shareTagOptions"),
            type_: o.get_field("type"),
            wait_for_acceptance: o.get_field("waitForAcceptance"),
        }
    }
}
