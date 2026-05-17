#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetRouteSpecHttp2Route {
    #[builder(into)]
    #[serde(rename = "actions")]
    pub r#actions: Vec<super::super::types::appmesh::GetRouteSpecHttp2RouteAction>,
    #[builder(into)]
    #[serde(rename = "matches")]
    pub r#matches: Vec<super::super::types::appmesh::GetRouteSpecHttp2RouteMatch>,
    #[builder(into)]
    #[serde(rename = "retryPolicies")]
    pub r#retry_policies: Vec<super::super::types::appmesh::GetRouteSpecHttp2RouteRetryPolicy>,
    #[builder(into)]
    #[serde(rename = "timeouts")]
    pub r#timeouts: Vec<super::super::types::appmesh::GetRouteSpecHttp2RouteTimeout>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetRouteSpecHttp2Route {
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
                "actions".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#actions,
                )
                .await,
            );
            map.insert(
                "matches".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#matches,
                )
                .await,
            );
            map.insert(
                "retry_policies".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#retry_policies,
                )
                .await,
            );
            map.insert(
                "timeouts".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#timeouts,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetRouteSpecHttp2Route {
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
                    r#actions: {
                        let field_value = match fields_map.get("actions") {
                            Some(value) => value,
                            None => bail!("Missing field 'actions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#matches: {
                        let field_value = match fields_map.get("matches") {
                            Some(value) => value,
                            None => bail!("Missing field 'matches' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#retry_policies: {
                        let field_value = match fields_map.get("retry_policies") {
                            Some(value) => value,
                            None => bail!("Missing field 'retry_policies' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#timeouts: {
                        let field_value = match fields_map.get("timeouts") {
                            Some(value) => value,
                            None => bail!("Missing field 'timeouts' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
