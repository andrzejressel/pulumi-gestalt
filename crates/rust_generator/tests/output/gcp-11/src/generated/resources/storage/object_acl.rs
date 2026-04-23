/// Authoritatively manages the access control list (ACL) for an object in a Google
/// Cloud Storage (GCS) bucket. Removing a `gcp.storage.ObjectACL` sets the
/// acl to the `private` [predefined ACL](https://cloud.google.com/storage/docs/access-control#predefined-acl).
///
/// For more information see
/// [the official documentation](https://cloud.google.com/storage/docs/access-control/lists)
/// and
/// [API](https://cloud.google.com/storage/docs/json_api/v1/objectAccessControls).
///
/// > Want fine-grained control over object ACLs? Use `gcp.storage.ObjectAccessControl` to control individual
/// role entity pairs.
///
/// ## Example Usage
///
/// Create an object ACL with one owner and one reader.
///
/// ```yaml
/// resources:
///   image-store:
///     type: gcp:storage:Bucket
///     properties:
///       name: image-store-bucket
///       location: EU
///   image:
///     type: gcp:storage:BucketObject
///     properties:
///       name: image1
///       bucket: ${["image-store"].name}
///       source:
///         fn::FileAsset: image1.jpg
///   image-store-acl:
///     type: gcp:storage:ObjectACL
///     properties:
///       bucket: ${["image-store"].name}
///       object: ${image.outputName}
///       roleEntities:
///         - OWNER:user-my.email@gmail.com
///         - READER:group-mygroup
/// ```
///
/// ## Import
///
/// This resource does not support import.
///
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod object_acl {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ObjectACLArgs {
        /// The name of the bucket the object is stored in.
        #[builder(into)]
        pub bucket: pulumi_gestalt_rust::Input<String>,
        /// The name of the object to apply the acl to.
        ///
        /// - - -
        #[builder(into)]
        pub object: pulumi_gestalt_rust::Input<String>,
        /// The "canned" [predefined ACL](https://cloud.google.com/storage/docs/access-control#predefined-acl) to apply. Must be set if `role_entity` is not.
        #[builder(into, default)]
        pub predefined_acl: pulumi_gestalt_rust::Input<Option<String>>,
        /// List of role/entity pairs in the form `ROLE:entity`. See [GCS Object ACL documentation](https://cloud.google.com/storage/docs/json_api/v1/objectAccessControls) for more details.
        /// Must be set if `predefined_acl` is not.
        #[builder(into, default)]
        pub role_entities: pulumi_gestalt_rust::Input<Option<Vec<String>>>,
    }
    #[allow(dead_code)]
    pub struct ObjectACLResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// The name of the bucket the object is stored in.
        pub bucket: pulumi_gestalt_rust::Output<String>,
        /// The name of the object to apply the acl to.
        ///
        /// - - -
        pub object: pulumi_gestalt_rust::Output<String>,
        /// The "canned" [predefined ACL](https://cloud.google.com/storage/docs/access-control#predefined-acl) to apply. Must be set if `role_entity` is not.
        pub predefined_acl: pulumi_gestalt_rust::Output<Option<String>>,
        /// List of role/entity pairs in the form `ROLE:entity`. See [GCS Object ACL documentation](https://cloud.google.com/storage/docs/json_api/v1/objectAccessControls) for more details.
        /// Must be set if `predefined_acl` is not.
        pub role_entities: pulumi_gestalt_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ObjectACLArgs,
    ) -> ObjectACLResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ObjectACLArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> ObjectACLResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ObjectACLArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> ObjectACLResult {
        let bucket_binding = args.bucket.get_output(ctx);
        let object_binding = args.object.get_output(ctx);
        let predefined_acl_binding = args.predefined_acl.get_output(ctx);
        let role_entities_binding = args.role_entities.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:storage/objectACL:ObjectACL".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "bucket".into(),
                    value: &bucket_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "object".into(),
                    value: &object_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "predefinedAcl".into(),
                    value: &predefined_acl_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "roleEntities".into(),
                    value: &role_entities_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        ObjectACLResult {
            id: o.get_id(),
            urn: o.get_urn(),
            bucket: o.get_field("bucket"),
            object: o.get_field("object"),
            predefined_acl: o.get_field("predefinedAcl"),
            role_entities: o.get_field("roleEntities"),
        }
    }
}
