#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetDataSetPhysicalTableMapS3Source {
    #[builder(into)]
    #[serde(rename = "dataSourceArn")]
    pub r#data_source_arn: String,
    #[builder(into)]
    #[serde(rename = "inputColumns")]
    pub r#input_columns: Vec<super::super::types::quicksight::GetDataSetPhysicalTableMapS3SourceInputColumn>,
    #[builder(into)]
    #[serde(rename = "uploadSettings")]
    pub r#upload_settings: Vec<super::super::types::quicksight::GetDataSetPhysicalTableMapS3SourceUploadSetting>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetDataSetPhysicalTableMapS3Source {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("data_source_arn".to_string(), self.r#data_source_arn.to_pulumi_value().await);
            map.insert("input_columns".to_string(), self.r#input_columns.to_pulumi_value().await);
            map.insert("upload_settings".to_string(), self.r#upload_settings.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetDataSetPhysicalTableMapS3Source {
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
                    r#data_source_arn: {
                        let field_value = match fields_map.get("data_source_arn") {
                            Some(value) => value,
                            None => bail!("Missing field 'data_source_arn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#input_columns: {
                        let field_value = match fields_map.get("input_columns") {
                            Some(value) => value,
                            None => bail!("Missing field 'input_columns' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Vec<super::super::types::quicksight::GetDataSetPhysicalTableMapS3SourceInputColumn> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#upload_settings: {
                        let field_value = match fields_map.get("upload_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'upload_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Vec<super::super::types::quicksight::GetDataSetPhysicalTableMapS3SourceUploadSetting> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
