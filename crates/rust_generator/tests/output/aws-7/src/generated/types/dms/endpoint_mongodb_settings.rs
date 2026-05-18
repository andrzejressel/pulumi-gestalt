#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct EndpointMongodbSettings {
    /// Authentication mechanism to access the MongoDB source endpoint. Default is `default`.
    #[builder(into)]
    #[serde(rename = "authMechanism")]
    pub r#auth_mechanism: Option<String>,
    /// Authentication database name. Not used when `auth_type` is `no`. Default is `admin`.
    #[builder(into)]
    #[serde(rename = "authSource")]
    pub r#auth_source: Option<String>,
    /// Authentication type to access the MongoDB source endpoint. Default is `password`.
    #[builder(into)]
    #[serde(rename = "authType")]
    pub r#auth_type: Option<String>,
    /// Number of documents to preview to determine the document organization. Use this setting when `nesting_level` is set to `one`. Default is `1000`.
    #[builder(into)]
    #[serde(rename = "docsToInvestigate")]
    pub r#docs_to_investigate: Option<String>,
    /// Document ID. Use this setting when `nesting_level` is set to `none`. Default is `false`.
    #[builder(into)]
    #[serde(rename = "extractDocId")]
    pub r#extract_doc_id: Option<String>,
    /// Specifies either document or table mode. Default is `none`. Valid values are `one` (table mode) and `none` (document mode).
    #[builder(into)]
    #[serde(rename = "nestingLevel")]
    pub r#nesting_level: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for EndpointMongodbSettings {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > + Send {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "auth_mechanism",
                    &self.r#auth_mechanism,
                ),
                to_pulumi_object_field(
                    "auth_source",
                    &self.r#auth_source,
                ),
                to_pulumi_object_field(
                    "auth_type",
                    &self.r#auth_type,
                ),
                to_pulumi_object_field(
                    "docs_to_investigate",
                    &self.r#docs_to_investigate,
                ),
                to_pulumi_object_field(
                    "extract_doc_id",
                    &self.r#extract_doc_id,
                ),
                to_pulumi_object_field(
                    "nesting_level",
                    &self.r#nesting_level,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for EndpointMongodbSettings {
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
                    r#auth_mechanism: {
                        let field_value = match fields_map.get("auth_mechanism") {
                            Some(value) => value,
                            None => bail!("Missing field 'auth_mechanism' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#auth_source: {
                        let field_value = match fields_map.get("auth_source") {
                            Some(value) => value,
                            None => bail!("Missing field 'auth_source' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#auth_type: {
                        let field_value = match fields_map.get("auth_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'auth_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#docs_to_investigate: {
                        let field_value = match fields_map.get("docs_to_investigate") {
                            Some(value) => value,
                            None => bail!("Missing field 'docs_to_investigate' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#extract_doc_id: {
                        let field_value = match fields_map.get("extract_doc_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'extract_doc_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#nesting_level: {
                        let field_value = match fields_map.get("nesting_level") {
                            Some(value) => value,
                            None => bail!("Missing field 'nesting_level' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
