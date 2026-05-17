#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ApiOperationRequestRepresentation {
    /// The Content Type of this representation, such as `application/json`.
    #[builder(into)]
    #[serde(rename = "contentType")]
    pub r#content_type: String,
    /// One or more `example` blocks as defined above.
    #[builder(into)]
    #[serde(rename = "examples")]
    pub r#examples: Option<Vec<super::super::types::apimanagement::ApiOperationRequestRepresentationExample>>,
    /// One or more `form_parameter` block as defined above.
    /// 
    /// > **NOTE:** This is Required when `content_type` is set to `application/x-www-form-urlencoded` or `multipart/form-data`.
    #[builder(into)]
    #[serde(rename = "formParameters")]
    pub r#form_parameters: Option<Vec<super::super::types::apimanagement::ApiOperationRequestRepresentationFormParameter>>,
    /// The ID of an API Management Schema which represents this Response.
    /// 
    /// > **NOTE:** This can only be specified when `content_type` is not set to `application/x-www-form-urlencoded` or `multipart/form-data`.
    #[builder(into)]
    #[serde(rename = "schemaId")]
    pub r#schema_id: Option<String>,
    /// The Type Name defined by the Schema.
    /// 
    /// > **NOTE:** This can only be specified when `content_type` is not set to `application/x-www-form-urlencoded` or `multipart/form-data`.
    #[builder(into)]
    #[serde(rename = "typeName")]
    pub r#type_name: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ApiOperationRequestRepresentation {
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
                "content_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#content_type,
                )
                .await,
            );
            map.insert(
                "examples".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#examples,
                )
                .await,
            );
            map.insert(
                "form_parameters".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#form_parameters,
                )
                .await,
            );
            map.insert(
                "schema_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#schema_id,
                )
                .await,
            );
            map.insert(
                "type_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#type_name,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ApiOperationRequestRepresentation {
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
                    r#content_type: {
                        let field_value = match fields_map.get("content_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'content_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#examples: {
                        let field_value = match fields_map.get("examples") {
                            Some(value) => value,
                            None => bail!("Missing field 'examples' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#form_parameters: {
                        let field_value = match fields_map.get("form_parameters") {
                            Some(value) => value,
                            None => bail!("Missing field 'form_parameters' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#schema_id: {
                        let field_value = match fields_map.get("schema_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'schema_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#type_name: {
                        let field_value = match fields_map.get("type_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'type_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
