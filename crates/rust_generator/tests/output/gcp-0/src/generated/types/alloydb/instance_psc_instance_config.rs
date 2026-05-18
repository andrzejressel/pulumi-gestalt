#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct InstancePscInstanceConfig {
    /// List of consumer projects that are allowed to create PSC endpoints to service-attachments to this instance.
    /// These should be specified as project numbers only.
    #[builder(into)]
    #[serde(rename = "allowedConsumerProjects")]
    pub r#allowed_consumer_projects: Option<Vec<String>>,
    /// (Output)
    /// The DNS name of the instance for PSC connectivity.
    /// Name convention: <uid>.<uid>.<region>.alloydb-psc.goog
    #[builder(into)]
    #[serde(rename = "pscDnsName")]
    pub r#psc_dns_name: Option<String>,
    /// (Output)
    /// The service attachment created when Private Service Connect (PSC) is enabled for the instance.
    /// The name of the resource will be in the format of
    /// `projects/<alloydb-tenant-project-number>/regions/<region-name>/serviceAttachments/<service-attachment-name>`
    #[builder(into)]
    #[serde(rename = "serviceAttachmentLink")]
    pub r#service_attachment_link: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for InstancePscInstanceConfig {
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
                    "allowed_consumer_projects",
                    &self.r#allowed_consumer_projects,
                ),
                to_pulumi_object_field(
                    "psc_dns_name",
                    &self.r#psc_dns_name,
                ),
                to_pulumi_object_field(
                    "service_attachment_link",
                    &self.r#service_attachment_link,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for InstancePscInstanceConfig {
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
                    r#allowed_consumer_projects: {
                        let field_value = match fields_map.get("allowed_consumer_projects") {
                            Some(value) => value,
                            None => bail!("Missing field 'allowed_consumer_projects' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#psc_dns_name: {
                        let field_value = match fields_map.get("psc_dns_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'psc_dns_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#service_attachment_link: {
                        let field_value = match fields_map.get("service_attachment_link") {
                            Some(value) => value,
                            None => bail!("Missing field 'service_attachment_link' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
