#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetCostCategoryRuleRuleOr {
    /// Return results that match both `Dimension` objects.
    #[builder(into)]
    #[serde(rename = "ands")]
    pub r#ands: Vec<super::super::types::costexplorer::GetCostCategoryRuleRuleOrAnd>,
    /// Configuration block for the filter that's based on `CostCategory` values. See below.
    #[builder(into)]
    #[serde(rename = "costCategories")]
    pub r#cost_categories: Vec<super::super::types::costexplorer::GetCostCategoryRuleRuleOrCostCategory>,
    /// Configuration block for the specific `Dimension` to use for `Expression`. See below.
    #[builder(into)]
    #[serde(rename = "dimensions")]
    pub r#dimensions: Vec<super::super::types::costexplorer::GetCostCategoryRuleRuleOrDimension>,
    /// Return results that do not match the `Dimension` object.
    #[builder(into)]
    #[serde(rename = "nots")]
    pub r#nots: Vec<super::super::types::costexplorer::GetCostCategoryRuleRuleOrNot>,
    /// Return results that match either `Dimension` object.
    #[builder(into)]
    #[serde(rename = "ors")]
    pub r#ors: Vec<super::super::types::costexplorer::GetCostCategoryRuleRuleOrOr>,
    /// Configuration block for the specific `Tag` to use for `Expression`. See below.
    #[builder(into)]
    #[serde(rename = "tags")]
    pub r#tags: Vec<super::super::types::costexplorer::GetCostCategoryRuleRuleOrTag>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetCostCategoryRuleRuleOr {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("ands".to_string(), self.r#ands.to_pulumi_value().await);
            map.insert("cost_categories".to_string(), self.r#cost_categories.to_pulumi_value().await);
            map.insert("dimensions".to_string(), self.r#dimensions.to_pulumi_value().await);
            map.insert("nots".to_string(), self.r#nots.to_pulumi_value().await);
            map.insert("ors".to_string(), self.r#ors.to_pulumi_value().await);
            map.insert("tags".to_string(), self.r#tags.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetCostCategoryRuleRuleOr {
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
                    r#ands: {
                        let field_value = match fields_map.get("ands") {
                            Some(value) => value,
                            None => bail!("Missing field 'ands' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Vec<super::super::types::costexplorer::GetCostCategoryRuleRuleOrAnd> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#cost_categories: {
                        let field_value = match fields_map.get("cost_categories") {
                            Some(value) => value,
                            None => bail!("Missing field 'cost_categories' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Vec<super::super::types::costexplorer::GetCostCategoryRuleRuleOrCostCategory> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#dimensions: {
                        let field_value = match fields_map.get("dimensions") {
                            Some(value) => value,
                            None => bail!("Missing field 'dimensions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Vec<super::super::types::costexplorer::GetCostCategoryRuleRuleOrDimension> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#nots: {
                        let field_value = match fields_map.get("nots") {
                            Some(value) => value,
                            None => bail!("Missing field 'nots' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Vec<super::super::types::costexplorer::GetCostCategoryRuleRuleOrNot> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#ors: {
                        let field_value = match fields_map.get("ors") {
                            Some(value) => value,
                            None => bail!("Missing field 'ors' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Vec<super::super::types::costexplorer::GetCostCategoryRuleRuleOrOr> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#tags: {
                        let field_value = match fields_map.get("tags") {
                            Some(value) => value,
                            None => bail!("Missing field 'tags' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Vec<super::super::types::costexplorer::GetCostCategoryRuleRuleOrTag> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
