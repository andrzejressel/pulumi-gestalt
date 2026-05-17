#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct EmailServiceDomainVerificationRecord {
    /// (Optional) An `dkim2` block as defined below.
    #[builder(into)]
    #[serde(rename = "dkim2s")]
    pub r#dkim_2_s: Option<Vec<super::super::types::communication::EmailServiceDomainVerificationRecordDkim2>>,
    /// (Optional) An `dkim` block as defined below.
    #[builder(into)]
    #[serde(rename = "dkims")]
    pub r#dkims: Option<Vec<super::super::types::communication::EmailServiceDomainVerificationRecordDkim>>,
    /// (Optional) An `dmarc` block as defined below.
    #[builder(into)]
    #[serde(rename = "dmarcs")]
    pub r#dmarcs: Option<Vec<super::super::types::communication::EmailServiceDomainVerificationRecordDmarc>>,
    /// (Optional) An `domain` block as defined below.
    #[builder(into)]
    #[serde(rename = "domains")]
    pub r#domains: Option<Vec<super::super::types::communication::EmailServiceDomainVerificationRecordDomain>>,
    /// (Optional) An `spf` block as defined below.
    #[builder(into)]
    #[serde(rename = "spfs")]
    pub r#spfs: Option<Vec<super::super::types::communication::EmailServiceDomainVerificationRecordSpf>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for EmailServiceDomainVerificationRecord {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "dkim_2_s",
                    &self.r#dkim_2_s,
                ),
                to_pulumi_object_field(
                    "dkims",
                    &self.r#dkims,
                ),
                to_pulumi_object_field(
                    "dmarcs",
                    &self.r#dmarcs,
                ),
                to_pulumi_object_field(
                    "domains",
                    &self.r#domains,
                ),
                to_pulumi_object_field(
                    "spfs",
                    &self.r#spfs,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for EmailServiceDomainVerificationRecord {
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
                    r#dkim_2_s: {
                        let field_value = match fields_map.get("dkim_2_s") {
                            Some(value) => value,
                            None => bail!("Missing field 'dkim_2_s' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#dkims: {
                        let field_value = match fields_map.get("dkims") {
                            Some(value) => value,
                            None => bail!("Missing field 'dkims' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#dmarcs: {
                        let field_value = match fields_map.get("dmarcs") {
                            Some(value) => value,
                            None => bail!("Missing field 'dmarcs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#domains: {
                        let field_value = match fields_map.get("domains") {
                            Some(value) => value,
                            None => bail!("Missing field 'domains' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#spfs: {
                        let field_value = match fields_map.get("spfs") {
                            Some(value) => value,
                            None => bail!("Missing field 'spfs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
