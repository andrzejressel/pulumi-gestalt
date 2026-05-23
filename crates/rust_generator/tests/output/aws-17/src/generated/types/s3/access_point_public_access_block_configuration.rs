#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AccessPointPublicAccessBlockConfiguration {
    /// Whether Amazon S3 should block public ACLs for buckets in this account. Defaults to `true`. Enabling this setting does not affect existing policies or ACLs. When set to `true` causes the following behavior:
    /// * PUT Bucket acl and PUT Object acl calls fail if the specified ACL is public.
    /// * PUT Object calls fail if the request includes a public ACL.
    /// * PUT Bucket calls fail if the request includes a public ACL.
    #[builder(into)]
    pub r#block_public_acls: Option<bool>,
    /// Whether Amazon S3 should block public bucket policies for buckets in this account. Defaults to `true`. Enabling this setting does not affect existing bucket policies. When set to `true` causes Amazon S3 to:
    /// * Reject calls to PUT Bucket policy if the specified bucket policy allows public access.
    #[builder(into)]
    pub r#block_public_policy: Option<bool>,
    /// Whether Amazon S3 should ignore public ACLs for buckets in this account. Defaults to `true`. Enabling this setting does not affect the persistence of any existing ACLs and doesn't prevent new public ACLs from being set. When set to `true` causes Amazon S3 to:
    /// * Ignore all public ACLs on buckets in this account and any objects that they contain.
    #[builder(into)]
    pub r#ignore_public_acls: Option<bool>,
    /// Whether Amazon S3 should restrict public bucket policies for buckets in this account. Defaults to `true`. Enabling this setting does not affect previously stored bucket policies, except that public and cross-account access within any public bucket policy, including non-public delegation to specific accounts, is blocked. When set to `true`:
    /// * Only the bucket owner and AWS Services can access buckets with public policies.
    #[builder(into)]
    pub r#restrict_public_buckets: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for AccessPointPublicAccessBlockConfiguration {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > + Send {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "blockPublicAcls",
                    &self.r#block_public_acls,
                ),
                to_pulumi_object_field(
                    "blockPublicPolicy",
                    &self.r#block_public_policy,
                ),
                to_pulumi_object_field(
                    "ignorePublicAcls",
                    &self.r#ignore_public_acls,
                ),
                to_pulumi_object_field(
                    "restrictPublicBuckets",
                    &self.r#restrict_public_buckets,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for AccessPointPublicAccessBlockConfiguration {
    fn from_pulumi_value(
        value: &pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    ) -> pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::rootcause::Result<Self> {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValueContent;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::rootcause::bail;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue;

        match value.content {
            PulumiValueContent::Object(ref _obj) => {
                use std::collections::BTreeMap;
                let fields_map: BTreeMap<String, PulumiValue> =
                    _obj.iter().cloned().collect();

                Ok(Self {
                    r#block_public_acls: {
                        let field_value = match fields_map.get("blockPublicAcls") {
                            Some(value) => value,
                            None => bail!("Missing field 'blockPublicAcls' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#block_public_policy: {
                        let field_value = match fields_map.get("blockPublicPolicy") {
                            Some(value) => value,
                            None => bail!("Missing field 'blockPublicPolicy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ignore_public_acls: {
                        let field_value = match fields_map.get("ignorePublicAcls") {
                            Some(value) => value,
                            None => bail!("Missing field 'ignorePublicAcls' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#restrict_public_buckets: {
                        let field_value = match fields_map.get("restrictPublicBuckets") {
                            Some(value) => value,
                            None => bail!("Missing field 'restrictPublicBuckets' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
