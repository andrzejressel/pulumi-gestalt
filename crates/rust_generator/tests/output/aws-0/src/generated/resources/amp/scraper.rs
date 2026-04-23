///
///
/// ## Import
///
/// Using `pulumi import`, import the Managed Scraper using its identifier.
/// For example:
///
/// ```sh
/// $ pulumi import aws:amp/scraper:Scraper example s-0123abc-0000-0123-a000-000000000000
/// ```
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod scraper {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ScraperArgs {
        /// a name to associate with the managed scraper. This is for your use, and does not need to be unique.
        #[builder(into, default)]
        pub alias: pulumi_gestalt_rust::Input<Option<String>>,
        /// Configuration block for the managed scraper to send metrics to. See `destination`.
        #[builder(into, default)]
        pub destination: pulumi_gestalt_rust::Input<
            Option<super::super::types::amp::ScraperDestination>,
        >,
        /// The configuration file to use in the new scraper. For more information, see [Scraper configuration](https://docs.aws.amazon.com/prometheus/latest/userguide/AMP-collector-how-to.html#AMP-collector-configuration).
        #[builder(into)]
        pub scrape_configuration: pulumi_gestalt_rust::Input<String>,
        /// Configuration block to specify where the managed scraper will collect metrics from. See `source`.
        ///
        /// The following arguments are optional:
        #[builder(into, default)]
        pub source: pulumi_gestalt_rust::Input<
            Option<super::super::types::amp::ScraperSource>,
        >,
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::Input<
            Option<std::collections::HashMap<String, String>>,
        >,
        #[builder(into, default)]
        pub timeouts: pulumi_gestalt_rust::Input<
            Option<super::super::types::amp::ScraperTimeouts>,
        >,
    }
    #[allow(dead_code)]
    pub struct ScraperResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// a name to associate with the managed scraper. This is for your use, and does not need to be unique.
        pub alias: pulumi_gestalt_rust::Output<Option<String>>,
        /// The Amazon Resource Name (ARN) of the new scraper.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Configuration block for the managed scraper to send metrics to. See `destination`.
        pub destination: pulumi_gestalt_rust::Output<
            Option<super::super::types::amp::ScraperDestination>,
        >,
        /// The Amazon Resource Name (ARN) of the IAM role that provides permissions for the scraper to discover, collect, and produce metrics
        pub role_arn: pulumi_gestalt_rust::Output<String>,
        /// The configuration file to use in the new scraper. For more information, see [Scraper configuration](https://docs.aws.amazon.com/prometheus/latest/userguide/AMP-collector-how-to.html#AMP-collector-configuration).
        pub scrape_configuration: pulumi_gestalt_rust::Output<String>,
        /// Configuration block to specify where the managed scraper will collect metrics from. See `source`.
        ///
        /// The following arguments are optional:
        pub source: pulumi_gestalt_rust::Output<
            Option<super::super::types::amp::ScraperSource>,
        >,
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub timeouts: pulumi_gestalt_rust::Output<
            Option<super::super::types::amp::ScraperTimeouts>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ScraperArgs,
    ) -> ScraperResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ScraperArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> ScraperResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ScraperArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> ScraperResult {
        let alias_binding = args.alias.get_output(ctx);
        let destination_binding = args.destination.get_output(ctx);
        let scrape_configuration_binding = args.scrape_configuration.get_output(ctx);
        let source_binding = args.source.get_output(ctx);
        let tags_binding = args.tags.get_output(ctx);
        let timeouts_binding = args.timeouts.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:amp/scraper:Scraper".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "alias".into(),
                    value: &alias_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "destination".into(),
                    value: &destination_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "scrapeConfiguration".into(),
                    value: &scrape_configuration_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "source".into(),
                    value: &source_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "timeouts".into(),
                    value: &timeouts_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        ScraperResult {
            id: o.get_id(),
            urn: o.get_urn(),
            alias: o.get_field("alias"),
            arn: o.get_field("arn"),
            destination: o.get_field("destination"),
            role_arn: o.get_field("roleArn"),
            scrape_configuration: o.get_field("scrapeConfiguration"),
            source: o.get_field("source"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            timeouts: o.get_field("timeouts"),
        }
    }
}
