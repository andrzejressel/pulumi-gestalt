#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct BackendServiceSecuritySettingsAwsV4Authentication {
    /// The access key used for s3 bucket authentication.
    /// Required for updating or creating a backend that uses AWS v4 signature authentication, but will not be returned as part of the configuration when queried with a REST API GET request.
    #[builder(into)]
    #[serde(rename = "accessKey")]
    pub r#access_key: Option<String>,
    /// The identifier of an access key used for s3 bucket authentication.
    #[builder(into)]
    #[serde(rename = "accessKeyId")]
    pub r#access_key_id: Option<String>,
    /// The optional version identifier for the access key. You can use this to keep track of different iterations of your access key.
    #[builder(into)]
    #[serde(rename = "accessKeyVersion")]
    pub r#access_key_version: Option<String>,
    /// The name of the cloud region of your origin. This is a free-form field with the name of the region your cloud uses to host your origin.
    /// For example, "us-east-1" for AWS or "us-ashburn-1" for OCI.
    #[builder(into)]
    #[serde(rename = "originRegion")]
    pub r#origin_region: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for BackendServiceSecuritySettingsAwsV4Authentication {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "access_key",
                    &self.r#access_key,
                ),
                to_pulumi_object_field(
                    "access_key_id",
                    &self.r#access_key_id,
                ),
                to_pulumi_object_field(
                    "access_key_version",
                    &self.r#access_key_version,
                ),
                to_pulumi_object_field(
                    "origin_region",
                    &self.r#origin_region,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for BackendServiceSecuritySettingsAwsV4Authentication {
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
                    r#access_key: {
                        let field_value = match fields_map.get("access_key") {
                            Some(value) => value,
                            None => bail!("Missing field 'access_key' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#access_key_id: {
                        let field_value = match fields_map.get("access_key_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'access_key_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#access_key_version: {
                        let field_value = match fields_map.get("access_key_version") {
                            Some(value) => value,
                            None => bail!("Missing field 'access_key_version' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
