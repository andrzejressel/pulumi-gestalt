#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_key_pair {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetKeyPairArgs {
        /// Custom filter block as described below.
        #[builder(into, default)]
        pub filters: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::super::types::ec2::GetKeyPairFilter>>,
        >,
        /// Whether to include the public key material in the response.
        #[builder(into, default)]
        pub include_public_key: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Key Pair name.
        #[builder(into, default)]
        pub key_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Key Pair ID.
        #[builder(into, default)]
        pub key_pair_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Any tags assigned to the Key Pair.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetKeyPairResult {
        /// ARN of the Key Pair.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Timestamp for when the key pair was created in ISO 8601 format.
        pub create_time: pulumi_gestalt_rust::Output<String>,
        pub filters: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::super::types::ec2::GetKeyPairFilter>>,
        >,
        /// SHA-1 digest of the DER encoded private key.
        pub fingerprint: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub include_public_key: pulumi_gestalt_rust::Output<Option<bool>>,
        pub key_name: pulumi_gestalt_rust::Output<Option<String>>,
        pub key_pair_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// Type of key pair.
        pub key_type: pulumi_gestalt_rust::Output<String>,
        /// Public key material.
        pub public_key: pulumi_gestalt_rust::Output<String>,
        /// Any tags assigned to the Key Pair.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetKeyPairArgs,
    ) -> GetKeyPairResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let filters_binding = args.filters.get_output(context);
        let include_public_key_binding = args.include_public_key.get_output(context);
        let key_name_binding = args.key_name.get_output(context);
        let key_pair_id_binding = args.key_pair_id.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:ec2/getKeyPair:getKeyPair".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "filters".into(),
                    value: &filters_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "includePublicKey".into(),
                    value: &include_public_key_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "keyName".into(),
                    value: &key_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "keyPairId".into(),
                    value: &key_pair_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetKeyPairResult {
            arn: o.get_field("arn"),
            create_time: o.get_field("createTime"),
            filters: o.get_field("filters"),
            fingerprint: o.get_field("fingerprint"),
            id: o.get_field("id"),
            include_public_key: o.get_field("includePublicKey"),
            key_name: o.get_field("keyName"),
            key_pair_id: o.get_field("keyPairId"),
            key_type: o.get_field("keyType"),
            public_key: o.get_field("publicKey"),
            tags: o.get_field("tags"),
        }
    }
}
