#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ServicePerimeterSpec {
    /// A list of AccessLevel resource names that allow resources within
    /// the ServicePerimeter to be accessed from the internet.
    /// AccessLevels listed must be in the same policy as this
    /// ServicePerimeter. Referencing a nonexistent AccessLevel is a
    /// syntax error. If no AccessLevel names are listed, resources within
    /// the perimeter can only be accessed via GCP calls with request
    /// origins within the perimeter. For Service Perimeter Bridge, must
    /// be empty.
    /// Format: accessPolicies/{policy_id}/accessLevels/{access_level_name}
    #[builder(into)]
    #[serde(rename = "accessLevels")]
    pub r#access_levels: Option<Vec<String>>,
    /// List of EgressPolicies to apply to the perimeter. A perimeter may
    /// have multiple EgressPolicies, each of which is evaluated separately.
    /// Access is granted if any EgressPolicy grants it. Must be empty for
    /// a perimeter bridge.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "egressPolicies")]
    pub r#egress_policies: Option<Vec<super::super::types::accesscontextmanager::ServicePerimeterSpecEgressPolicy>>,
    /// List of `IngressPolicies` to apply to the perimeter. A perimeter may
    /// have multiple `IngressPolicies`, each of which is evaluated
    /// separately. Access is granted if any `Ingress Policy` grants it.
    /// Must be empty for a perimeter bridge.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "ingressPolicies")]
    pub r#ingress_policies: Option<Vec<super::super::types::accesscontextmanager::ServicePerimeterSpecIngressPolicy>>,
    /// A list of GCP resources that are inside of the service perimeter.
    /// Currently only projects are allowed.
    /// Format: projects/{project_number}
    #[builder(into)]
    #[serde(rename = "resources")]
    pub r#resources: Option<Vec<String>>,
    /// GCP services that are subject to the Service Perimeter
    /// restrictions. Must contain a list of services. For example, if
    /// `storage.googleapis.com` is specified, access to the storage
    /// buckets inside the perimeter must meet the perimeter's access
    /// restrictions.
    #[builder(into)]
    #[serde(rename = "restrictedServices")]
    pub r#restricted_services: Option<Vec<String>>,
    /// Specifies how APIs are allowed to communicate within the Service
    /// Perimeter.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "vpcAccessibleServices")]
    pub r#vpc_accessible_services: Option<Box<super::super::types::accesscontextmanager::ServicePerimeterSpecVpcAccessibleServices>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ServicePerimeterSpec {
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
                "access_levels".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#access_levels,
                )
                .await,
            );
            map.insert(
                "egress_policies".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#egress_policies,
                )
                .await,
            );
            map.insert(
                "ingress_policies".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#ingress_policies,
                )
                .await,
            );
            map.insert(
                "resources".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#resources,
                )
                .await,
            );
            map.insert(
                "restricted_services".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#restricted_services,
                )
                .await,
            );
            map.insert(
                "vpc_accessible_services".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#vpc_accessible_services,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ServicePerimeterSpec {
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
                    r#access_levels: {
                        let field_value = match fields_map.get("access_levels") {
                            Some(value) => value,
                            None => bail!("Missing field 'access_levels' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#egress_policies: {
                        let field_value = match fields_map.get("egress_policies") {
                            Some(value) => value,
                            None => bail!("Missing field 'egress_policies' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ingress_policies: {
                        let field_value = match fields_map.get("ingress_policies") {
                            Some(value) => value,
                            None => bail!("Missing field 'ingress_policies' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#resources: {
                        let field_value = match fields_map.get("resources") {
                            Some(value) => value,
                            None => bail!("Missing field 'resources' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#restricted_services: {
                        let field_value = match fields_map.get("restricted_services") {
                            Some(value) => value,
                            None => bail!("Missing field 'restricted_services' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#vpc_accessible_services: {
                        let field_value = match fields_map.get("vpc_accessible_services") {
                            Some(value) => value,
                            None => bail!("Missing field 'vpc_accessible_services' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
