#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AccountRestore {
    /// A `database` block as defined below. Changing this forces a new resource to be created.
    #[builder(into)]
    pub r#databases: Option<Vec<super::super::types::cosmosdb::AccountRestoreDatabase>>,
    /// One or more `gremlin_database` blocks as defined below. Changing this forces a new resource to be created.
    #[builder(into)]
    pub r#gremlin_databases: Option<Vec<super::super::types::cosmosdb::AccountRestoreGremlinDatabase>>,
    /// The creation time of the database or the collection (Datetime Format `RFC 3339`). Changing this forces a new resource to be created.
    #[builder(into)]
    pub r#restore_timestamp_in_utc: String,
    /// The resource ID of the restorable database account from which the restore has to be initiated. The example is `/subscriptions/{subscriptionId}/providers/Microsoft.DocumentDB/locations/{location}/restorableDatabaseAccounts/{restorableDatabaseAccountName}`. Changing this forces a new resource to be created.
    /// 
    /// > **Note:** Any database account with `Continuous` type (live account or accounts deleted in last 30 days) is a restorable database account and there cannot be Create/Update/Delete operations on the restorable database accounts. They can only be read and retrieved by `azure.cosmosdb.getRestorableDatabaseAccounts`.
    #[builder(into)]
    pub r#source_cosmosdb_account_id: String,
    /// A list of specific tables available for restore. Changing this forces a new resource to be created.
    #[builder(into)]
    pub r#tables_to_restores: Option<Vec<String>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for AccountRestore {
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
                    "databases",
                    &self.r#databases,
                ),
                to_pulumi_object_field(
                    "gremlinDatabases",
                    &self.r#gremlin_databases,
                ),
                to_pulumi_object_field(
                    "restoreTimestampInUtc",
                    &self.r#restore_timestamp_in_utc,
                ),
                to_pulumi_object_field(
                    "sourceCosmosdbAccountId",
                    &self.r#source_cosmosdb_account_id,
                ),
                to_pulumi_object_field(
                    "tablesToRestores",
                    &self.r#tables_to_restores,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for AccountRestore {
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
                    r#databases: {
                        let field_value = match fields_map.get("databases") {
                            Some(value) => value,
                            None => bail!("Missing field 'databases' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#gremlin_databases: {
                        let field_value = match fields_map.get("gremlinDatabases") {
                            Some(value) => value,
                            None => bail!("Missing field 'gremlinDatabases' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#restore_timestamp_in_utc: {
                        let field_value = match fields_map.get("restoreTimestampInUtc") {
                            Some(value) => value,
                            None => bail!("Missing field 'restoreTimestampInUtc' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#source_cosmosdb_account_id: {
                        let field_value = match fields_map.get("sourceCosmosdbAccountId") {
                            Some(value) => value,
                            None => bail!("Missing field 'sourceCosmosdbAccountId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#tables_to_restores: {
                        let field_value = match fields_map.get("tablesToRestores") {
                            Some(value) => value,
                            None => bail!("Missing field 'tablesToRestores' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
