#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RepositoryCatalogData {
    /// A detailed description of the contents of the repository. It is publicly visible in the Amazon ECR Public Gallery. The text must be in markdown format.
    #[builder(into)]
    #[serde(rename = "aboutText")]
    pub r#about_text: Option<String>,
    /// The system architecture that the images in the repository are compatible with. On the Amazon ECR Public Gallery, the following supported architectures will appear as badges on the repository and are used as search filters: `ARM`, `ARM 64`, `x86`, `x86-64`
    #[builder(into)]
    #[serde(rename = "architectures")]
    pub r#architectures: Option<Vec<String>>,
    /// A short description of the contents of the repository. This text appears in both the image details and also when searching for repositories on the Amazon ECR Public Gallery.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    /// The base64-encoded repository logo payload. (Only visible for verified accounts) Note that drift detection is disabled for this attribute.
    #[builder(into)]
    #[serde(rename = "logoImageBlob")]
    pub r#logo_image_blob: Option<String>,
    /// The operating systems that the images in the repository are compatible with. On the Amazon ECR Public Gallery, the following supported operating systems will appear as badges on the repository and are used as search filters: `Linux`, `Windows`
    #[builder(into)]
    #[serde(rename = "operatingSystems")]
    pub r#operating_systems: Option<Vec<String>>,
    /// Detailed information on how to use the contents of the repository. It is publicly visible in the Amazon ECR Public Gallery. The usage text provides context, support information, and additional usage details for users of the repository. The text must be in markdown format.
    #[builder(into)]
    #[serde(rename = "usageText")]
    pub r#usage_text: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for RepositoryCatalogData {
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
                "about_text".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#about_text,
                )
                .await,
            );
            map.insert(
                "architectures".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#architectures,
                )
                .await,
            );
            map.insert(
                "description".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#description,
                )
                .await,
            );
            map.insert(
                "logo_image_blob".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#logo_image_blob,
                )
                .await,
            );
            map.insert(
                "operating_systems".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#operating_systems,
                )
                .await,
            );
            map.insert(
                "usage_text".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#usage_text,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for RepositoryCatalogData {
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
                    r#about_text: {
                        let field_value = match fields_map.get("about_text") {
                            Some(value) => value,
                            None => bail!("Missing field 'about_text' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#architectures: {
                        let field_value = match fields_map.get("architectures") {
                            Some(value) => value,
                            None => bail!("Missing field 'architectures' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#description: {
                        let field_value = match fields_map.get("description") {
                            Some(value) => value,
                            None => bail!("Missing field 'description' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#logo_image_blob: {
                        let field_value = match fields_map.get("logo_image_blob") {
                            Some(value) => value,
                            None => bail!("Missing field 'logo_image_blob' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#operating_systems: {
                        let field_value = match fields_map.get("operating_systems") {
                            Some(value) => value,
                            None => bail!("Missing field 'operating_systems' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#usage_text: {
                        let field_value = match fields_map.get("usage_text") {
                            Some(value) => value,
                            None => bail!("Missing field 'usage_text' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
