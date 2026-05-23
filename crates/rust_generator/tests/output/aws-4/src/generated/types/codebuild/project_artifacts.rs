#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ProjectArtifacts {
    /// Artifact identifier. Must be the same specified inside the AWS CodeBuild build specification.
    #[builder(into)]
    pub r#artifact_identifier: Option<String>,
    /// Specifies the bucket owner's access for objects that another account uploads to their Amazon S3 bucket. By default, only the account that uploads the objects to the bucket has access to these objects. This property allows you to give the bucket owner access to these objects. Valid values are `NONE`, `READ_ONLY`, and `FULL`. your CodeBuild service role must have the `s3:PutBucketAcl` permission. This permission allows CodeBuild to modify the access control list for the bucket.
    #[builder(into)]
    pub r#bucket_owner_access: Option<String>,
    /// Whether to disable encrypting output artifacts. If `type` is set to `NO_ARTIFACTS`, this value is ignored. Defaults to `false`.
    #[builder(into)]
    pub r#encryption_disabled: Option<bool>,
    /// Information about the build output artifact location. If `type` is set to `CODEPIPELINE` or `NO_ARTIFACTS`, this value is ignored. If `type` is set to `S3`, this is the name of the output bucket.
    #[builder(into)]
    pub r#location: Option<String>,
    /// Name of the project. If `type` is set to `S3`, this is the name of the output artifact object
    #[builder(into)]
    pub r#name: Option<String>,
    /// Namespace to use in storing build artifacts. If `type` is set to `S3`, then valid values are `BUILD_ID`, `NONE`.
    #[builder(into)]
    pub r#namespace_type: Option<String>,
    /// Whether a name specified in the build specification overrides the artifact name.
    #[builder(into)]
    pub r#override_artifact_name: Option<bool>,
    /// Type of build output artifact to create. If `type` is set to `S3`, valid values are `NONE`, `ZIP`
    #[builder(into)]
    pub r#packaging: Option<String>,
    /// If `type` is set to `S3`, this is the path to the output artifact.
    #[builder(into)]
    pub r#path: Option<String>,
    /// Build output artifact's type. Valid values: `CODEPIPELINE`, `NO_ARTIFACTS`, `S3`.
    #[builder(into)]
    pub r#type_: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ProjectArtifacts {
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
                    "artifactIdentifier",
                    &self.r#artifact_identifier,
                ),
                to_pulumi_object_field(
                    "bucketOwnerAccess",
                    &self.r#bucket_owner_access,
                ),
                to_pulumi_object_field(
                    "encryptionDisabled",
                    &self.r#encryption_disabled,
                ),
                to_pulumi_object_field(
                    "location",
                    &self.r#location,
                ),
                to_pulumi_object_field(
                    "name",
                    &self.r#name,
                ),
                to_pulumi_object_field(
                    "namespaceType",
                    &self.r#namespace_type,
                ),
                to_pulumi_object_field(
                    "overrideArtifactName",
                    &self.r#override_artifact_name,
                ),
                to_pulumi_object_field(
                    "packaging",
                    &self.r#packaging,
                ),
                to_pulumi_object_field(
                    "path",
                    &self.r#path,
                ),
                to_pulumi_object_field(
                    "type",
                    &self.r#type_,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ProjectArtifacts {
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
                    r#artifact_identifier: {
                        let field_value = match fields_map.get("artifactIdentifier") {
                            Some(value) => value,
                            None => bail!("Missing field 'artifactIdentifier' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#bucket_owner_access: {
                        let field_value = match fields_map.get("bucketOwnerAccess") {
                            Some(value) => value,
                            None => bail!("Missing field 'bucketOwnerAccess' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#encryption_disabled: {
                        let field_value = match fields_map.get("encryptionDisabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'encryptionDisabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#location: {
                        let field_value = match fields_map.get("location") {
                            Some(value) => value,
                            None => bail!("Missing field 'location' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#name: {
                        let field_value = match fields_map.get("name") {
                            Some(value) => value,
                            None => bail!("Missing field 'name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#namespace_type: {
                        let field_value = match fields_map.get("namespaceType") {
                            Some(value) => value,
                            None => bail!("Missing field 'namespaceType' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#override_artifact_name: {
                        let field_value = match fields_map.get("overrideArtifactName") {
                            Some(value) => value,
                            None => bail!("Missing field 'overrideArtifactName' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#packaging: {
                        let field_value = match fields_map.get("packaging") {
                            Some(value) => value,
                            None => bail!("Missing field 'packaging' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#path: {
                        let field_value = match fields_map.get("path") {
                            Some(value) => value,
                            None => bail!("Missing field 'path' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#type_: {
                        let field_value = match fields_map.get("type") {
                            Some(value) => value,
                            None => bail!("Missing field 'type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
