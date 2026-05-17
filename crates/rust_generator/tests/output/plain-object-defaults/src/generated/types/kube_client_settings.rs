#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct KubeClientSettings {
    /// Maximum burst for throttle. Default value is 10.
    #[builder(into)]
    #[serde(rename = "burst")]
    pub r#burst: Option<i32>,
    /// Maximum queries per second (QPS) to the API server from this client. Default value is 5.
    #[builder(into)]
    #[serde(rename = "qps")]
    pub r#qps: Option<f64>,
    #[builder(into)]
    #[serde(rename = "recTest")]
    pub r#rec_test: Option<Box<super::types::KubeClientSettings>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for KubeClientSettings {
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
                "burst".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#burst,
                )
                .await,
            );
            map.insert(
                "qps".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#qps,
                )
                .await,
            );
            map.insert(
                "rec_test".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#rec_test,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for KubeClientSettings {
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
                    r#burst: {
                        let field_value = match fields_map.get("burst") {
                            Some(value) => value,
                            None => bail!("Missing field 'burst' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#qps: {
                        let field_value = match fields_map.get("qps") {
                            Some(value) => value,
                            None => bail!("Missing field 'qps' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#rec_test: {
                        let field_value = match fields_map.get("rec_test") {
                            Some(value) => value,
                            None => bail!("Missing field 'rec_test' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
