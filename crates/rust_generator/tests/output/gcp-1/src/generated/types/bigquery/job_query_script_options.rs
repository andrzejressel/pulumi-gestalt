#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct JobQueryScriptOptions {
    /// Determines which statement in the script represents the "key result",
    /// used to populate the schema and query results of the script job.
    /// Possible values are: `LAST`, `FIRST_SELECT`.
    #[builder(into)]
    #[serde(rename = "keyResultStatement")]
    pub r#key_result_statement: Option<String>,
    /// Limit on the number of bytes billed per statement. Exceeding this budget results in an error.
    #[builder(into)]
    #[serde(rename = "statementByteBudget")]
    pub r#statement_byte_budget: Option<String>,
    /// Timeout period for each statement in a script.
    #[builder(into)]
    #[serde(rename = "statementTimeoutMs")]
    pub r#statement_timeout_ms: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for JobQueryScriptOptions {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_concurrent, to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "key_result_statement",
                    &self.r#key_result_statement,
                ),
                to_pulumi_object_field(
                    "statement_byte_budget",
                    &self.r#statement_byte_budget,
                ),
                to_pulumi_object_field(
                    "statement_timeout_ms",
                    &self.r#statement_timeout_ms,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for JobQueryScriptOptions {
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
                    r#key_result_statement: {
                        let field_value = match fields_map.get("key_result_statement") {
                            Some(value) => value,
                            None => bail!("Missing field 'key_result_statement' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#statement_byte_budget: {
                        let field_value = match fields_map.get("statement_byte_budget") {
                            Some(value) => value,
                            None => bail!("Missing field 'statement_byte_budget' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#statement_timeout_ms: {
                        let field_value = match fields_map.get("statement_timeout_ms") {
                            Some(value) => value,
                            None => bail!("Missing field 'statement_timeout_ms' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
