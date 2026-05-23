#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct InstanceRestoreToPointInTime {
    /// The date and time to restore from. Value must be a time in Universal Coordinated Time (UTC) format and must be before the latest restorable time for the DB instance. Cannot be specified with `use_latest_restorable_time`.
    #[builder(into)]
    pub r#restore_time: Option<String>,
    /// The ARN of the automated backup from which to restore. Required if `source_db_instance_identifier` or `source_dbi_resource_id` is not specified.
    #[builder(into)]
    pub r#source_db_instance_automated_backups_arn: Option<String>,
    /// The identifier of the source DB instance from which to restore. Must match the identifier of an existing DB instance. Required if `source_db_instance_automated_backups_arn` or `source_dbi_resource_id` is not specified.
    #[builder(into)]
    pub r#source_db_instance_identifier: Option<String>,
    /// The resource ID of the source DB instance from which to restore. Required if `source_db_instance_identifier` or `source_db_instance_automated_backups_arn` is not specified.
    #[builder(into)]
    pub r#source_dbi_resource_id: Option<String>,
    /// A boolean value that indicates whether the DB instance is restored from the latest backup time. Defaults to `false`. Cannot be specified with `restore_time`.
    #[builder(into)]
    pub r#use_latest_restorable_time: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for InstanceRestoreToPointInTime {
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
                    "restoreTime",
                    &self.r#restore_time,
                ),
                to_pulumi_object_field(
                    "sourceDbInstanceAutomatedBackupsArn",
                    &self.r#source_db_instance_automated_backups_arn,
                ),
                to_pulumi_object_field(
                    "sourceDbInstanceIdentifier",
                    &self.r#source_db_instance_identifier,
                ),
                to_pulumi_object_field(
                    "sourceDbiResourceId",
                    &self.r#source_dbi_resource_id,
                ),
                to_pulumi_object_field(
                    "useLatestRestorableTime",
                    &self.r#use_latest_restorable_time,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for InstanceRestoreToPointInTime {
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
                    r#restore_time: {
                        let field_value = match fields_map.get("restoreTime") {
                            Some(value) => value,
                            None => bail!("Missing field 'restoreTime' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#source_db_instance_automated_backups_arn: {
                        let field_value = match fields_map.get("sourceDbInstanceAutomatedBackupsArn") {
                            Some(value) => value,
                            None => bail!("Missing field 'sourceDbInstanceAutomatedBackupsArn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#source_db_instance_identifier: {
                        let field_value = match fields_map.get("sourceDbInstanceIdentifier") {
                            Some(value) => value,
                            None => bail!("Missing field 'sourceDbInstanceIdentifier' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#source_dbi_resource_id: {
                        let field_value = match fields_map.get("sourceDbiResourceId") {
                            Some(value) => value,
                            None => bail!("Missing field 'sourceDbiResourceId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#use_latest_restorable_time: {
                        let field_value = match fields_map.get("useLatestRestorableTime") {
                            Some(value) => value,
                            None => bail!("Missing field 'useLatestRestorableTime' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
