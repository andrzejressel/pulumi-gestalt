#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, Debug, PartialEq, Clone)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub enum CloudAuditOptionsLogName {
    /// Default. Should not be used.
    #[serde(rename = "UNSPECIFIED_LOG_NAME")]
    UnspecifiedLogName,
    /// Corresponds to "cloudaudit.googleapis.com/activity"
    #[serde(rename = "ADMIN_ACTIVITY")]
    AdminActivity,
    /// Corresponds to "cloudaudit.googleapis.com/data_access"
    #[serde(rename = "DATA_ACCESS")]
    DataAccess,
    /// What if triple quotes """ are used in the description
    #[serde(rename = "SYNTHETIC")]
    Synthetic,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for CloudAuditOptionsLogName {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > + Send {
        let value = match self {
            CloudAuditOptionsLogName::UnspecifiedLogName => "UNSPECIFIED_LOG_NAME".to_string(),
            CloudAuditOptionsLogName::AdminActivity => "ADMIN_ACTIVITY".to_string(),
            CloudAuditOptionsLogName::DataAccess => "DATA_ACCESS".to_string(),
            CloudAuditOptionsLogName::Synthetic => "SYNTHETIC".to_string(),
        };
        std::future::ready(pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue {
            content: pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValueContent::String(value),
            secret: false,
            dependencies: std::collections::HashSet::new(),
        })
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for CloudAuditOptionsLogName {
    fn from_pulumi_value(
        value: &pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    ) -> pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::rootcause::Result<Self> {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValueContent;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::rootcause::bail;

        match &value.content {
            PulumiValueContent::String(s) => match s.as_str() {
                "UNSPECIFIED_LOG_NAME" => Ok(CloudAuditOptionsLogName::UnspecifiedLogName),
                "ADMIN_ACTIVITY" => Ok(CloudAuditOptionsLogName::AdminActivity),
                "DATA_ACCESS" => Ok(CloudAuditOptionsLogName::DataAccess),
                "SYNTHETIC" => Ok(CloudAuditOptionsLogName::Synthetic),
                _ => bail!("Invalid string enum value: {}", s),
            },
            _ => bail!("Expected String, got {:?}", value.content),
        }
    }
}
