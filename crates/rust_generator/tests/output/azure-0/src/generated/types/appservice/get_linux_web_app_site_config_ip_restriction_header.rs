#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetLinuxWebAppSiteConfigIpRestrictionHeader {
    /// The list of Azure Front Door IDs.
    #[builder(into)]
    pub r#x_azure_fdids: Vec<String>,
    /// Specifies if a Front Door Health Probe is expected.
    #[builder(into)]
    pub r#x_fd_health_probes: Vec<String>,
    /// The list of addresses for which matching is applied.
    #[builder(into)]
    pub r#x_forwarded_fors: Vec<String>,
    /// The list of Hosts for which matching will be applied.
    #[builder(into)]
    pub r#x_forwarded_hosts: Vec<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetLinuxWebAppSiteConfigIpRestrictionHeader {
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
                    "xAzureFdids",
                    &self.r#x_azure_fdids,
                ),
                to_pulumi_object_field(
                    "xFdHealthProbes",
                    &self.r#x_fd_health_probes,
                ),
                to_pulumi_object_field(
                    "xForwardedFors",
                    &self.r#x_forwarded_fors,
                ),
                to_pulumi_object_field(
                    "xForwardedHosts",
                    &self.r#x_forwarded_hosts,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetLinuxWebAppSiteConfigIpRestrictionHeader {
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
                        let field_value = match fields_map.get("xAzureFdids") {
                            Some(value) => value,
                            None => bail!("Missing field 'xAzureFdids' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#x_fd_health_probes: {
                        let field_value = match fields_map.get("xFdHealthProbes") {
                            Some(value) => value,
                            None => bail!("Missing field 'xFdHealthProbes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#x_forwarded_fors: {
                        let field_value = match fields_map.get("xForwardedFors") {
                            Some(value) => value,
                            None => bail!("Missing field 'xForwardedFors' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#x_forwarded_hosts: {
                        let field_value = match fields_map.get("xForwardedHosts") {
                            Some(value) => value,
                            None => bail!("Missing field 'xForwardedHosts' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
