#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetWindowsWebAppSiteConfigHandlerMapping {
    /// The command-line arguments to be passed to the script processor.
    #[builder(into)]
    #[serde(rename = "arguments")]
    pub r#arguments: String,
    /// The extension to be handled by the specified FastCGI application.
    #[builder(into)]
    #[serde(rename = "extension")]
    pub r#extension: String,
    /// The absolute path to the FastCGI application.
    #[builder(into)]
    #[serde(rename = "scriptProcessorPath")]
    pub r#script_processor_path: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetWindowsWebAppSiteConfigHandlerMapping {
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
                "arguments".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#arguments,
                )
                .await,
            );
            map.insert(
                "extension".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#extension,
                )
                .await,
            );
            map.insert(
                "script_processor_path".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#script_processor_path,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetWindowsWebAppSiteConfigHandlerMapping {
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
                    r#arguments: {
                        let field_value = match fields_map.get("arguments") {
                            Some(value) => value,
                            None => bail!("Missing field 'arguments' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#extension: {
                        let field_value = match fields_map.get("extension") {
                            Some(value) => value,
                            None => bail!("Missing field 'extension' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#script_processor_path: {
                        let field_value = match fields_map.get("script_processor_path") {
                            Some(value) => value,
                            None => bail!("Missing field 'script_processor_path' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
