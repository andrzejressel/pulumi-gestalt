#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetTriggerBuild {
    /// Artifacts produced by the build that should be uploaded upon successful completion of all build steps.
    #[builder(into)]
    pub r#artifacts: Vec<super::super::types::cloudbuild::GetTriggerBuildArtifact>,
    /// Secrets and secret environment variables.
    #[builder(into)]
    pub r#available_secrets: Vec<super::super::types::cloudbuild::GetTriggerBuildAvailableSecret>,
    /// A list of images to be pushed upon the successful completion of all build steps.
    /// The images are pushed using the builder service account's credentials.
    /// The digests of the pushed images will be stored in the Build resource's results field.
    /// If any of the images fail to be pushed, the build status is marked FAILURE.
    #[builder(into)]
    pub r#images: Vec<String>,
    /// Google Cloud Storage bucket where logs should be written.
    /// Logs file names will be of the format ${logsBucket}/log-${build_id}.txt.
    #[builder(into)]
    pub r#logs_bucket: String,
    /// Special options for this build.
    #[builder(into)]
    pub r#options: Vec<super::super::types::cloudbuild::GetTriggerBuildOption>,
    /// TTL in queue for this build. If provided and the build is enqueued longer than this value,
    /// the build will expire and the build status will be EXPIRED.
    /// The TTL starts ticking from createTime.
    /// A duration in seconds with up to nine fractional digits, terminated by 's'. Example: "3.5s".
    #[builder(into)]
    pub r#queue_ttl: String,
    /// Secrets to decrypt using Cloud Key Management Service.
    #[builder(into)]
    pub r#secrets: Vec<super::super::types::cloudbuild::GetTriggerBuildSecret>,
    /// The location of the source files to build.
    /// 
    /// One of 'storageSource' or 'repoSource' must be provided.
    #[builder(into)]
    pub r#sources: Vec<super::super::types::cloudbuild::GetTriggerBuildSource>,
    /// The operations to be performed on the workspace.
    #[builder(into)]
    pub r#steps: Vec<super::super::types::cloudbuild::GetTriggerBuildStep>,
    /// Substitutions data for Build resource.
    #[builder(into)]
    pub r#substitutions: std::collections::HashMap<String, String>,
    /// Tags for annotation of a Build. These are not docker tags.
    #[builder(into)]
    pub r#tags: Vec<String>,
    /// Amount of time that this build should be allowed to run, to second granularity.
    /// If this amount of time elapses, work on the build will cease and the build status will be TIMEOUT.
    /// This timeout must be equal to or greater than the sum of the timeouts for build steps within the build.
    /// The expected format is the number of seconds followed by s.
    /// Default time is ten minutes (600s).
    #[builder(into)]
    pub r#timeout: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetTriggerBuild {
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
                    "artifacts",
                    &self.r#artifacts,
                ),
                to_pulumi_object_field(
                    "availableSecrets",
                    &self.r#available_secrets,
                ),
                to_pulumi_object_field(
                    "images",
                    &self.r#images,
                ),
                to_pulumi_object_field(
                    "logsBucket",
                    &self.r#logs_bucket,
                ),
                to_pulumi_object_field(
                    "options",
                    &self.r#options,
                ),
                to_pulumi_object_field(
                    "queueTtl",
                    &self.r#queue_ttl,
                ),
                to_pulumi_object_field(
                    "secrets",
                    &self.r#secrets,
                ),
                to_pulumi_object_field(
                    "sources",
                    &self.r#sources,
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
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetTriggerBuild {
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
                        let field_value = match fields_map.get("availableSecrets") {
                            Some(value) => value,
                            None => bail!("Missing field 'availableSecrets' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("logsBucket") {
                            Some(value) => value,
                            None => bail!("Missing field 'logsBucket' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("queueTtl") {
                            Some(value) => value,
                            None => bail!("Missing field 'queueTtl' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#sources: {
                        let field_value = match fields_map.get("sources") {
                            Some(value) => value,
                            None => bail!("Missing field 'sources' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
