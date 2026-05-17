#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FlexibleAppVersionDeployment {
    /// Options for the build operations performed as a part of the version deployment. Only applicable when creating a version using source code directly.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "cloudBuildOptions")]
    pub r#cloud_build_options: Option<Box<super::super::types::appengine::FlexibleAppVersionDeploymentCloudBuildOptions>>,
    /// The Docker image for the container that runs the version.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "container")]
    pub r#container: Option<Box<super::super::types::appengine::FlexibleAppVersionDeploymentContainer>>,
    /// Manifest of the files stored in Google Cloud Storage that are included as part of this version.
    /// All files must be readable using the credentials supplied with this call.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "files")]
    pub r#files: Option<Vec<super::super::types::appengine::FlexibleAppVersionDeploymentFile>>,
    /// Zip File
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "zip")]
    pub r#zip: Option<Box<super::super::types::appengine::FlexibleAppVersionDeploymentZip>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for FlexibleAppVersionDeployment {
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
                "cloud_build_options".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#cloud_build_options,
                )
                .await,
            );
            map.insert(
                "container".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#container,
                )
                .await,
            );
            map.insert(
                "files".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#files,
                )
                .await,
            );
            map.insert(
                "zip".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#zip,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for FlexibleAppVersionDeployment {
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
                    r#cloud_build_options: {
                        let field_value = match fields_map.get("cloud_build_options") {
                            Some(value) => value,
                            None => bail!("Missing field 'cloud_build_options' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#container: {
                        let field_value = match fields_map.get("container") {
                            Some(value) => value,
                            None => bail!("Missing field 'container' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#files: {
                        let field_value = match fields_map.get("files") {
                            Some(value) => value,
                            None => bail!("Missing field 'files' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#zip: {
                        let field_value = match fields_map.get("zip") {
                            Some(value) => value,
                            None => bail!("Missing field 'zip' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
