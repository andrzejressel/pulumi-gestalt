#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ThingGroupMetadata {
    #[builder(into)]
    #[serde(rename = "creationDate")]
    pub r#creation_date: Option<String>,
    /// The name of the parent Thing Group.
    #[builder(into)]
    #[serde(rename = "parentGroupName")]
    pub r#parent_group_name: Option<String>,
    #[builder(into)]
    #[serde(rename = "rootToParentGroups")]
    pub r#root_to_parent_groups: Option<Vec<super::super::types::iot::ThingGroupMetadataRootToParentGroup>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ThingGroupMetadata {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("creation_date".to_string(), self.r#creation_date.to_pulumi_value().await);
            map.insert("parent_group_name".to_string(), self.r#parent_group_name.to_pulumi_value().await);
            map.insert("root_to_parent_groups".to_string(), self.r#root_to_parent_groups.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ThingGroupMetadata {
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
                    r#creation_date: {
                        let field_value = match fields_map.get("creation_date") {
                            Some(value) => value,
                            None => bail!("Missing field 'creation_date' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#parent_group_name: {
                        let field_value = match fields_map.get("parent_group_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'parent_group_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#root_to_parent_groups: {
                        let field_value = match fields_map.get("root_to_parent_groups") {
                            Some(value) => value,
                            None => bail!("Missing field 'root_to_parent_groups' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<super::super::types::iot::ThingGroupMetadataRootToParentGroup>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
