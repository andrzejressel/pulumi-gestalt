#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct MultiRegionAccessPointDetailsPublicAccessBlock {
    #[builder(into)]
    #[serde(rename = "blockPublicAcls")]
    pub r#block_public_acls: Option<bool>,
    #[builder(into)]
    #[serde(rename = "blockPublicPolicy")]
    pub r#block_public_policy: Option<bool>,
    #[builder(into)]
    #[serde(rename = "ignorePublicAcls")]
    pub r#ignore_public_acls: Option<bool>,
    #[builder(into)]
    #[serde(rename = "restrictPublicBuckets")]
    pub r#restrict_public_buckets: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for MultiRegionAccessPointDetailsPublicAccessBlock {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("block_public_acls".to_string(), self.r#block_public_acls.to_pulumi_value().await);
            map.insert("block_public_policy".to_string(), self.r#block_public_policy.to_pulumi_value().await);
            map.insert("ignore_public_acls".to_string(), self.r#ignore_public_acls.to_pulumi_value().await);
            map.insert("restrict_public_buckets".to_string(), self.r#restrict_public_buckets.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for MultiRegionAccessPointDetailsPublicAccessBlock {
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
                    r#block_public_acls: {
                        let field_value = match fields_map.get("block_public_acls") {
                            Some(value) => value,
                            None => bail!("Missing field 'block_public_acls' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<bool> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#block_public_policy: {
                        let field_value = match fields_map.get("block_public_policy") {
                            Some(value) => value,
                            None => bail!("Missing field 'block_public_policy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<bool> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#ignore_public_acls: {
                        let field_value = match fields_map.get("ignore_public_acls") {
                            Some(value) => value,
                            None => bail!("Missing field 'ignore_public_acls' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<bool> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#restrict_public_buckets: {
                        let field_value = match fields_map.get("restrict_public_buckets") {
                            Some(value) => value,
                            None => bail!("Missing field 'restrict_public_buckets' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<bool> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
