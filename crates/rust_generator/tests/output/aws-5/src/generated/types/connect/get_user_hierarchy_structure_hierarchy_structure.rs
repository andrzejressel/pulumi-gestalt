#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetUserHierarchyStructureHierarchyStructure {
    /// Details of level five. See below.
    #[builder(into)]
    #[serde(rename = "levelFives")]
    pub r#level_fives: Vec<super::super::types::connect::GetUserHierarchyStructureHierarchyStructureLevelFife>,
    /// Details of level four. See below.
    #[builder(into)]
    #[serde(rename = "levelFours")]
    pub r#level_fours: Vec<super::super::types::connect::GetUserHierarchyStructureHierarchyStructureLevelFour>,
    /// Details of level one. See below.
    #[builder(into)]
    #[serde(rename = "levelOnes")]
    pub r#level_ones: Vec<super::super::types::connect::GetUserHierarchyStructureHierarchyStructureLevelOne>,
    /// Details of level three. See below.
    #[builder(into)]
    #[serde(rename = "levelThrees")]
    pub r#level_threes: Vec<super::super::types::connect::GetUserHierarchyStructureHierarchyStructureLevelThree>,
    /// Details of level two. See below.
    #[builder(into)]
    #[serde(rename = "levelTwos")]
    pub r#level_twos: Vec<super::super::types::connect::GetUserHierarchyStructureHierarchyStructureLevelTwo>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetUserHierarchyStructureHierarchyStructure {
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
                "level_fives".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#level_fives,
                )
                .await,
            );
            map.insert(
                "level_fours".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#level_fours,
                )
                .await,
            );
            map.insert(
                "level_ones".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#level_ones,
                )
                .await,
            );
            map.insert(
                "level_threes".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#level_threes,
                )
                .await,
            );
            map.insert(
                "level_twos".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#level_twos,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetUserHierarchyStructureHierarchyStructure {
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
                    r#level_fives: {
                        let field_value = match fields_map.get("level_fives") {
                            Some(value) => value,
                            None => bail!("Missing field 'level_fives' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#level_fours: {
                        let field_value = match fields_map.get("level_fours") {
                            Some(value) => value,
                            None => bail!("Missing field 'level_fours' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#level_ones: {
                        let field_value = match fields_map.get("level_ones") {
                            Some(value) => value,
                            None => bail!("Missing field 'level_ones' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#level_threes: {
                        let field_value = match fields_map.get("level_threes") {
                            Some(value) => value,
                            None => bail!("Missing field 'level_threes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#level_twos: {
                        let field_value = match fields_map.get("level_twos") {
                            Some(value) => value,
                            None => bail!("Missing field 'level_twos' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
