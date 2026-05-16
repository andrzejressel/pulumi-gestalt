#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetDataSetLogicalTableMapDataTransformTagColumnOperationTag {
    #[builder(into)]
    #[serde(rename = "columnDescriptions")]
    pub r#column_descriptions: Vec<super::super::types::quicksight::GetDataSetLogicalTableMapDataTransformTagColumnOperationTagColumnDescription>,
    #[builder(into)]
    #[serde(rename = "columnGeographicRole")]
    pub r#column_geographic_role: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetDataSetLogicalTableMapDataTransformTagColumnOperationTag {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("column_descriptions".to_string(), self.r#column_descriptions.to_pulumi_value().await);
            map.insert("column_geographic_role".to_string(), self.r#column_geographic_role.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetDataSetLogicalTableMapDataTransformTagColumnOperationTag {
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
                    r#column_descriptions: {
                        let field_value = match fields_map.get("column_descriptions") {
                            Some(value) => value,
                            None => bail!("Missing field 'column_descriptions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Vec<super::super::types::quicksight::GetDataSetLogicalTableMapDataTransformTagColumnOperationTagColumnDescription> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#column_geographic_role: {
                        let field_value = match fields_map.get("column_geographic_role") {
                            Some(value) => value,
                            None => bail!("Missing field 'column_geographic_role' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
