#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct BatchEnvironmentConfigExecutionConfig {
    /// The Cloud KMS key to use for encryption.
    #[builder(into)]
    #[serde(rename = "kmsKey")]
    pub r#kms_key: Option<String>,
    /// Tags used for network traffic control.
    #[builder(into)]
    #[serde(rename = "networkTags")]
    pub r#network_tags: Option<Vec<String>>,
    /// Network configuration for workload execution.
    #[builder(into)]
    #[serde(rename = "networkUri")]
    pub r#network_uri: Option<String>,
    /// Service account that used to execute workload.
    #[builder(into)]
    #[serde(rename = "serviceAccount")]
    pub r#service_account: Option<String>,
    /// A Cloud Storage bucket used to stage workload dependencies, config files, and store
    /// workload output and other ephemeral data, such as Spark history files. If you do not specify a staging bucket,
    /// Cloud Dataproc will determine a Cloud Storage location according to the region where your workload is running,
    /// and then create and manage project-level, per-location staging and temporary buckets.
    /// This field requires a Cloud Storage bucket name, not a gs://... URI to a Cloud Storage bucket.
    #[builder(into)]
    #[serde(rename = "stagingBucket")]
    pub r#staging_bucket: Option<String>,
    /// Subnetwork configuration for workload execution.
    #[builder(into)]
    #[serde(rename = "subnetworkUri")]
    pub r#subnetwork_uri: Option<String>,
    /// The duration after which the workload will be terminated.
    /// When the workload exceeds this duration, it will be unconditionally terminated without waiting for ongoing
    /// work to finish. If ttl is not specified for a batch workload, the workload will be allowed to run until it
    /// exits naturally (or run forever without exiting). If ttl is not specified for an interactive session,
    /// it defaults to 24 hours. If ttl is not specified for a batch that uses 2.1+ runtime version, it defaults to 4 hours.
    /// Minimum value is 10 minutes; maximum value is 14 days. If both ttl and idleTtl are specified (for an interactive session),
    /// the conditions are treated as OR conditions: the workload will be terminated when it has been idle for idleTtl or
    /// when ttl has been exceeded, whichever occurs first.
    #[builder(into)]
    #[serde(rename = "ttl")]
    pub r#ttl: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for BatchEnvironmentConfigExecutionConfig {
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
                    "kms_key",
                    &self.r#kms_key,
                ),
                to_pulumi_object_field(
                    "network_tags",
                    &self.r#network_tags,
                ),
                to_pulumi_object_field(
                    "network_uri",
                    &self.r#network_uri,
                ),
                to_pulumi_object_field(
                    "service_account",
                    &self.r#service_account,
                ),
                to_pulumi_object_field(
                    "staging_bucket",
                    &self.r#staging_bucket,
                ),
                to_pulumi_object_field(
                    "subnetwork_uri",
                    &self.r#subnetwork_uri,
                ),
                to_pulumi_object_field(
                    "ttl",
                    &self.r#ttl,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for BatchEnvironmentConfigExecutionConfig {
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
                    r#kms_key: {
                        let field_value = match fields_map.get("kms_key") {
                            Some(value) => value,
                            None => bail!("Missing field 'kms_key' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#network_tags: {
                        let field_value = match fields_map.get("network_tags") {
                            Some(value) => value,
                            None => bail!("Missing field 'network_tags' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#network_uri: {
                        let field_value = match fields_map.get("network_uri") {
                            Some(value) => value,
                            None => bail!("Missing field 'network_uri' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#service_account: {
                        let field_value = match fields_map.get("service_account") {
                            Some(value) => value,
                            None => bail!("Missing field 'service_account' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#staging_bucket: {
                        let field_value = match fields_map.get("staging_bucket") {
                            Some(value) => value,
                            None => bail!("Missing field 'staging_bucket' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#subnetwork_uri: {
                        let field_value = match fields_map.get("subnetwork_uri") {
                            Some(value) => value,
                            None => bail!("Missing field 'subnetwork_uri' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ttl: {
                        let field_value = match fields_map.get("ttl") {
                            Some(value) => value,
                            None => bail!("Missing field 'ttl' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
