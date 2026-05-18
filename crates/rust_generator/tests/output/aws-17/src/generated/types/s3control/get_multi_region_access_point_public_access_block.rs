#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetMultiRegionAccessPointPublicAccessBlock {
    /// Specifies whether Amazon S3 should block public access control lists (ACLs). When set to `true` causes the following behavior:
    /// * PUT Bucket acl and PUT Object acl calls fail if the specified ACL is public.
    /// * PUT Object calls fail if the request includes a public ACL.
    /// * PUT Bucket calls fail if the request includes a public ACL.
    #[builder(into)]
    #[serde(rename = "blockPublicAcls")]
    pub r#block_public_acls: bool,
    /// Specifies whether Amazon S3 should block public bucket policies for buckets in this account. When set to `true` causes Amazon S3 to:
    /// * Reject calls to PUT Bucket policy if the specified bucket policy allows public access.
    #[builder(into)]
    #[serde(rename = "blockPublicPolicy")]
    pub r#block_public_policy: bool,
    /// Specifies whether Amazon S3 should ignore public ACLs for buckets in this account. When set to `true` causes Amazon S3 to:
    /// * Ignore all public ACLs on buckets in this account and any objects that they contain.
    #[builder(into)]
    #[serde(rename = "ignorePublicAcls")]
    pub r#ignore_public_acls: bool,
    /// Specifies whether Amazon S3 should restrict public bucket policies for buckets in this account. When set to `true`:
    /// * Only the bucket owner and AWS Services can access buckets with public policies.
    #[builder(into)]
    #[serde(rename = "restrictPublicBuckets")]
    pub r#restrict_public_buckets: bool,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetMultiRegionAccessPointPublicAccessBlock {
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
                    "block_public_acls",
                    &self.r#block_public_acls,
                ),
                to_pulumi_object_field(
                    "block_public_policy",
                    &self.r#block_public_policy,
                ),
                to_pulumi_object_field(
                    "ignore_public_acls",
                    &self.r#ignore_public_acls,
                ),
                to_pulumi_object_field(
                    "restrict_public_buckets",
                    &self.r#restrict_public_buckets,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetMultiRegionAccessPointPublicAccessBlock {
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
                        let field_value = match fields_map.get("block_public_acls") {
                            Some(value) => value,
                            None => bail!("Missing field 'block_public_acls' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#block_public_policy: {
                        let field_value = match fields_map.get("block_public_policy") {
                            Some(value) => value,
                            None => bail!("Missing field 'block_public_policy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ignore_public_acls: {
                        let field_value = match fields_map.get("ignore_public_acls") {
                            Some(value) => value,
                            None => bail!("Missing field 'ignore_public_acls' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#restrict_public_buckets: {
                        let field_value = match fields_map.get("restrict_public_buckets") {
                            Some(value) => value,
                            None => bail!("Missing field 'restrict_public_buckets' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
