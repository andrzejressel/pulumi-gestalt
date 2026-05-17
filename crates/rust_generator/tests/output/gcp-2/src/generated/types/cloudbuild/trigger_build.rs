#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct TriggerBuild {
    /// Artifacts produced by the build that should be uploaded upon successful completion of all build steps.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "artifacts")]
    pub r#artifacts: Option<Box<super::super::types::cloudbuild::TriggerBuildArtifacts>>,
    /// Secrets and secret environment variables.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "availableSecrets")]
    pub r#available_secrets: Option<Box<super::super::types::cloudbuild::TriggerBuildAvailableSecrets>>,
    /// A list of images to be pushed upon the successful completion of all build steps.
    /// The images are pushed using the builder service account's credentials.
    /// The digests of the pushed images will be stored in the Build resource's results field.
    /// If any of the images fail to be pushed, the build status is marked FAILURE.
    #[builder(into)]
    #[serde(rename = "images")]
    pub r#images: Option<Vec<String>>,
    /// Google Cloud Storage bucket where logs should be written.
    /// Logs file names will be of the format ${logsBucket}/log-${build_id}.txt.
    #[builder(into)]
    #[serde(rename = "logsBucket")]
    pub r#logs_bucket: Option<String>,
    /// Special options for this build.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "options")]
    pub r#options: Option<Box<super::super::types::cloudbuild::TriggerBuildOptions>>,
    /// TTL in queue for this build. If provided and the build is enqueued longer than this value,
    /// the build will expire and the build status will be EXPIRED.
    /// The TTL starts ticking from createTime.
    /// A duration in seconds with up to nine fractional digits, terminated by 's'. Example: "3.5s".
    #[builder(into)]
    #[serde(rename = "queueTtl")]
    pub r#queue_ttl: Option<String>,
    /// Secrets to decrypt using Cloud Key Management Service.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "secrets")]
    pub r#secrets: Option<Vec<super::super::types::cloudbuild::TriggerBuildSecret>>,
    /// The location of the source files to build.
    /// One of `storageSource` or `repoSource` must be provided.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "source")]
    pub r#source: Option<Box<super::super::types::cloudbuild::TriggerBuildSource>>,
    /// The operations to be performed on the workspace.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "steps")]
    pub r#steps: Vec<super::super::types::cloudbuild::TriggerBuildStep>,
    /// Substitutions data for Build resource.
    #[builder(into)]
    #[serde(rename = "substitutions")]
    pub r#substitutions: Option<std::collections::HashMap<String, String>>,
    /// Tags for annotation of a Build. These are not docker tags.
    #[builder(into)]
    #[serde(rename = "tags")]
    pub r#tags: Option<Vec<String>>,
    /// Amount of time that this build should be allowed to run, to second granularity.
    /// If this amount of time elapses, work on the build will cease and the build status will be TIMEOUT.
    /// This timeout must be equal to or greater than the sum of the timeouts for build steps within the build.
    /// The expected format is the number of seconds followed by s.
    /// Default time is ten minutes (600s).
    #[builder(into)]
    #[serde(rename = "timeout")]
    pub r#timeout: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for TriggerBuild {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_concurrent, to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "artifacts",
                    &self.r#artifacts,
                ),
                to_pulumi_object_field(
                    "available_secrets",
                    &self.r#available_secrets,
                ),
                to_pulumi_object_field(
                    "images",
                    &self.r#images,
                ),
                to_pulumi_object_field(
                    "logs_bucket",
                    &self.r#logs_bucket,
                ),
                to_pulumi_object_field(
                    "options",
                    &self.r#options,
                ),
                to_pulumi_object_field(
                    "queue_ttl",
                    &self.r#queue_ttl,
                ),
                to_pulumi_object_field(
                    "secrets",
                    &self.r#secrets,
                ),
                to_pulumi_object_field(
                    "source",
                    &self.r#source,
                ),
                to_pulumi_object_field(
                    "steps",
                    &self.r#steps,
                ),
                to_pulumi_object_field(
                    "substitutions",
                    &self.r#substitutions,
                ),
                to_pulumi_object_field(
                    "tags",
                    &self.r#tags,
                ),
                to_pulumi_object_field(
                    "timeout",
                    &self.r#timeout,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for TriggerBuild {
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
                    r#artifacts: {
                        let field_value = match fields_map.get("artifacts") {
                            Some(value) => value,
                            None => bail!("Missing field 'artifacts' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#available_secrets: {
                        let field_value = match fields_map.get("available_secrets") {
                            Some(value) => value,
                            None => bail!("Missing field 'available_secrets' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#images: {
                        let field_value = match fields_map.get("images") {
                            Some(value) => value,
                            None => bail!("Missing field 'images' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#logs_bucket: {
                        let field_value = match fields_map.get("logs_bucket") {
                            Some(value) => value,
                            None => bail!("Missing field 'logs_bucket' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#options: {
                        let field_value = match fields_map.get("options") {
                            Some(value) => value,
                            None => bail!("Missing field 'options' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#queue_ttl: {
                        let field_value = match fields_map.get("queue_ttl") {
                            Some(value) => value,
                            None => bail!("Missing field 'queue_ttl' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#secrets: {
                        let field_value = match fields_map.get("secrets") {
                            Some(value) => value,
                            None => bail!("Missing field 'secrets' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#source: {
                        let field_value = match fields_map.get("source") {
                            Some(value) => value,
                            None => bail!("Missing field 'source' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#steps: {
                        let field_value = match fields_map.get("steps") {
                            Some(value) => value,
                            None => bail!("Missing field 'steps' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#substitutions: {
                        let field_value = match fields_map.get("substitutions") {
                            Some(value) => value,
                            None => bail!("Missing field 'substitutions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#tags: {
                        let field_value = match fields_map.get("tags") {
                            Some(value) => value,
                            None => bail!("Missing field 'tags' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#timeout: {
                        let field_value = match fields_map.get("timeout") {
                            Some(value) => value,
                            None => bail!("Missing field 'timeout' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
