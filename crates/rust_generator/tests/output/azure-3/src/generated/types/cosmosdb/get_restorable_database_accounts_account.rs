#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetRestorableDatabaseAccountsAccount {
    /// The API type of the Cosmos DB Restorable Database Account.
    #[builder(into)]
    pub r#api_type: String,
    /// The creation time of the regional Cosmos DB Restorable Database Account.
    #[builder(into)]
    pub r#creation_time: String,
    /// The deletion time of the regional Cosmos DB Restorable Database Account.
    #[builder(into)]
    pub r#deletion_time: String,
    /// The ID of the Cosmos DB Restorable Database Account.
    #[builder(into)]
    pub r#id: String,
    /// One or more `restorable_locations` blocks as defined below.
    #[builder(into)]
    pub r#restorable_locations: Vec<super::super::types::cosmosdb::GetRestorableDatabaseAccountsAccountRestorableLocation>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetRestorableDatabaseAccountsAccount {
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
                    "apiType",
                    &self.r#api_type,
                ),
                to_pulumi_object_field(
                    "creationTime",
                    &self.r#creation_time,
                ),
                to_pulumi_object_field(
                    "deletionTime",
                    &self.r#deletion_time,
                ),
                to_pulumi_object_field(
                    "id",
                    &self.r#id,
                ),
                to_pulumi_object_field(
                    "restorableLocations",
                    &self.r#restorable_locations,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetRestorableDatabaseAccountsAccount {
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
                    r#api_type: {
                        let field_value = match fields_map.get("apiType") {
                            Some(value) => value,
                            None => bail!("Missing field 'apiType' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#creation_time: {
                        let field_value = match fields_map.get("creationTime") {
                            Some(value) => value,
                            None => bail!("Missing field 'creationTime' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#deletion_time: {
                        let field_value = match fields_map.get("deletionTime") {
                            Some(value) => value,
                            None => bail!("Missing field 'deletionTime' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#id: {
                        let field_value = match fields_map.get("id") {
                            Some(value) => value,
                            None => bail!("Missing field 'id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#restorable_locations: {
                        let field_value = match fields_map.get("restorableLocations") {
                            Some(value) => value,
                            None => bail!("Missing field 'restorableLocations' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
