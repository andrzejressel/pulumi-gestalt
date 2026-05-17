#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PreventionJobTriggerInspectJobAction {
    /// Create a de-identified copy of the requested table or files.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "deidentify")]
    pub r#deidentify: Option<Box<super::super::types::dataloss::PreventionJobTriggerInspectJobActionDeidentify>>,
    /// Sends an email when the job completes. The email goes to IAM project owners and technical Essential Contacts.
    #[builder(into)]
    #[serde(rename = "jobNotificationEmails")]
    pub r#job_notification_emails: Option<Box<super::super::types::dataloss::PreventionJobTriggerInspectJobActionJobNotificationEmails>>,
    /// Publish a message into a given Pub/Sub topic when the job completes.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "pubSub")]
    pub r#pub_sub: Option<Box<super::super::types::dataloss::PreventionJobTriggerInspectJobActionPubSub>>,
    /// Publish findings of a DlpJob to Data Catalog.
    #[builder(into)]
    #[serde(rename = "publishFindingsToCloudDataCatalog")]
    pub r#publish_findings_to_cloud_data_catalog: Option<Box<super::super::types::dataloss::PreventionJobTriggerInspectJobActionPublishFindingsToCloudDataCatalog>>,
    /// Publish the result summary of a DlpJob to the Cloud Security Command Center.
    #[builder(into)]
    #[serde(rename = "publishSummaryToCscc")]
    pub r#publish_summary_to_cscc: Option<Box<super::super::types::dataloss::PreventionJobTriggerInspectJobActionPublishSummaryToCscc>>,
    /// Enable Stackdriver metric dlp.googleapis.com/findingCount.
    #[builder(into)]
    #[serde(rename = "publishToStackdriver")]
    pub r#publish_to_stackdriver: Option<Box<super::super::types::dataloss::PreventionJobTriggerInspectJobActionPublishToStackdriver>>,
    /// If set, the detailed findings will be persisted to the specified OutputStorageConfig. Only a single instance of this action can be specified. Compatible with: Inspect, Risk
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "saveFindings")]
    pub r#save_findings: Option<Box<super::super::types::dataloss::PreventionJobTriggerInspectJobActionSaveFindings>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for PreventionJobTriggerInspectJobAction {
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
                "deidentify".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#deidentify,
                )
                .await,
            );
            map.insert(
                "job_notification_emails".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#job_notification_emails,
                )
                .await,
            );
            map.insert(
                "pub_sub".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#pub_sub,
                )
                .await,
            );
            map.insert(
                "publish_findings_to_cloud_data_catalog".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#publish_findings_to_cloud_data_catalog,
                )
                .await,
            );
            map.insert(
                "publish_summary_to_cscc".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#publish_summary_to_cscc,
                )
                .await,
            );
            map.insert(
                "publish_to_stackdriver".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#publish_to_stackdriver,
                )
                .await,
            );
            map.insert(
                "save_findings".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#save_findings,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for PreventionJobTriggerInspectJobAction {
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
                    r#deidentify: {
                        let field_value = match fields_map.get("deidentify") {
                            Some(value) => value,
                            None => bail!("Missing field 'deidentify' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#job_notification_emails: {
                        let field_value = match fields_map.get("job_notification_emails") {
                            Some(value) => value,
                            None => bail!("Missing field 'job_notification_emails' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#pub_sub: {
                        let field_value = match fields_map.get("pub_sub") {
                            Some(value) => value,
                            None => bail!("Missing field 'pub_sub' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#publish_findings_to_cloud_data_catalog: {
                        let field_value = match fields_map.get("publish_findings_to_cloud_data_catalog") {
                            Some(value) => value,
                            None => bail!("Missing field 'publish_findings_to_cloud_data_catalog' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#publish_summary_to_cscc: {
                        let field_value = match fields_map.get("publish_summary_to_cscc") {
                            Some(value) => value,
                            None => bail!("Missing field 'publish_summary_to_cscc' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#publish_to_stackdriver: {
                        let field_value = match fields_map.get("publish_to_stackdriver") {
                            Some(value) => value,
                            None => bail!("Missing field 'publish_to_stackdriver' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#save_findings: {
                        let field_value = match fields_map.get("save_findings") {
                            Some(value) => value,
                            None => bail!("Missing field 'save_findings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
