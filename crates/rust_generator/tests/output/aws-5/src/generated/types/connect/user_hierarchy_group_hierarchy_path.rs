#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct UserHierarchyGroupHierarchyPath {
    /// A block that defines the details of level five. The level block is documented below.
    #[builder(into)]
    #[serde(rename = "levelFives")]
    pub r#level_fives: Option<Vec<super::super::types::connect::UserHierarchyGroupHierarchyPathLevelFife>>,
    /// A block that defines the details of level four. The level block is documented below.
    #[builder(into)]
    #[serde(rename = "levelFours")]
    pub r#level_fours: Option<Vec<super::super::types::connect::UserHierarchyGroupHierarchyPathLevelFour>>,
    /// A block that defines the details of level one. The level block is documented below.
    #[builder(into)]
    #[serde(rename = "levelOnes")]
    pub r#level_ones: Option<Vec<super::super::types::connect::UserHierarchyGroupHierarchyPathLevelOne>>,
    /// A block that defines the details of level three. The level block is documented below.
    #[builder(into)]
    #[serde(rename = "levelThrees")]
    pub r#level_threes: Option<Vec<super::super::types::connect::UserHierarchyGroupHierarchyPathLevelThree>>,
    /// A block that defines the details of level two. The level block is documented below.
    #[builder(into)]
    #[serde(rename = "levelTwos")]
    pub r#level_twos: Option<Vec<super::super::types::connect::UserHierarchyGroupHierarchyPathLevelTwo>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for UserHierarchyGroupHierarchyPath {
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
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for UserHierarchyGroupHierarchyPath {
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
