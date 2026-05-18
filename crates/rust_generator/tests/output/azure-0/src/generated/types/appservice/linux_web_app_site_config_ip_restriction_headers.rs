#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct LinuxWebAppSiteConfigIpRestrictionHeaders {
    /// Specifies a list of Azure Front Door IDs.
    #[builder(into)]
    #[serde(rename = "xAzureFdids")]
    pub r#x_azure_fdids: Option<Vec<String>>,
    /// Specifies if a Front Door Health Probe should be expected. The only possible value is `1`.
    #[builder(into)]
    #[serde(rename = "xFdHealthProbe")]
    pub r#x_fd_health_probe: Option<String>,
    /// Specifies a list of addresses for which matching should be applied. Omitting this value means allow any.
    #[builder(into)]
    #[serde(rename = "xForwardedFors")]
    pub r#x_forwarded_fors: Option<Vec<String>>,
    /// Specifies a list of Hosts for which matching should be applied.
    #[builder(into)]
    #[serde(rename = "xForwardedHosts")]
    pub r#x_forwarded_hosts: Option<Vec<String>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for LinuxWebAppSiteConfigIpRestrictionHeaders {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > + Send {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "x_azure_fdids",
                    &self.r#x_azure_fdids,
                ),
                to_pulumi_object_field(
                    "x_fd_health_probe",
                    &self.r#x_fd_health_probe,
                ),
                to_pulumi_object_field(
                    "x_forwarded_fors",
                    &self.r#x_forwarded_fors,
                ),
                to_pulumi_object_field(
                    "x_forwarded_hosts",
                    &self.r#x_forwarded_hosts,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for LinuxWebAppSiteConfigIpRestrictionHeaders {
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
                    r#x_fd_health_probe: {
                        let field_value = match fields_map.get("x_fd_health_probe") {
                            Some(value) => value,
                            None => bail!("Missing field 'x_fd_health_probe' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
