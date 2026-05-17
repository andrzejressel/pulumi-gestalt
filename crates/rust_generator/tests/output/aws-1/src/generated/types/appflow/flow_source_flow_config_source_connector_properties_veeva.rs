#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FlowSourceFlowConfigSourceConnectorPropertiesVeeva {
    /// Document type specified in the Veeva document extract flow.
    #[builder(into)]
    #[serde(rename = "documentType")]
    pub r#document_type: Option<String>,
    /// Boolean value to include All Versions of files in Veeva document extract flow.
    #[builder(into)]
    #[serde(rename = "includeAllVersions")]
    pub r#include_all_versions: Option<bool>,
    /// Boolean value to include file renditions in Veeva document extract flow.
    #[builder(into)]
    #[serde(rename = "includeRenditions")]
    pub r#include_renditions: Option<bool>,
    /// Boolean value to include source files in Veeva document extract flow.
    #[builder(into)]
    #[serde(rename = "includeSourceFiles")]
    pub r#include_source_files: Option<bool>,
    #[builder(into)]
    #[serde(rename = "object")]
    pub r#object: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for FlowSourceFlowConfigSourceConnectorPropertiesVeeva {
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
                "document_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#document_type,
                )
                .await,
            );
            map.insert(
                "include_all_versions".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#include_all_versions,
                )
                .await,
            );
            map.insert(
                "include_renditions".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#include_renditions,
                )
                .await,
            );
            map.insert(
                "include_source_files".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#include_source_files,
                )
                .await,
            );
            map.insert(
                "object".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#object,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for FlowSourceFlowConfigSourceConnectorPropertiesVeeva {
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
                    r#document_type: {
                        let field_value = match fields_map.get("document_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'document_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#include_all_versions: {
                        let field_value = match fields_map.get("include_all_versions") {
                            Some(value) => value,
                            None => bail!("Missing field 'include_all_versions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#include_renditions: {
                        let field_value = match fields_map.get("include_renditions") {
                            Some(value) => value,
                            None => bail!("Missing field 'include_renditions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#include_source_files: {
                        let field_value = match fields_map.get("include_source_files") {
                            Some(value) => value,
                            None => bail!("Missing field 'include_source_files' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#object: {
                        let field_value = match fields_map.get("object") {
                            Some(value) => value,
                            None => bail!("Missing field 'object' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
