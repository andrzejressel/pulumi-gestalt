#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DomainMappingSpec {
    /// The mode of the certificate.
    /// Default value is `AUTOMATIC`.
    /// Possible values are: `NONE`, `AUTOMATIC`.
    /// 
    /// - - -
    #[builder(into)]
    #[serde(rename = "certificateMode")]
    pub r#certificate_mode: Option<String>,
    /// If set, the mapping will override any mapping set before this spec was set.
    /// It is recommended that the user leaves this empty to receive an error
    /// warning about a potential conflict and only set it once the respective UI
    /// has given such a warning.
    #[builder(into)]
    #[serde(rename = "forceOverride")]
    pub r#force_override: Option<bool>,
    /// The name of the Cloud Run Service that this DomainMapping applies to.
    /// The route must exist.
    #[builder(into)]
    #[serde(rename = "routeName")]
    pub r#route_name: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for DomainMappingSpec {
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
                "certificate_mode".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#certificate_mode,
                )
                .await,
            );
            map.insert(
                "force_override".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#force_override,
                )
                .await,
            );
            map.insert(
                "route_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#route_name,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for DomainMappingSpec {
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
                    r#certificate_mode: {
                        let field_value = match fields_map.get("certificate_mode") {
                            Some(value) => value,
                            None => bail!("Missing field 'certificate_mode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#force_override: {
                        let field_value = match fields_map.get("force_override") {
                            Some(value) => value,
                            None => bail!("Missing field 'force_override' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#route_name: {
                        let field_value = match fields_map.get("route_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'route_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
