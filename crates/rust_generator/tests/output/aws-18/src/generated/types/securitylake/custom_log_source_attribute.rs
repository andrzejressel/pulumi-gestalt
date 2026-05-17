#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct CustomLogSourceAttribute {
    /// The ARN of the AWS Glue crawler.
    #[builder(into)]
    #[serde(rename = "crawlerArn")]
    pub r#crawler_arn: String,
    /// The ARN of the AWS Glue database where results are written.
    #[builder(into)]
    #[serde(rename = "databaseArn")]
    pub r#database_arn: String,
    /// The ARN of the AWS Glue table.
    #[builder(into)]
    #[serde(rename = "tableArn")]
    pub r#table_arn: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for CustomLogSourceAttribute {
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
                "crawler_arn".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#crawler_arn,
                )
                .await,
            );
            map.insert(
                "database_arn".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#database_arn,
                )
                .await,
            );
            map.insert(
                "table_arn".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#table_arn,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for CustomLogSourceAttribute {
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
                    r#crawler_arn: {
                        let field_value = match fields_map.get("crawler_arn") {
                            Some(value) => value,
                            None => bail!("Missing field 'crawler_arn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#database_arn: {
                        let field_value = match fields_map.get("database_arn") {
                            Some(value) => value,
                            None => bail!("Missing field 'database_arn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#table_arn: {
                        let field_value = match fields_map.get("table_arn") {
                            Some(value) => value,
                            None => bail!("Missing field 'table_arn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
