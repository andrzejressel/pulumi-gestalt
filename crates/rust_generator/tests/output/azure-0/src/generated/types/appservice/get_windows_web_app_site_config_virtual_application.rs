#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetWindowsWebAppSiteConfigVirtualApplication {
    /// The path on disk to the Virtual Directory
    #[builder(into)]
    #[serde(rename = "physicalPath")]
    pub r#physical_path: String,
    /// Is this Application Pre-loaded at startup.
    #[builder(into)]
    #[serde(rename = "preload")]
    pub r#preload: bool,
    /// A `virtual_directory` block as defined below.
    #[builder(into)]
    #[serde(rename = "virtualDirectories")]
    pub r#virtual_directories: Vec<super::super::types::appservice::GetWindowsWebAppSiteConfigVirtualApplicationVirtualDirectory>,
    /// The Virtual Path of the Virtual Directory.
    #[builder(into)]
    #[serde(rename = "virtualPath")]
    pub r#virtual_path: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetWindowsWebAppSiteConfigVirtualApplication {
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
                "physical_path".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#physical_path,
                )
                .await,
            );
            map.insert(
                "preload".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#preload,
                )
                .await,
            );
            map.insert(
                "virtual_directories".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#virtual_directories,
                )
                .await,
            );
            map.insert(
                "virtual_path".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#virtual_path,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetWindowsWebAppSiteConfigVirtualApplication {
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
                    r#physical_path: {
                        let field_value = match fields_map.get("physical_path") {
                            Some(value) => value,
                            None => bail!("Missing field 'physical_path' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#preload: {
                        let field_value = match fields_map.get("preload") {
                            Some(value) => value,
                            None => bail!("Missing field 'preload' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#virtual_directories: {
                        let field_value = match fields_map.get("virtual_directories") {
                            Some(value) => value,
                            None => bail!("Missing field 'virtual_directories' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#virtual_path: {
                        let field_value = match fields_map.get("virtual_path") {
                            Some(value) => value,
                            None => bail!("Missing field 'virtual_path' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
