#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ServicePerimetersServicePerimeterStatusEgressPolicyEgressToOperation {
    /// API methods or permissions to allow. Method or permission must belong
    /// to the service specified by `serviceName` field. A single MethodSelector
    /// entry with `*` specified for the `method` field will allow all methods
    /// AND permissions for the service specified in `serviceName`.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "methodSelectors")]
    pub r#method_selectors: Option<Vec<super::super::types::accesscontextmanager::ServicePerimetersServicePerimeterStatusEgressPolicyEgressToOperationMethodSelector>>,
    /// The name of the API whose methods or permissions the `IngressPolicy` or
    /// `EgressPolicy` want to allow. A single `ApiOperation` with serviceName
    /// field set to `*` will allow all methods AND permissions for all services.
    #[builder(into)]
    #[serde(rename = "serviceName")]
    pub r#service_name: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ServicePerimetersServicePerimeterStatusEgressPolicyEgressToOperation {
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
                "method_selectors".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#method_selectors,
                )
                .await,
            );
            map.insert(
                "service_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#service_name,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ServicePerimetersServicePerimeterStatusEgressPolicyEgressToOperation {
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
                    r#method_selectors: {
                        let field_value = match fields_map.get("method_selectors") {
                            Some(value) => value,
                            None => bail!("Missing field 'method_selectors' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#service_name: {
                        let field_value = match fields_map.get("service_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'service_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
