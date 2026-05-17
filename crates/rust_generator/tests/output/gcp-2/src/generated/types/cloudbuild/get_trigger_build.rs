#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetTriggerBuild {
    /// Artifacts produced by the build that should be uploaded upon successful completion of all build steps.
    #[builder(into)]
    #[serde(rename = "artifacts")]
    pub r#artifacts: Vec<super::super::types::cloudbuild::GetTriggerBuildArtifact>,
    /// Secrets and secret environment variables.
    #[builder(into)]
    #[serde(rename = "availableSecrets")]
    pub r#available_secrets: Vec<super::super::types::cloudbuild::GetTriggerBuildAvailableSecret>,
    /// A list of images to be pushed upon the successful completion of all build steps.
    /// The images are pushed using the builder service account's credentials.
    /// The digests of the pushed images will be stored in the Build resource's results field.
    /// If any of the images fail to be pushed, the build status is marked FAILURE.
    #[builder(into)]
    #[serde(rename = "images")]
    pub r#images: Vec<String>,
    /// Google Cloud Storage bucket where logs should be written.
    /// Logs file names will be of the format ${logsBucket}/log-${build_id}.txt.
    #[builder(into)]
    #[serde(rename = "logsBucket")]
    pub r#logs_bucket: String,
    /// Special options for this build.
    #[builder(into)]
    #[serde(rename = "options")]
    pub r#options: Vec<super::super::types::cloudbuild::GetTriggerBuildOption>,
    /// TTL in queue for this build. If provided and the build is enqueued longer than this value,
    /// the build will expire and the build status will be EXPIRED.
    /// The TTL starts ticking from createTime.
    /// A duration in seconds with up to nine fractional digits, terminated by 's'. Example: "3.5s".
    #[builder(into)]
    #[serde(rename = "queueTtl")]
    pub r#queue_ttl: String,
    /// Secrets to decrypt using Cloud Key Management Service.
    #[builder(into)]
    #[serde(rename = "secrets")]
    pub r#secrets: Vec<super::super::types::cloudbuild::GetTriggerBuildSecret>,
    /// The location of the source files to build.
    /// 
    /// One of 'storageSource' or 'repoSource' must be provided.
    #[builder(into)]
    #[serde(rename = "sources")]
    pub r#sources: Vec<super::super::types::cloudbuild::GetTriggerBuildSource>,
    /// The operations to be performed on the workspace.
    #[builder(into)]
    #[serde(rename = "steps")]
    pub r#steps: Vec<super::super::types::cloudbuild::GetTriggerBuildStep>,
    /// Substitutions data for Build resource.
    #[builder(into)]
    #[serde(rename = "substitutions")]
    pub r#substitutions: std::collections::HashMap<String, String>,
    /// Tags for annotation of a Build. These are not docker tags.
    #[builder(into)]
    #[serde(rename = "tags")]
    pub r#tags: Vec<String>,
    /// Amount of time that this build should be allowed to run, to second granularity.
    /// If this amount of time elapses, work on the build will cease and the build status will be TIMEOUT.
    /// This timeout must be equal to or greater than the sum of the timeouts for build steps within the build.
    /// The expected format is the number of seconds followed by s.
    /// Default time is ten minutes (600s).
    #[builder(into)]
    #[serde(rename = "timeout")]
    pub r#timeout: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetTriggerBuild {
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
                "artifacts".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#artifacts,
                )
                .await,
            );
            map.insert(
                "available_secrets".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#available_secrets,
                )
                .await,
            );
            map.insert(
                "images".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#images,
                )
                .await,
            );
            map.insert(
                "logs_bucket".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#logs_bucket,
                )
                .await,
            );
            map.insert(
                "options".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#options,
                )
                .await,
            );
            map.insert(
                "queue_ttl".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#queue_ttl,
                )
                .await,
            );
            map.insert(
                "secrets".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#secrets,
                )
                .await,
            );
            map.insert(
                "sources".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#sources,
                )
                .await,
            );
            map.insert(
                "steps".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#steps,
                )
                .await,
            );
            map.insert(
                "substitutions".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#substitutions,
                )
                .await,
            );
            map.insert(
                "tags".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#tags,
                )
                .await,
            );
            map.insert(
                "timeout".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#timeout,
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
