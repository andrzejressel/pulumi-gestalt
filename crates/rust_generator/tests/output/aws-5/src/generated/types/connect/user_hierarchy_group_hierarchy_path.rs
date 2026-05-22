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
    > + Send {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "levelFives",
                    &self.r#level_fives,
                ),
                to_pulumi_object_field(
                    "levelFours",
                    &self.r#level_fours,
                ),
                to_pulumi_object_field(
                    "levelOnes",
                    &self.r#level_ones,
                ),
                to_pulumi_object_field(
                    "levelThrees",
                    &self.r#level_threes,
                ),
                to_pulumi_object_field(
                    "levelTwos",
                    &self.r#level_twos,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
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
                        let field_value = match fields_map.get("levelFives") {
                            Some(value) => value,
                            None => bail!("Missing field 'levelFives' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#level_fours: {
                        let field_value = match fields_map.get("levelFours") {
                            Some(value) => value,
                            None => bail!("Missing field 'levelFours' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#level_ones: {
                        let field_value = match fields_map.get("levelOnes") {
                            Some(value) => value,
                            None => bail!("Missing field 'levelOnes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#level_threes: {
                        let field_value = match fields_map.get("levelThrees") {
                            Some(value) => value,
                            None => bail!("Missing field 'levelThrees' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#level_twos: {
                        let field_value = match fields_map.get("levelTwos") {
                            Some(value) => value,
                            None => bail!("Missing field 'levelTwos' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
