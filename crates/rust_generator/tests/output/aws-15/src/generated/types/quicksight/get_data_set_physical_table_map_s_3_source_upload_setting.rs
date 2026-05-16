#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetDataSetPhysicalTableMapS3SourceUploadSetting {
    #[builder(into)]
    #[serde(rename = "containsHeader")]
    pub r#contains_header: bool,
    #[builder(into)]
    #[serde(rename = "delimiter")]
    pub r#delimiter: String,
    #[builder(into)]
    #[serde(rename = "format")]
    pub r#format: String,
    #[builder(into)]
    #[serde(rename = "startFromRow")]
    pub r#start_from_row: i32,
    #[builder(into)]
    #[serde(rename = "textQualifier")]
    pub r#text_qualifier: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetDataSetPhysicalTableMapS3SourceUploadSetting {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("contains_header".to_string(), self.r#contains_header.to_pulumi_value().await);
            map.insert("delimiter".to_string(), self.r#delimiter.to_pulumi_value().await);
            map.insert("format".to_string(), self.r#format.to_pulumi_value().await);
            map.insert("start_from_row".to_string(), self.r#start_from_row.to_pulumi_value().await);
            map.insert("text_qualifier".to_string(), self.r#text_qualifier.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetDataSetPhysicalTableMapS3SourceUploadSetting {
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
                    r#contains_header: {
                        let field_value = match fields_map.get("contains_header") {
                            Some(value) => value,
                            None => bail!("Missing field 'contains_header' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <bool as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#delimiter: {
                        let field_value = match fields_map.get("delimiter") {
                            Some(value) => value,
                            None => bail!("Missing field 'delimiter' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#format: {
                        let field_value = match fields_map.get("format") {
                            Some(value) => value,
                            None => bail!("Missing field 'format' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#start_from_row: {
                        let field_value = match fields_map.get("start_from_row") {
                            Some(value) => value,
                            None => bail!("Missing field 'start_from_row' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <i32 as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#text_qualifier: {
                        let field_value = match fields_map.get("text_qualifier") {
                            Some(value) => value,
                            None => bail!("Missing field 'text_qualifier' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
