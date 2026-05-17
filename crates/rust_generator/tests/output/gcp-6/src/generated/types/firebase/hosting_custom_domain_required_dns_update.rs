#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct HostingCustomDomainRequiredDnsUpdate {
    /// (Output)
    /// The last time Hosting checked your CustomDomain's DNS records.
    #[builder(into)]
    #[serde(rename = "checkTime")]
    pub r#check_time: Option<String>,
    /// A text string to serve at the path.
    #[builder(into)]
    #[serde(rename = "desireds")]
    pub r#desireds: Option<Vec<super::super::types::firebase::HostingCustomDomainRequiredDnsUpdateDesired>>,
    /// Whether Hosting was able to find the required file contents on the
    /// specified path during its last check.
    #[builder(into)]
    #[serde(rename = "discovereds")]
    pub r#discovereds: Option<Vec<super::super::types::firebase::HostingCustomDomainRequiredDnsUpdateDiscovered>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for HostingCustomDomainRequiredDnsUpdate {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_concurrent, to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "check_time",
                    &self.r#check_time,
                ),
                to_pulumi_object_field(
                    "desireds",
                    &self.r#desireds,
                ),
                to_pulumi_object_field(
                    "discovereds",
                    &self.r#discovereds,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for HostingCustomDomainRequiredDnsUpdate {
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
                    r#check_time: {
                        let field_value = match fields_map.get("check_time") {
                            Some(value) => value,
                            None => bail!("Missing field 'check_time' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#desireds: {
                        let field_value = match fields_map.get("desireds") {
                            Some(value) => value,
                            None => bail!("Missing field 'desireds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#discovereds: {
                        let field_value = match fields_map.get("discovereds") {
                            Some(value) => value,
                            None => bail!("Missing field 'discovereds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
