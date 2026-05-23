#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct JobCopy {
    /// Specifies whether the job is allowed to create new tables. The following values are supported:
    /// CREATE_IF_NEEDED: If the table does not exist, BigQuery creates the table.
    /// CREATE_NEVER: The table must already exist. If it does not, a 'notFound' error is returned in the job result.
    /// Creation, truncation and append actions occur as one atomic update upon job completion
    /// Default value is `CREATE_IF_NEEDED`.
    /// Possible values are: `CREATE_IF_NEEDED`, `CREATE_NEVER`.
    #[builder(into)]
    pub r#create_disposition: Option<String>,
    /// Custom encryption configuration (e.g., Cloud KMS keys)
    /// Structure is documented below.
    #[builder(into)]
    pub r#destination_encryption_configuration: Option<Box<super::super::types::bigquery::JobCopyDestinationEncryptionConfiguration>>,
    /// The destination table.
    /// Structure is documented below.
    #[builder(into)]
    pub r#destination_table: Option<Box<super::super::types::bigquery::JobCopyDestinationTable>>,
    /// Source tables to copy.
    /// Structure is documented below.
    #[builder(into)]
    pub r#source_tables: Vec<super::super::types::bigquery::JobCopySourceTable>,
    /// Specifies the action that occurs if the destination table already exists. The following values are supported:
    /// WRITE_TRUNCATE: If the table already exists, BigQuery overwrites the table data and uses the schema from the query result.
    /// WRITE_APPEND: If the table already exists, BigQuery appends the data to the table.
    /// WRITE_EMPTY: If the table already exists and contains data, a 'duplicate' error is returned in the job result.
    /// Each action is atomic and only occurs if BigQuery is able to complete the job successfully.
    /// Creation, truncation and append actions occur as one atomic update upon job completion.
    /// Default value is `WRITE_EMPTY`.
    /// Possible values are: `WRITE_TRUNCATE`, `WRITE_APPEND`, `WRITE_EMPTY`.
    #[builder(into)]
    pub r#write_disposition: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for JobCopy {
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
                    "createDisposition",
                    &self.r#create_disposition,
                ),
                to_pulumi_object_field(
                    "destinationEncryptionConfiguration",
                    &self.r#destination_encryption_configuration,
                ),
                to_pulumi_object_field(
                    "destinationTable",
                    &self.r#destination_table,
                ),
                to_pulumi_object_field(
                    "sourceTables",
                    &self.r#source_tables,
                ),
                to_pulumi_object_field(
                    "writeDisposition",
                    &self.r#write_disposition,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for JobCopy {
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
                    r#create_disposition: {
                        let field_value = match fields_map.get("createDisposition") {
                            Some(value) => value,
                            None => bail!("Missing field 'createDisposition' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#destination_encryption_configuration: {
                        let field_value = match fields_map.get("destinationEncryptionConfiguration") {
                            Some(value) => value,
                            None => bail!("Missing field 'destinationEncryptionConfiguration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#destination_table: {
                        let field_value = match fields_map.get("destinationTable") {
                            Some(value) => value,
                            None => bail!("Missing field 'destinationTable' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#source_tables: {
                        let field_value = match fields_map.get("sourceTables") {
                            Some(value) => value,
                            None => bail!("Missing field 'sourceTables' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#write_disposition: {
                        let field_value = match fields_map.get("writeDisposition") {
                            Some(value) => value,
                            None => bail!("Missing field 'writeDisposition' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
