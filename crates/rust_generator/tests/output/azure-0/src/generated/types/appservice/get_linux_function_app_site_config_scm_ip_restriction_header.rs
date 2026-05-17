#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetLinuxFunctionAppSiteConfigScmIpRestrictionHeader {
    /// A list of Azure Front Door IDs.
    #[builder(into)]
    #[serde(rename = "xAzureFdids")]
    pub r#x_azure_fdids: Vec<String>,
    /// Should a Front Door Health Probe be expected?
    #[builder(into)]
    #[serde(rename = "xFdHealthProbes")]
    pub r#x_fd_health_probes: Vec<String>,
    /// A list of addresses for which matching is applied.
    #[builder(into)]
    #[serde(rename = "xForwardedFors")]
    pub r#x_forwarded_fors: Vec<String>,
    /// A list of Hosts for which matching is applied.
    #[builder(into)]
    #[serde(rename = "xForwardedHosts")]
    pub r#x_forwarded_hosts: Vec<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetLinuxFunctionAppSiteConfigScmIpRestrictionHeader {
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
                "x_azure_fdids".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#x_azure_fdids,
                )
                .await,
            );
            map.insert(
                "x_fd_health_probes".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#x_fd_health_probes,
                )
                .await,
            );
            map.insert(
                "x_forwarded_fors".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#x_forwarded_fors,
                )
                .await,
            );
            map.insert(
                "x_forwarded_hosts".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#x_forwarded_hosts,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetLinuxFunctionAppSiteConfigScmIpRestrictionHeader {
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
                    r#x_azure_fdids: {
                        let field_value = match fields_map.get("x_azure_fdids") {
                            Some(value) => value,
                            None => bail!("Missing field 'x_azure_fdids' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#x_fd_health_probes: {
                        let field_value = match fields_map.get("x_fd_health_probes") {
                            Some(value) => value,
                            None => bail!("Missing field 'x_fd_health_probes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#x_forwarded_fors: {
                        let field_value = match fields_map.get("x_forwarded_fors") {
                            Some(value) => value,
                            None => bail!("Missing field 'x_forwarded_fors' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#x_forwarded_hosts: {
                        let field_value = match fields_map.get("x_forwarded_hosts") {
                            Some(value) => value,
                            None => bail!("Missing field 'x_forwarded_hosts' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
