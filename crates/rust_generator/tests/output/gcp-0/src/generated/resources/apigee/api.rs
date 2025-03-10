/// To get more information about API proxies see, see:
///
/// * [API documentation](https://cloud.google.com/apigee/docs/reference/apis/apigee/rest/v1/organizations.apis)
/// * How-to Guides
///   * [API proxies](https://cloud.google.com/apigee/docs/resources)
///
///
/// ## Import
///
/// An API proxy can be imported using any of these accepted formats:
///
/// * `{{org_id}}/apis/{{name}}`
///
/// * `{{org_id}}/{{name}}`
///
/// When using the `pulumi import` command, API proxy can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:apigee/api:Api default {{org_id}}/apis/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:apigee/api:Api default {{org_id}}/{{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod api {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ApiArgs {
        /// Path to the config zip bundle.
        ///
        /// - - -
        #[builder(into)]
        pub config_bundle: pulumi_gestalt_rust::InputOrOutput<String>,
        #[builder(into, default)]
        pub detect_md5hash: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the API proxy.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Apigee Organization name associated with the Apigee instance.
        #[builder(into)]
        pub org_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ApiResult {
        /// Path to the config zip bundle.
        ///
        /// - - -
        pub config_bundle: pulumi_gestalt_rust::Output<String>,
        pub detect_md5hash: pulumi_gestalt_rust::Output<Option<String>>,
        /// The id of the most recently created revision for this API proxy.
        pub latest_revision_id: pulumi_gestalt_rust::Output<String>,
        /// (Computed) Base 64 MD5 hash of the uploaded data. It is speculative as remote does not return hash of the bundle. Remote changes are detected using returned last_modified timestamp.
        pub md5hash: pulumi_gestalt_rust::Output<String>,
        /// Metadata describing the API proxy.
        /// Structure is documented below.
        pub meta_datas: pulumi_gestalt_rust::Output<
            Vec<super::super::types::apigee::ApiMetaData>,
        >,
        /// The ID of the API proxy.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The Apigee Organization name associated with the Apigee instance.
        pub org_id: pulumi_gestalt_rust::Output<String>,
        /// A list of revisions of this API proxy.
        pub revisions: pulumi_gestalt_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ApiArgs,
    ) -> ApiResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let config_bundle_binding = args.config_bundle.get_output(context);
        let detect_md5hash_binding = args.detect_md5hash.get_output(context);
        let name_binding = args.name.get_output(context);
        let org_id_binding = args.org_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:apigee/api:Api".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "configBundle".into(),
                    value: &config_bundle_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "detectMd5hash".into(),
                    value: &detect_md5hash_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "orgId".into(),
                    value: &org_id_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        ApiResult {
            config_bundle: o.get_field("configBundle"),
            detect_md5hash: o.get_field("detectMd5hash"),
            latest_revision_id: o.get_field("latestRevisionId"),
            md5hash: o.get_field("md5hash"),
            meta_datas: o.get_field("metaDatas"),
            name: o.get_field("name"),
            org_id: o.get_field("orgId"),
            revisions: o.get_field("revisions"),
        }
    }
}
