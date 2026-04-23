/// Resource for managing an AWS DataZone Glossary Term.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:iam:Role
///     properties:
///       name: example
///       assumeRolePolicy:
///         fn::toJSON:
///           Version: 2012-10-17
///           Statement:
///             - Action:
///                 - sts:AssumeRole
///                 - sts:TagSession
///               Effect: Allow
///               Principal:
///                 Service: datazone.amazonaws.com
///             - Action:
///                 - sts:AssumeRole
///                 - sts:TagSession
///               Effect: Allow
///               Principal:
///                 Service: cloudformation.amazonaws.com
///       inlinePolicies:
///         - name: example
///           policy:
///             fn::toJSON:
///               Version: 2012-10-17
///               Statement:
///                 - Action:
///                     - datazone:*
///                     - ram:*
///                     - sso:*
///                     - kms:*
///                   Effect: Allow
///                   Resource: '*'
///   exampleDomain:
///     type: aws:datazone:Domain
///     name: example
///     properties:
///       name: example_name
///       domainExecutionRole: ${example.arn}
///   exampleSecurityGroup:
///     type: aws:ec2:SecurityGroup
///     name: example
///     properties:
///       name: example_name
///   exampleProject:
///     type: aws:datazone:Project
///     name: example
///     properties:
///       domainIdentifier: ${exampleDomain.id}
///       glossaryTerms:
///         - 2N8w6XJCwZf
///       name: example
///       skipDeletionCheck: true
///   exampleGlossary:
///     type: aws:datazone:Glossary
///     name: example
///     properties:
///       description: description
///       name: example
///       owningProjectIdentifier: ${exampleProject.id}
///       status: ENABLED
///       domainIdentifier: ${exampleProject.domainIdentifier}
///   exampleGlossaryTerm:
///     type: aws:datazone:GlossaryTerm
///     name: example
///     properties:
///       domainIdentifier: ${exampleDomain.id}
///       glossaryIdentifier: ${exampleGlossary.id}
///       name: example
///       status: ENABLED
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import DataZone Glossary Term using a comma-delimited string combining the `domain_identifier`, `id`, and the `glossary_identifier`. For example:
///
/// ```sh
/// $ pulumi import aws:datazone/glossaryTerm:GlossaryTerm example domain-id,glossary-term-id,glossary-id
/// ```
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod glossary_term {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GlossaryTermArgs {
        /// Identifier of domain.
        #[builder(into, default)]
        pub domain_identifier: pulumi_gestalt_rust::Input<Option<String>>,
        /// Identifier of glossary.
        #[builder(into)]
        pub glossary_identifier: pulumi_gestalt_rust::Input<String>,
        /// Long description of entry.
        #[builder(into, default)]
        pub long_description: pulumi_gestalt_rust::Input<Option<String>>,
        /// Name of glossary term.
        ///
        /// The following arguments are optional:
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::Input<Option<String>>,
        /// Short description of entry.
        #[builder(into, default)]
        pub short_description: pulumi_gestalt_rust::Input<Option<String>>,
        /// If glossary term is ENABLED or DISABLED.
        #[builder(into, default)]
        pub status: pulumi_gestalt_rust::Input<Option<String>>,
        /// Object classifying the term relations through the following attributes:
        #[builder(into, default)]
        pub term_relations: pulumi_gestalt_rust::Input<
            Option<super::super::types::datazone::GlossaryTermTermRelations>,
        >,
        #[builder(into, default)]
        pub timeouts: pulumi_gestalt_rust::Input<
            Option<super::super::types::datazone::GlossaryTermTimeouts>,
        >,
    }
    #[allow(dead_code)]
    pub struct GlossaryTermResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// Time of glossary term creation.
        pub created_at: pulumi_gestalt_rust::Output<String>,
        /// Creator of glossary term.
        pub created_by: pulumi_gestalt_rust::Output<String>,
        /// Identifier of domain.
        pub domain_identifier: pulumi_gestalt_rust::Output<Option<String>>,
        /// Identifier of glossary.
        pub glossary_identifier: pulumi_gestalt_rust::Output<String>,
        /// Long description of entry.
        pub long_description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Name of glossary term.
        ///
        /// The following arguments are optional:
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Short description of entry.
        pub short_description: pulumi_gestalt_rust::Output<Option<String>>,
        /// If glossary term is ENABLED or DISABLED.
        pub status: pulumi_gestalt_rust::Output<Option<String>>,
        /// Object classifying the term relations through the following attributes:
        pub term_relations: pulumi_gestalt_rust::Output<
            Option<super::super::types::datazone::GlossaryTermTermRelations>,
        >,
        pub timeouts: pulumi_gestalt_rust::Output<
            Option<super::super::types::datazone::GlossaryTermTimeouts>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: GlossaryTermArgs,
    ) -> GlossaryTermResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: GlossaryTermArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> GlossaryTermResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: GlossaryTermArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> GlossaryTermResult {
        let domain_identifier_binding = args.domain_identifier.get_output(ctx);
        let glossary_identifier_binding = args.glossary_identifier.get_output(ctx);
        let long_description_binding = args.long_description.get_output(ctx);
        let name_binding = args.name.get_output(ctx);
        let short_description_binding = args.short_description.get_output(ctx);
        let status_binding = args.status.get_output(ctx);
        let term_relations_binding = args.term_relations.get_output(ctx);
        let timeouts_binding = args.timeouts.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:datazone/glossaryTerm:GlossaryTerm".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "domainIdentifier".into(),
                    value: &domain_identifier_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "glossaryIdentifier".into(),
                    value: &glossary_identifier_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "longDescription".into(),
                    value: &long_description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "shortDescription".into(),
                    value: &short_description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "status".into(),
                    value: &status_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "termRelations".into(),
                    value: &term_relations_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "timeouts".into(),
                    value: &timeouts_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        GlossaryTermResult {
            id: o.get_id(),
            urn: o.get_urn(),
            created_at: o.get_field("createdAt"),
            created_by: o.get_field("createdBy"),
            domain_identifier: o.get_field("domainIdentifier"),
            glossary_identifier: o.get_field("glossaryIdentifier"),
            long_description: o.get_field("longDescription"),
            name: o.get_field("name"),
            short_description: o.get_field("shortDescription"),
            status: o.get_field("status"),
            term_relations: o.get_field("termRelations"),
            timeouts: o.get_field("timeouts"),
        }
    }
}
