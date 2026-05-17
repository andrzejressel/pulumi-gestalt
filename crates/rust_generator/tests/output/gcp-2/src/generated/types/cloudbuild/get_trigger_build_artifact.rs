#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetTriggerBuildArtifact {
    /// A list of images to be pushed upon the successful completion of all build steps.
    /// 
    /// The images will be pushed using the builder service account's credentials.
    /// 
    /// The digests of the pushed images will be stored in the Build resource's results field.
    /// 
    /// If any of the images fail to be pushed, the build is marked FAILURE.
    #[builder(into)]
    #[serde(rename = "images")]
    pub r#images: Vec<String>,
    /// A Maven artifact to upload to Artifact Registry upon successful completion of all build steps.
    /// 
    /// The location and generation of the uploaded objects will be stored in the Build resource's results field.
    /// 
    /// If any objects fail to be pushed, the build is marked FAILURE.
    #[builder(into)]
    #[serde(rename = "mavenArtifacts")]
    pub r#maven_artifacts: Vec<super::super::types::cloudbuild::GetTriggerBuildArtifactMavenArtifact>,
    /// Npm package to upload to Artifact Registry upon successful completion of all build steps.
    /// 
    /// The location and generation of the uploaded objects will be stored in the Build resource's results field.
    /// 
    /// If any objects fail to be pushed, the build is marked FAILURE.
    #[builder(into)]
    #[serde(rename = "npmPackages")]
    pub r#npm_packages: Vec<super::super::types::cloudbuild::GetTriggerBuildArtifactNpmPackage>,
    /// A list of objects to be uploaded to Cloud Storage upon successful completion of all build steps.
    /// 
    /// Files in the workspace matching specified paths globs will be uploaded to the
    /// Cloud Storage location using the builder service account's credentials.
    /// 
    /// The location and generation of the uploaded objects will be stored in the Build resource's results field.
    /// 
    /// If any objects fail to be pushed, the build is marked FAILURE.
    #[builder(into)]
    #[serde(rename = "objects")]
    pub r#objects: Vec<super::super::types::cloudbuild::GetTriggerBuildArtifactObject>,
    /// Python package to upload to Artifact Registry upon successful completion of all build steps. A package can encapsulate multiple objects to be uploaded to a single repository.
    /// 
    /// The location and generation of the uploaded objects will be stored in the Build resource's results field.
    /// 
    /// If any objects fail to be pushed, the build is marked FAILURE.
    #[builder(into)]
    #[serde(rename = "pythonPackages")]
    pub r#python_packages: Vec<super::super::types::cloudbuild::GetTriggerBuildArtifactPythonPackage>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetTriggerBuildArtifact {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "images".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#images,
                )
                .await,
            );
            map.insert(
                "maven_artifacts".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#maven_artifacts,
                )
                .await,
            );
            map.insert(
                "npm_packages".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#npm_packages,
                )
                .await,
            );
            map.insert(
                "objects".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#objects,
                )
                .await,
            );
            map.insert(
                "python_packages".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#python_packages,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetTriggerBuildArtifact {
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
                    r#images: {
                        let field_value = match fields_map.get("images") {
                            Some(value) => value,
                            None => bail!("Missing field 'images' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#maven_artifacts: {
                        let field_value = match fields_map.get("maven_artifacts") {
                            Some(value) => value,
                            None => bail!("Missing field 'maven_artifacts' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#npm_packages: {
                        let field_value = match fields_map.get("npm_packages") {
                            Some(value) => value,
                            None => bail!("Missing field 'npm_packages' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#objects: {
                        let field_value = match fields_map.get("objects") {
                            Some(value) => value,
                            None => bail!("Missing field 'objects' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#python_packages: {
                        let field_value = match fields_map.get("python_packages") {
                            Some(value) => value,
                            None => bail!("Missing field 'python_packages' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
