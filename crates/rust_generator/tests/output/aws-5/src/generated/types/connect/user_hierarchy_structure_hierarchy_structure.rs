#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct UserHierarchyStructureHierarchyStructure {
    /// A block that defines the details of level five. The level block is documented below.
    /// 
    /// Each level block supports the following arguments:
    #[builder(into)]
    #[serde(rename = "levelFive")]
    pub r#level_five: Option<Box<super::super::types::connect::UserHierarchyStructureHierarchyStructureLevelFive>>,
    /// A block that defines the details of level four. The level block is documented below.
    #[builder(into)]
    #[serde(rename = "levelFour")]
    pub r#level_four: Option<Box<super::super::types::connect::UserHierarchyStructureHierarchyStructureLevelFour>>,
    /// A block that defines the details of level one. The level block is documented below.
    #[builder(into)]
    #[serde(rename = "levelOne")]
    pub r#level_one: Option<Box<super::super::types::connect::UserHierarchyStructureHierarchyStructureLevelOne>>,
    /// A block that defines the details of level three. The level block is documented below.
    #[builder(into)]
    #[serde(rename = "levelThree")]
    pub r#level_three: Option<Box<super::super::types::connect::UserHierarchyStructureHierarchyStructureLevelThree>>,
    /// A block that defines the details of level two. The level block is documented below.
    #[builder(into)]
    #[serde(rename = "levelTwo")]
    pub r#level_two: Option<Box<super::super::types::connect::UserHierarchyStructureHierarchyStructureLevelTwo>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for UserHierarchyStructureHierarchyStructure {
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
                    "level_five",
                    &self.r#level_five,
                ),
                to_pulumi_object_field(
                    "level_four",
                    &self.r#level_four,
                ),
                to_pulumi_object_field(
                    "level_one",
                    &self.r#level_one,
                ),
                to_pulumi_object_field(
                    "level_three",
                    &self.r#level_three,
                ),
                to_pulumi_object_field(
                    "level_two",
                    &self.r#level_two,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for UserHierarchyStructureHierarchyStructure {
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
                    r#level_five: {
                        let field_value = match fields_map.get("level_five") {
                            Some(value) => value,
                            None => bail!("Missing field 'level_five' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#level_four: {
                        let field_value = match fields_map.get("level_four") {
                            Some(value) => value,
                            None => bail!("Missing field 'level_four' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#level_one: {
                        let field_value = match fields_map.get("level_one") {
                            Some(value) => value,
                            None => bail!("Missing field 'level_one' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#level_three: {
                        let field_value = match fields_map.get("level_three") {
                            Some(value) => value,
                            None => bail!("Missing field 'level_three' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#level_two: {
                        let field_value = match fields_map.get("level_two") {
                            Some(value) => value,
                            None => bail!("Missing field 'level_two' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
