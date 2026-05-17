#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ServicePerimetersServicePerimeterSpecIngressPolicyIngressFrom {
    /// A list of identities that are allowed access through this ingress policy.
    /// Should be in the format of email address. The email address should represent
    /// individual user or service account only.
    #[builder(into)]
    #[serde(rename = "identities")]
    pub r#identities: Option<Vec<String>>,
    /// Specifies the type of identities that are allowed access from outside the
    /// perimeter. If left unspecified, then members of `identities` field will be
    /// allowed access.
    /// Possible values are: `IDENTITY_TYPE_UNSPECIFIED`, `ANY_IDENTITY`, `ANY_USER_ACCOUNT`, `ANY_SERVICE_ACCOUNT`.
    #[builder(into)]
    #[serde(rename = "identityType")]
    pub r#identity_type: Option<String>,
    /// Sources that this `IngressPolicy` authorizes access from.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "sources")]
    pub r#sources: Option<Vec<super::super::types::accesscontextmanager::ServicePerimetersServicePerimeterSpecIngressPolicyIngressFromSource>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ServicePerimetersServicePerimeterSpecIngressPolicyIngressFrom {
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
                "identities".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#identities,
                )
                .await,
            );
            map.insert(
                "identity_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#identity_type,
                )
                .await,
            );
            map.insert(
                "sources".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#sources,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ServicePerimetersServicePerimeterSpecIngressPolicyIngressFrom {
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
                    r#identities: {
                        let field_value = match fields_map.get("identities") {
                            Some(value) => value,
                            None => bail!("Missing field 'identities' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#identity_type: {
                        let field_value = match fields_map.get("identity_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'identity_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#sources: {
                        let field_value = match fields_map.get("sources") {
                            Some(value) => value,
                            None => bail!("Missing field 'sources' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
