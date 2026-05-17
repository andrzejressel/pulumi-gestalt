#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct EdgeCacheOriginAwsV4Authentication {
    /// The access key ID your origin uses to identify the key.
    #[builder(into)]
    #[serde(rename = "accessKeyId")]
    pub r#access_key_id: String,
    /// The name of the AWS region that your origin is in.
    #[builder(into)]
    #[serde(rename = "originRegion")]
    pub r#origin_region: String,
    /// The Secret Manager secret version of the secret access key used by your origin.
    /// 
    /// This is the resource name of the secret version in the format 'projects/*/secrets/*/versions/*' where the '*' values are replaced by the project, secret, and version you require.
    #[builder(into)]
    #[serde(rename = "secretAccessKeyVersion")]
    pub r#secret_access_key_version: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for EdgeCacheOriginAwsV4Authentication {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "access_key_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#access_key_id,
                )
                .await,
            );
            map.insert(
                "origin_region".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#origin_region,
                )
                .await,
            );
            map.insert(
                "secret_access_key_version".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#secret_access_key_version,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for EdgeCacheOriginAwsV4Authentication {
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
                    r#access_key_id: {
                        let field_value = match fields_map.get("access_key_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'access_key_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#origin_region: {
                        let field_value = match fields_map.get("origin_region") {
                            Some(value) => value,
                            None => bail!("Missing field 'origin_region' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#secret_access_key_version: {
                        let field_value = match fields_map.get("secret_access_key_version") {
                            Some(value) => value,
                            None => bail!("Missing field 'secret_access_key_version' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
