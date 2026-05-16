#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ProjectArtifacts {
    /// Artifact identifier. Must be the same specified inside the AWS CodeBuild build specification.
    #[builder(into)]
    #[serde(rename = "artifactIdentifier")]
    pub r#artifact_identifier: Option<String>,
    /// Specifies the bucket owner's access for objects that another account uploads to their Amazon S3 bucket. By default, only the account that uploads the objects to the bucket has access to these objects. This property allows you to give the bucket owner access to these objects. Valid values are `NONE`, `READ_ONLY`, and `FULL`. your CodeBuild service role must have the `s3:PutBucketAcl` permission. This permission allows CodeBuild to modify the access control list for the bucket.
    #[builder(into)]
    #[serde(rename = "bucketOwnerAccess")]
    pub r#bucket_owner_access: Option<String>,
    /// Whether to disable encrypting output artifacts. If `type` is set to `NO_ARTIFACTS`, this value is ignored. Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "encryptionDisabled")]
    pub r#encryption_disabled: Option<bool>,
    /// Information about the build output artifact location. If `type` is set to `CODEPIPELINE` or `NO_ARTIFACTS`, this value is ignored. If `type` is set to `S3`, this is the name of the output bucket.
    #[builder(into)]
    #[serde(rename = "location")]
    pub r#location: Option<String>,
    /// Name of the project. If `type` is set to `S3`, this is the name of the output artifact object
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// Namespace to use in storing build artifacts. If `type` is set to `S3`, then valid values are `BUILD_ID`, `NONE`.
    #[builder(into)]
    #[serde(rename = "namespaceType")]
    pub r#namespace_type: Option<String>,
    /// Whether a name specified in the build specification overrides the artifact name.
    #[builder(into)]
    #[serde(rename = "overrideArtifactName")]
    pub r#override_artifact_name: Option<bool>,
    /// Type of build output artifact to create. If `type` is set to `S3`, valid values are `NONE`, `ZIP`
    #[builder(into)]
    #[serde(rename = "packaging")]
    pub r#packaging: Option<String>,
    /// If `type` is set to `S3`, this is the path to the output artifact.
    #[builder(into)]
    #[serde(rename = "path")]
    pub r#path: Option<String>,
    /// Build output artifact's type. Valid values: `CODEPIPELINE`, `NO_ARTIFACTS`, `S3`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ProjectArtifacts {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("artifact_identifier".to_string(), self.r#artifact_identifier.to_pulumi_value().await);
            map.insert("bucket_owner_access".to_string(), self.r#bucket_owner_access.to_pulumi_value().await);
            map.insert("encryption_disabled".to_string(), self.r#encryption_disabled.to_pulumi_value().await);
            map.insert("location".to_string(), self.r#location.to_pulumi_value().await);
            map.insert("name".to_string(), self.r#name.to_pulumi_value().await);
            map.insert("namespace_type".to_string(), self.r#namespace_type.to_pulumi_value().await);
            map.insert("override_artifact_name".to_string(), self.r#override_artifact_name.to_pulumi_value().await);
            map.insert("packaging".to_string(), self.r#packaging.to_pulumi_value().await);
            map.insert("path".to_string(), self.r#path.to_pulumi_value().await);
            map.insert("type_".to_string(), self.r#type_.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ProjectArtifacts {
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
                    r#artifact_identifier: {
                        let field_value = match fields_map.get("artifact_identifier") {
                            Some(value) => value,
                            None => bail!("Missing field 'artifact_identifier' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#bucket_owner_access: {
                        let field_value = match fields_map.get("bucket_owner_access") {
                            Some(value) => value,
                            None => bail!("Missing field 'bucket_owner_access' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#encryption_disabled: {
                        let field_value = match fields_map.get("encryption_disabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'encryption_disabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<bool> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#location: {
                        let field_value = match fields_map.get("location") {
                            Some(value) => value,
                            None => bail!("Missing field 'location' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#name: {
                        let field_value = match fields_map.get("name") {
                            Some(value) => value,
                            None => bail!("Missing field 'name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#namespace_type: {
                        let field_value = match fields_map.get("namespace_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'namespace_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#override_artifact_name: {
                        let field_value = match fields_map.get("override_artifact_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'override_artifact_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<bool> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#packaging: {
                        let field_value = match fields_map.get("packaging") {
                            Some(value) => value,
                            None => bail!("Missing field 'packaging' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#path: {
                        let field_value = match fields_map.get("path") {
                            Some(value) => value,
                            None => bail!("Missing field 'path' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#type_: {
                        let field_value = match fields_map.get("type_") {
                            Some(value) => value,
                            None => bail!("Missing field 'type_' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
