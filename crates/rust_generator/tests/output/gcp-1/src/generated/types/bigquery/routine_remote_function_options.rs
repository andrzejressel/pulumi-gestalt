#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RoutineRemoteFunctionOptions {
    /// Fully qualified name of the user-provided connection object which holds
    /// the authentication information to send requests to the remote service.
    /// Format: "projects/{projectId}/locations/{locationId}/connections/{connectionId}"
    #[builder(into)]
    #[serde(rename = "connection")]
    pub r#connection: Option<String>,
    /// Endpoint of the user-provided remote service, e.g.
    /// `https://us-east1-my_gcf_project.cloudfunctions.net/remote_add`
    #[builder(into)]
    #[serde(rename = "endpoint")]
    pub r#endpoint: Option<String>,
    /// Max number of rows in each batch sent to the remote service. If absent or if 0,
    /// BigQuery dynamically decides the number of rows in a batch.
    #[builder(into)]
    #[serde(rename = "maxBatchingRows")]
    pub r#max_batching_rows: Option<String>,
    /// User-defined context as a set of key/value pairs, which will be sent as function
    /// invocation context together with batched arguments in the requests to the remote
    /// service. The total number of bytes of keys and values must be less than 8KB.
    /// An object containing a list of "key": value pairs. Example:
    /// `{ "name": "wrench", "mass": "1.3kg", "count": "3" }`.
    #[builder(into)]
    #[serde(rename = "userDefinedContext")]
    pub r#user_defined_context: Option<std::collections::HashMap<String, String>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for RoutineRemoteFunctionOptions {
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
                "connection".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#connection,
                )
                .await,
            );
            map.insert(
                "endpoint".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#endpoint,
                )
                .await,
            );
            map.insert(
                "max_batching_rows".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#max_batching_rows,
                )
                .await,
            );
            map.insert(
                "user_defined_context".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#user_defined_context,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for RoutineRemoteFunctionOptions {
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
                    r#connection: {
                        let field_value = match fields_map.get("connection") {
                            Some(value) => value,
                            None => bail!("Missing field 'connection' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#endpoint: {
                        let field_value = match fields_map.get("endpoint") {
                            Some(value) => value,
                            None => bail!("Missing field 'endpoint' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_batching_rows: {
                        let field_value = match fields_map.get("max_batching_rows") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_batching_rows' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#user_defined_context: {
                        let field_value = match fields_map.get("user_defined_context") {
                            Some(value) => value,
                            None => bail!("Missing field 'user_defined_context' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
