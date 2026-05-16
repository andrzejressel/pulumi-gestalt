#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ResourceSetResourceSet {
    /// Description of the resource set.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    /// Unique identifier for the resource set. It's returned in the responses to create and list commands. You provide it to operations like update and delete.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Option<String>,
    /// Last time that the reosurce set was changed.
    #[builder(into)]
    #[serde(rename = "lastUpdateTime")]
    pub r#last_update_time: Option<String>,
    /// Descriptive name of the resource set. You can't change the name of a resource set after you create it.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// Indicates whether the resource set is in or out of the admin's Region scope. Valid values are `ACTIVE` (Admin can manage and delete the resource set) or `OUT_OF_ADMIN_SCOPE` (Admin can view the resource set, but theyy can't edit or delete the resource set.)
    #[builder(into)]
    #[serde(rename = "resourceSetStatus")]
    pub r#resource_set_status: Option<String>,
    /// Determines the resources that can be associated to the resource set. Depending on your setting for max results and the number of resource sets, a single call might not return the full list.
    #[builder(into)]
    #[serde(rename = "resourceTypeLists")]
    pub r#resource_type_lists: Option<Vec<String>>,
    #[builder(into)]
    #[serde(rename = "updateToken")]
    pub r#update_token: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ResourceSetResourceSet {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("description".to_string(), self.r#description.to_pulumi_value().await);
            map.insert("id".to_string(), self.r#id.to_pulumi_value().await);
            map.insert("last_update_time".to_string(), self.r#last_update_time.to_pulumi_value().await);
            map.insert("name".to_string(), self.r#name.to_pulumi_value().await);
            map.insert("resource_set_status".to_string(), self.r#resource_set_status.to_pulumi_value().await);
            map.insert("resource_type_lists".to_string(), self.r#resource_type_lists.to_pulumi_value().await);
            map.insert("update_token".to_string(), self.r#update_token.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ResourceSetResourceSet {
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
                    r#description: {
                        let field_value = match fields_map.get("description") {
                            Some(value) => value,
                            None => bail!("Missing field 'description' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#id: {
                        let field_value = match fields_map.get("id") {
                            Some(value) => value,
                            None => bail!("Missing field 'id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#last_update_time: {
                        let field_value = match fields_map.get("last_update_time") {
                            Some(value) => value,
                            None => bail!("Missing field 'last_update_time' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#name: {
                        let field_value = match fields_map.get("name") {
                            Some(value) => value,
                            None => bail!("Missing field 'name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#resource_set_status: {
                        let field_value = match fields_map.get("resource_set_status") {
                            Some(value) => value,
                            None => bail!("Missing field 'resource_set_status' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#resource_type_lists: {
                        let field_value = match fields_map.get("resource_type_lists") {
                            Some(value) => value,
                            None => bail!("Missing field 'resource_type_lists' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<String>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#update_token: {
                        let field_value = match fields_map.get("update_token") {
                            Some(value) => value,
                            None => bail!("Missing field 'update_token' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
