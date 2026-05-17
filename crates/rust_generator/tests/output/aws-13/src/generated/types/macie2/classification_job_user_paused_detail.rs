#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ClassificationJobUserPausedDetail {
    #[builder(into)]
    #[serde(rename = "jobExpiresAt")]
    pub r#job_expires_at: Option<String>,
    #[builder(into)]
    #[serde(rename = "jobImminentExpirationHealthEventArn")]
    pub r#job_imminent_expiration_health_event_arn: Option<String>,
    #[builder(into)]
    #[serde(rename = "jobPausedAt")]
    pub r#job_paused_at: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ClassificationJobUserPausedDetail {
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
                "job_expires_at".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#job_expires_at,
                )
                .await,
            );
            map.insert(
                "job_imminent_expiration_health_event_arn".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#job_imminent_expiration_health_event_arn,
                )
                .await,
            );
            map.insert(
                "job_paused_at".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#job_paused_at,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ClassificationJobUserPausedDetail {
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
                    r#job_expires_at: {
                        let field_value = match fields_map.get("job_expires_at") {
                            Some(value) => value,
                            None => bail!("Missing field 'job_expires_at' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#job_imminent_expiration_health_event_arn: {
                        let field_value = match fields_map.get("job_imminent_expiration_health_event_arn") {
                            Some(value) => value,
                            None => bail!("Missing field 'job_imminent_expiration_health_event_arn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#job_paused_at: {
                        let field_value = match fields_map.get("job_paused_at") {
                            Some(value) => value,
                            None => bail!("Missing field 'job_paused_at' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
