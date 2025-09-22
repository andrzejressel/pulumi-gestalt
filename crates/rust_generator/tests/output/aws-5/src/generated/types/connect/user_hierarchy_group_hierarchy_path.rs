#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
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
