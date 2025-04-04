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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod object_acl {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ObjectACLArgs {
        /// The name of the bucket the object is stored in.
        #[builder(into)]
        pub bucket: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the object to apply the acl to.
        ///
        /// - - -
        #[builder(into)]
        pub object: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The "canned" [predefined ACL](https://cloud.google.com/storage/docs/access-control#predefined-acl) to apply. Must be set if `role_entity` is not.
        #[builder(into, default)]
        pub predefined_acl: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// List of role/entity pairs in the form `ROLE:entity`. See [GCS Object ACL documentation](https://cloud.google.com/storage/docs/json_api/v1/objectAccessControls) for more details.
        /// Must be set if `predefined_acl` is not.
        #[builder(into, default)]
        pub role_entities: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
    }
    #[allow(dead_code)]
    pub struct ObjectACLResult {
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
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ObjectACLArgs,
    ) -> ObjectACLResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let bucket_binding = args.bucket.get_output(context);
        let object_binding = args.object.get_output(context);
        let predefined_acl_binding = args.predefined_acl.get_output(context);
        let role_entities_binding = args.role_entities.get_output(context);
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
        };
        let o = context.register_resource(request);
        ObjectACLResult {
            bucket: o.get_field("bucket"),
            object: o.get_field("object"),
            predefined_acl: o.get_field("predefinedAcl"),
            role_entities: o.get_field("roleEntities"),
        }
    }
}
