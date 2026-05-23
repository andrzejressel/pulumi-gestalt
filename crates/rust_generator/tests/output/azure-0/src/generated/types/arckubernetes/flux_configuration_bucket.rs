#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FluxConfigurationBucket {
    /// Specifies the plaintext access key used to securely access the S3 bucket.
    #[builder(into)]
    pub r#access_key: Option<String>,
    /// Specifies the bucket name to sync from the url endpoint for the flux configuration.
    #[builder(into)]
    pub r#bucket_name: String,
    /// Specifies the name of a local secret on the Kubernetes cluster to use as the authentication secret rather than the managed or user-provided configuration secrets.
    #[builder(into)]
    pub r#local_auth_reference: Option<String>,
    /// Specifies the Base64-encoded secret key used to authenticate with the bucket source.
    #[builder(into)]
    pub r#secret_key_base_64: Option<String>,
    /// Specifies the interval at which to re-reconcile the cluster git repository source with the remote. Defaults to `600`.
    #[builder(into)]
    pub r#sync_interval_in_seconds: Option<i32>,
    /// Specifies the maximum time to attempt to reconcile the cluster git repository source with the remote. Defaults to `600`.
    #[builder(into)]
    pub r#timeout_in_seconds: Option<i32>,
    /// Specify whether to communicate with a bucket using TLS is enabled. Defaults to `true`.
    #[builder(into)]
    pub r#tls_enabled: Option<bool>,
    /// Specifies the URL to sync for the flux configuration S3 bucket. It must start with `http://` or `https://`.
    #[builder(into)]
    pub r#url: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for FluxConfigurationBucket {
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
                    "accessKey",
                    &self.r#access_key,
                ),
                to_pulumi_object_field(
                    "bucketName",
                    &self.r#bucket_name,
                ),
                to_pulumi_object_field(
                    "localAuthReference",
                    &self.r#local_auth_reference,
                ),
                to_pulumi_object_field(
                    "secretKeyBase64",
                    &self.r#secret_key_base_64,
                ),
                to_pulumi_object_field(
                    "syncIntervalInSeconds",
                    &self.r#sync_interval_in_seconds,
                ),
                to_pulumi_object_field(
                    "timeoutInSeconds",
                    &self.r#timeout_in_seconds,
                ),
                to_pulumi_object_field(
                    "tlsEnabled",
                    &self.r#tls_enabled,
                ),
                to_pulumi_object_field(
                    "url",
                    &self.r#url,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for FluxConfigurationBucket {
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
                        let field_value = match fields_map.get("accessKey") {
                            Some(value) => value,
                            None => bail!("Missing field 'accessKey' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#bucket_name: {
                        let field_value = match fields_map.get("bucketName") {
                            Some(value) => value,
                            None => bail!("Missing field 'bucketName' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#local_auth_reference: {
                        let field_value = match fields_map.get("localAuthReference") {
                            Some(value) => value,
                            None => bail!("Missing field 'localAuthReference' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#secret_key_base_64: {
                        let field_value = match fields_map.get("secretKeyBase64") {
                            Some(value) => value,
                            None => bail!("Missing field 'secretKeyBase64' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#sync_interval_in_seconds: {
                        let field_value = match fields_map.get("syncIntervalInSeconds") {
                            Some(value) => value,
                            None => bail!("Missing field 'syncIntervalInSeconds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#timeout_in_seconds: {
                        let field_value = match fields_map.get("timeoutInSeconds") {
                            Some(value) => value,
                            None => bail!("Missing field 'timeoutInSeconds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#tls_enabled: {
                        let field_value = match fields_map.get("tlsEnabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'tlsEnabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#url: {
                        let field_value = match fields_map.get("url") {
                            Some(value) => value,
                            None => bail!("Missing field 'url' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
