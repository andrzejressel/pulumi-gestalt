#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PipeEnrichmentParametersHttpParameters {
    #[builder(into)]
    #[serde(rename = "headerParameters")]
    pub r#header_parameters: Option<std::collections::HashMap<String, String>>,
    #[builder(into)]
    #[serde(rename = "pathParameterValues")]
    pub r#path_parameter_values: Option<String>,
    #[builder(into)]
    #[serde(rename = "queryStringParameters")]
    pub r#query_string_parameters: Option<std::collections::HashMap<String, String>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for PipeEnrichmentParametersHttpParameters {
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
                "header_parameters".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#header_parameters,
                )
                .await,
            );
            map.insert(
                "path_parameter_values".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#path_parameter_values,
                )
                .await,
            );
            map.insert(
                "query_string_parameters".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#query_string_parameters,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for PipeEnrichmentParametersHttpParameters {
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
                    r#header_parameters: {
                        let field_value = match fields_map.get("header_parameters") {
                            Some(value) => value,
                            None => bail!("Missing field 'header_parameters' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#path_parameter_values: {
                        let field_value = match fields_map.get("path_parameter_values") {
                            Some(value) => value,
                            None => bail!("Missing field 'path_parameter_values' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#query_string_parameters: {
                        let field_value = match fields_map.get("query_string_parameters") {
                            Some(value) => value,
                            None => bail!("Missing field 'query_string_parameters' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
