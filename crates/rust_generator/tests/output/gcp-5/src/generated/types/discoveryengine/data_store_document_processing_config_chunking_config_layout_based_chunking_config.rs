#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DataStoreDocumentProcessingConfigChunkingConfigLayoutBasedChunkingConfig {
    /// The token size limit for each chunk.
    /// Supported values: 100-500 (inclusive). Default value: 500.
    #[builder(into)]
    #[serde(rename = "chunkSize")]
    pub r#chunk_size: Option<i32>,
    /// Whether to include appending different levels of headings to chunks from the middle of the document to prevent context loss.
    /// Default value: False.
    #[builder(into)]
    #[serde(rename = "includeAncestorHeadings")]
    pub r#include_ancestor_headings: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for DataStoreDocumentProcessingConfigChunkingConfigLayoutBasedChunkingConfig {
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
                "chunk_size".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#chunk_size,
                )
                .await,
            );
            map.insert(
                "include_ancestor_headings".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#include_ancestor_headings,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for DataStoreDocumentProcessingConfigChunkingConfigLayoutBasedChunkingConfig {
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
                    r#chunk_size: {
                        let field_value = match fields_map.get("chunk_size") {
                            Some(value) => value,
                            None => bail!("Missing field 'chunk_size' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#include_ancestor_headings: {
                        let field_value = match fields_map.get("include_ancestor_headings") {
                            Some(value) => value,
                            None => bail!("Missing field 'include_ancestor_headings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
