#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct BucketAclV2AccessControlPolicy {
    /// Set of `grant` configuration blocks. See below.
    #[builder(into)]
    #[serde(rename = "grants")]
    pub r#grants: Option<Vec<super::super::types::s3::BucketAclV2AccessControlPolicyGrant>>,
    /// Configuration block for the bucket owner's display name and ID. See below.
    #[builder(into)]
    #[serde(rename = "owner")]
    pub r#owner: Box<super::super::types::s3::BucketAclV2AccessControlPolicyOwner>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for BucketAclV2AccessControlPolicy {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("grants".to_string(), self.r#grants.to_pulumi_value().await);
            map.insert("owner".to_string(), self.r#owner.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for BucketAclV2AccessControlPolicy {
    fn from_pulumi_value(
        value: &pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    ) -> pulumi_gestalt_rust::__private::rootcause::Result<Self> {
        use std::collections::BTreeMap;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValueContent;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue;
        use pulumi_gestalt_rust::__private::rootcause::bail;

        match value.content {
            PulumiValueContent::Object(ref obj) => {
                let fields_map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> =
                    obj.iter().cloned().collect();

                Ok(Self {
                    r#grants: {
                        let field_value = match fields_map.get("grants") {
                            Some(value) => value,
                            None => bail!("Missing field 'grants' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<super::super::types::s3::BucketAclV2AccessControlPolicyGrant>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#owner: {
                        let field_value = match fields_map.get("owner") {
                            Some(value) => value,
                            None => bail!("Missing field 'owner' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Box<super::super::types::s3::BucketAclV2AccessControlPolicyOwner> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
