#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct EdgeCacheOriginOriginOverrideActionHeaderActionRequestHeadersToAdd {
    /// The name of the header to add.
    #[builder(into)]
    #[serde(rename = "headerName")]
    pub r#header_name: String,
    /// The value of the header to add.
    #[builder(into)]
    #[serde(rename = "headerValue")]
    pub r#header_value: String,
    /// Whether to replace all existing headers with the same name.
    /// By default, added header values are appended
    /// to the response or request headers with the
    /// same field names. The added values are
    /// separated by commas.
    /// To overwrite existing values, set `replace` to `true`.
    #[builder(into)]
    #[serde(rename = "replace")]
    pub r#replace: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for EdgeCacheOriginOriginOverrideActionHeaderActionRequestHeadersToAdd {
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
                "header_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#header_name,
                )
                .await,
            );
            map.insert(
                "header_value".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#header_value,
                )
                .await,
            );
            map.insert(
                "replace".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#replace,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for EdgeCacheOriginOriginOverrideActionHeaderActionRequestHeadersToAdd {
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
                    r#header_name: {
                        let field_value = match fields_map.get("header_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'header_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#header_value: {
                        let field_value = match fields_map.get("header_value") {
                            Some(value) => value,
                            None => bail!("Missing field 'header_value' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#replace: {
                        let field_value = match fields_map.get("replace") {
                            Some(value) => value,
                            None => bail!("Missing field 'replace' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
