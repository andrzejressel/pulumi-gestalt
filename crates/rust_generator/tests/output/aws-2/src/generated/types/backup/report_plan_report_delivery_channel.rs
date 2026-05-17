#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ReportPlanReportDeliveryChannel {
    /// A list of the format of your reports: CSV, JSON, or both. If not specified, the default format is CSV.
    #[builder(into)]
    #[serde(rename = "formats")]
    pub r#formats: Option<Vec<String>>,
    /// The unique name of the S3 bucket that receives your reports.
    #[builder(into)]
    #[serde(rename = "s3BucketName")]
    pub r#s_3_bucket_name: String,
    /// The prefix for where Backup Audit Manager delivers your reports to Amazon S3. The prefix is this part of the following path: s3://your-bucket-name/prefix/Backup/us-west-2/year/month/day/report-name. If not specified, there is no prefix.
    #[builder(into)]
    #[serde(rename = "s3KeyPrefix")]
    pub r#s_3_key_prefix: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ReportPlanReportDeliveryChannel {
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
                "formats".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#formats,
                )
                .await,
            );
            map.insert(
                "s_3_bucket_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#s_3_bucket_name,
                )
                .await,
            );
            map.insert(
                "s_3_key_prefix".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#s_3_key_prefix,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ReportPlanReportDeliveryChannel {
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
                    r#formats: {
                        let field_value = match fields_map.get("formats") {
                            Some(value) => value,
                            None => bail!("Missing field 'formats' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#s_3_bucket_name: {
                        let field_value = match fields_map.get("s_3_bucket_name") {
                            Some(value) => value,
                            None => bail!("Missing field 's_3_bucket_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#s_3_key_prefix: {
                        let field_value = match fields_map.get("s_3_key_prefix") {
                            Some(value) => value,
                            None => bail!("Missing field 's_3_key_prefix' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
