#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct WindowsFileSystemAuditLogConfiguration {
    /// The Amazon Resource Name (ARN) for the destination of the audit logs. The destination can be any Amazon CloudWatch Logs log group ARN or Amazon Kinesis Data Firehose delivery stream ARN. Can be specified when `file_access_audit_log_level` and `file_share_access_audit_log_level` are not set to `DISABLED`. The name of the Amazon CloudWatch Logs log group must begin with the `/aws/fsx` prefix. The name of the Amazon Kinesis Data Firehouse delivery stream must begin with the `aws-fsx` prefix. If you do not provide a destination in `audit_log_destionation`, Amazon FSx will create and use a log stream in the CloudWatch Logs /aws/fsx/windows log group.
    #[builder(into)]
    #[serde(rename = "auditLogDestination")]
    pub r#audit_log_destination: Option<String>,
    /// Sets which attempt type is logged by Amazon FSx for file and folder accesses. Valid values are `SUCCESS_ONLY`, `FAILURE_ONLY`, `SUCCESS_AND_FAILURE`, and `DISABLED`. Default value is `DISABLED`.
    #[builder(into)]
    #[serde(rename = "fileAccessAuditLogLevel")]
    pub r#file_access_audit_log_level: Option<String>,
    /// Sets which attempt type is logged by Amazon FSx for file share accesses. Valid values are `SUCCESS_ONLY`, `FAILURE_ONLY`, `SUCCESS_AND_FAILURE`, and `DISABLED`. Default value is `DISABLED`.
    #[builder(into)]
    #[serde(rename = "fileShareAccessAuditLogLevel")]
    pub r#file_share_access_audit_log_level: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for WindowsFileSystemAuditLogConfiguration {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "audit_log_destination".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#audit_log_destination,
                )
                .await,
            );
            map.insert(
                "file_access_audit_log_level".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#file_access_audit_log_level,
                )
                .await,
            );
            map.insert(
                "file_share_access_audit_log_level".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#file_share_access_audit_log_level,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for WindowsFileSystemAuditLogConfiguration {
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
                    r#audit_log_destination: {
                        let field_value = match fields_map.get("audit_log_destination") {
                            Some(value) => value,
                            None => bail!("Missing field 'audit_log_destination' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#file_access_audit_log_level: {
                        let field_value = match fields_map.get("file_access_audit_log_level") {
                            Some(value) => value,
                            None => bail!("Missing field 'file_access_audit_log_level' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#file_share_access_audit_log_level: {
                        let field_value = match fields_map.get("file_share_access_audit_log_level") {
                            Some(value) => value,
                            None => bail!("Missing field 'file_share_access_audit_log_level' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
