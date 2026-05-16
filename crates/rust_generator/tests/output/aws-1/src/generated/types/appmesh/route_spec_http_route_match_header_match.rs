#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RouteSpecHttpRouteMatchHeaderMatch {
    /// Header value sent by the client must match the specified value exactly.
    #[builder(into)]
    #[serde(rename = "exact")]
    pub r#exact: Option<String>,
    /// Header value sent by the client must begin with the specified characters.
    #[builder(into)]
    #[serde(rename = "prefix")]
    pub r#prefix: Option<String>,
    /// Object that specifies the range of numbers that the header value sent by the client must be included in.
    #[builder(into)]
    #[serde(rename = "range")]
    pub r#range: Option<Box<super::super::types::appmesh::RouteSpecHttpRouteMatchHeaderMatchRange>>,
    /// Header value sent by the client must include the specified characters.
    #[builder(into)]
    #[serde(rename = "regex")]
    pub r#regex: Option<String>,
    /// Header value sent by the client must end with the specified characters.
    #[builder(into)]
    #[serde(rename = "suffix")]
    pub r#suffix: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for RouteSpecHttpRouteMatchHeaderMatch {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("exact".to_string(), self.r#exact.to_pulumi_value().await);
            map.insert("prefix".to_string(), self.r#prefix.to_pulumi_value().await);
            map.insert("range".to_string(), self.r#range.to_pulumi_value().await);
            map.insert("regex".to_string(), self.r#regex.to_pulumi_value().await);
            map.insert("suffix".to_string(), self.r#suffix.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for RouteSpecHttpRouteMatchHeaderMatch {
    fn from_pulumi_value(
        value: &pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    ) -> pulumi_gestalt_rust::__private::rootcause::Result<Self> {
        use std::collections::BTreeMap;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValueContent;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue;
        use pulumi_gestalt_rust::__private::rootcause::bail;

        match value.content {
            PulumiValueContent::Object(ref obj) => {
                let fields_map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> =
                    obj.iter().cloned().collect();

                Ok(Self {
                    r#exact: {
                        let field_value = match fields_map.get("exact") {
                            Some(value) => value,
                            None => bail!("Missing field 'exact' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#prefix: {
                        let field_value = match fields_map.get("prefix") {
                            Some(value) => value,
                            None => bail!("Missing field 'prefix' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#range: {
                        let field_value = match fields_map.get("range") {
                            Some(value) => value,
                            None => bail!("Missing field 'range' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::appmesh::RouteSpecHttpRouteMatchHeaderMatchRange>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#regex: {
                        let field_value = match fields_map.get("regex") {
                            Some(value) => value,
                            None => bail!("Missing field 'regex' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#suffix: {
                        let field_value = match fields_map.get("suffix") {
                            Some(value) => value,
                            None => bail!("Missing field 'suffix' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
