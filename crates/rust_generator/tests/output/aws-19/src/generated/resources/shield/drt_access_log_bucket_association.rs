/// Resource for managing an AWS Shield DRT Access Log Bucket Association.
/// Up to 10 log buckets can be associated for DRT Access sharing with the Shield Response Team (SRT).
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
///     let test = drt_access_role_arn_association::create(
///         "test",
///         DrtAccessRoleArnAssociationArgs::builder()
///             .role_arn(
///                 "arn:aws:iam:${current.name}:${currentAwsCallerIdentity.accountId}:${shieldDrtAccessRoleName}",
///             )
///             .build_struct(),
///     );
///     let testDrtAccessLogBucketAssociation = drt_access_log_bucket_association::create(
///         "testDrtAccessLogBucketAssociation",
///         DrtAccessLogBucketAssociationArgs::builder()
///             .log_bucket("${shieldDrtAccessLogBucket}")
///             .role_arn_association_id("${test.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Shield DRT access log bucket associations using the `log_bucket`. For example:
///
/// ```sh
/// $ pulumi import aws:shield/drtAccessLogBucketAssociation:DrtAccessLogBucketAssociation example example-bucket
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod drt_access_log_bucket_association {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DrtAccessLogBucketAssociationArgs {
        /// The Amazon S3 bucket that contains the logs that you want to share.
        #[builder(into)]
        pub log_bucket: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the Role Arn association used for allowing Shield DRT Access.
        #[builder(into)]
        pub role_arn_association_id: pulumi_gestalt_rust::InputOrOutput<String>,
        #[builder(into, default)]
        pub timeouts: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::shield::DrtAccessLogBucketAssociationTimeouts>,
        >,
    }
    #[allow(dead_code)]
    pub struct DrtAccessLogBucketAssociationResult {
        /// The Amazon S3 bucket that contains the logs that you want to share.
        pub log_bucket: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Role Arn association used for allowing Shield DRT Access.
        pub role_arn_association_id: pulumi_gestalt_rust::Output<String>,
        pub timeouts: pulumi_gestalt_rust::Output<
            Option<super::super::types::shield::DrtAccessLogBucketAssociationTimeouts>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: DrtAccessLogBucketAssociationArgs,
    ) -> DrtAccessLogBucketAssociationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let log_bucket_binding = args.log_bucket.get_output(context);
        let role_arn_association_id_binding = args
            .role_arn_association_id
            .get_output(context);
        let timeouts_binding = args.timeouts.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:shield/drtAccessLogBucketAssociation:DrtAccessLogBucketAssociation"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "logBucket".into(),
                    value: &log_bucket_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "roleArnAssociationId".into(),
                    value: &role_arn_association_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "timeouts".into(),
                    value: &timeouts_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        DrtAccessLogBucketAssociationResult {
            log_bucket: o.get_field("logBucket"),
            role_arn_association_id: o.get_field("roleArnAssociationId"),
            timeouts: o.get_field("timeouts"),
        }
    }
}
