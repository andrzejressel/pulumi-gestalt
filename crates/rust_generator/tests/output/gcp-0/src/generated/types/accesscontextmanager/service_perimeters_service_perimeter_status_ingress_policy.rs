#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ServicePerimetersServicePerimeterStatusIngressPolicy {
    /// Defines the conditions on the source of a request causing this `IngressPolicy`
    /// to apply.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "ingressFrom")]
    pub r#ingress_from: Option<Box<super::super::types::accesscontextmanager::ServicePerimetersServicePerimeterStatusIngressPolicyIngressFrom>>,
    /// Defines the conditions on the `ApiOperation` and request destination that cause
    /// this `IngressPolicy` to apply.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "ingressTo")]
    pub r#ingress_to: Option<Box<super::super::types::accesscontextmanager::ServicePerimetersServicePerimeterStatusIngressPolicyIngressTo>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ServicePerimetersServicePerimeterStatusIngressPolicy {
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
                "ingress_from".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#ingress_from,
                )
                .await,
            );
            map.insert(
                "ingress_to".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#ingress_to,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ServicePerimetersServicePerimeterStatusIngressPolicy {
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
                    r#ingress_from: {
                        let field_value = match fields_map.get("ingress_from") {
                            Some(value) => value,
                            None => bail!("Missing field 'ingress_from' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ingress_to: {
                        let field_value = match fields_map.get("ingress_to") {
                            Some(value) => value,
                            None => bail!("Missing field 'ingress_to' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
