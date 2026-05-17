#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct LocalUserPermissionScopePermissions {
    /// Specifies if the Local User has the create permission for this scope. Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "create")]
    pub r#create: Option<bool>,
    /// Specifies if the Local User has the delete permission for this scope. Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "delete")]
    pub r#delete: Option<bool>,
    /// Specifies if the Local User has the list permission for this scope. Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "list")]
    pub r#list: Option<bool>,
    /// Specifies if the Local User has the read permission for this scope. Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "read")]
    pub r#read: Option<bool>,
    /// Specifies if the Local User has the write permission for this scope. Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "write")]
    pub r#write: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for LocalUserPermissionScopePermissions {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_concurrent, to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "create",
                    &self.r#create,
                ),
                to_pulumi_object_field(
                    "delete",
                    &self.r#delete,
                ),
                to_pulumi_object_field(
                    "list",
                    &self.r#list,
                ),
                to_pulumi_object_field(
                    "read",
                    &self.r#read,
                ),
                to_pulumi_object_field(
                    "write",
                    &self.r#write,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for LocalUserPermissionScopePermissions {
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
                    r#create: {
                        let field_value = match fields_map.get("create") {
                            Some(value) => value,
                            None => bail!("Missing field 'create' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#delete: {
                        let field_value = match fields_map.get("delete") {
                            Some(value) => value,
                            None => bail!("Missing field 'delete' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#list: {
                        let field_value = match fields_map.get("list") {
                            Some(value) => value,
                            None => bail!("Missing field 'list' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#read: {
                        let field_value = match fields_map.get("read") {
                            Some(value) => value,
                            None => bail!("Missing field 'read' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#write: {
                        let field_value = match fields_map.get("write") {
                            Some(value) => value,
                            None => bail!("Missing field 'write' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
