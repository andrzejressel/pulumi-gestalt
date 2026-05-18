#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetAccountBlobContainerSasPermissions {
    /// Should Add permissions be enabled for this SAS?
    #[builder(into)]
    #[serde(rename = "add")]
    pub r#add: bool,
    /// Should Create permissions be enabled for this SAS?
    #[builder(into)]
    #[serde(rename = "create")]
    pub r#create: bool,
    /// Should Delete permissions be enabled for this SAS?
    #[builder(into)]
    #[serde(rename = "delete")]
    pub r#delete: bool,
    /// Should List permissions be enabled for this SAS?
    /// 
    /// Refer to the [SAS creation reference from Azure](https://docs.microsoft.com/rest/api/storageservices/create-service-sas)
    /// for additional details on the fields above.
    #[builder(into)]
    #[serde(rename = "list")]
    pub r#list: bool,
    /// Should Read permissions be enabled for this SAS?
    #[builder(into)]
    #[serde(rename = "read")]
    pub r#read: bool,
    /// Should Write permissions be enabled for this SAS?
    #[builder(into)]
    #[serde(rename = "write")]
    pub r#write: bool,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetAccountBlobContainerSasPermissions {
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
                    "add",
                    &self.r#add,
                ),
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
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetAccountBlobContainerSasPermissions {
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
                    r#add: {
                        let field_value = match fields_map.get("add") {
                            Some(value) => value,
                            None => bail!("Missing field 'add' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
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
