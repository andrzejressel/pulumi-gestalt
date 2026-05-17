#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct TriggerPredicateCondition {
    /// The condition crawl state. Currently, the values supported are `RUNNING`, `SUCCEEDED`, `CANCELLED`, and `FAILED`. If this is specified, `crawler_name` must also be specified. Conflicts with `state`.
    #[builder(into)]
    #[serde(rename = "crawlState")]
    pub r#crawl_state: Option<String>,
    /// The name of the crawler to watch. If this is specified, `crawl_state` must also be specified. Conflicts with `job_name`.
    #[builder(into)]
    #[serde(rename = "crawlerName")]
    pub r#crawler_name: Option<String>,
    /// The name of the job to watch. If this is specified, `state` must also be specified. Conflicts with `crawler_name`.
    #[builder(into)]
    #[serde(rename = "jobName")]
    pub r#job_name: Option<String>,
    /// A logical operator. Defaults to `EQUALS`.
    #[builder(into)]
    #[serde(rename = "logicalOperator")]
    pub r#logical_operator: Option<String>,
    /// The condition job state. Currently, the values supported are `SUCCEEDED`, `STOPPED`, `TIMEOUT` and `FAILED`. If this is specified, `job_name` must also be specified. Conflicts with `crawler_state`.
    #[builder(into)]
    #[serde(rename = "state")]
    pub r#state: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for TriggerPredicateCondition {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "crawl_state".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#crawl_state,
                )
                .await,
            );
            map.insert(
                "crawler_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#crawler_name,
                )
                .await,
            );
            map.insert(
                "job_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#job_name,
                )
                .await,
            );
            map.insert(
                "logical_operator".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#logical_operator,
                )
                .await,
            );
            map.insert(
                "state".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#state,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for TriggerPredicateCondition {
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
                    r#crawl_state: {
                        let field_value = match fields_map.get("crawl_state") {
                            Some(value) => value,
                            None => bail!("Missing field 'crawl_state' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#crawler_name: {
                        let field_value = match fields_map.get("crawler_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'crawler_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#job_name: {
                        let field_value = match fields_map.get("job_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'job_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#logical_operator: {
                        let field_value = match fields_map.get("logical_operator") {
                            Some(value) => value,
                            None => bail!("Missing field 'logical_operator' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#state: {
                        let field_value = match fields_map.get("state") {
                            Some(value) => value,
                            None => bail!("Missing field 'state' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
