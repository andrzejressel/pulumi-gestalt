#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DomainMappingSslSettings {
    /// ID of the AuthorizedCertificate resource configuring SSL for the application. Clearing this field will
    /// remove SSL support.
    /// By default, a managed certificate is automatically created for every domain mapping. To omit SSL support
    /// or to configure SSL manually, specify `SslManagementType.MANUAL` on a `CREATE` or `UPDATE` request. You must be
    /// authorized to administer the `AuthorizedCertificate` resource to manually map it to a DomainMapping resource.
    /// Example: 12345.
    #[builder(into)]
    #[serde(rename = "certificateId")]
    pub r#certificate_id: Option<String>,
    /// (Output)
    /// ID of the managed `AuthorizedCertificate` resource currently being provisioned, if applicable. Until the new
    /// managed certificate has been successfully provisioned, the previous SSL state will be preserved. Once the
    /// provisioning process completes, the `certificateId` field will reflect the new managed certificate and this
    /// field will be left empty. To remove SSL support while there is still a pending managed certificate, clear the
    /// `certificateId` field with an update request.
    #[builder(into)]
    #[serde(rename = "pendingManagedCertificateId")]
    pub r#pending_managed_certificate_id: Option<String>,
    /// SSL management type for this domain. If `AUTOMATIC`, a managed certificate is automatically provisioned.
    /// If `MANUAL`, `certificateId` must be manually specified in order to configure SSL for this domain.
    /// Possible values are: `AUTOMATIC`, `MANUAL`.
    #[builder(into)]
    #[serde(rename = "sslManagementType")]
    pub r#ssl_management_type: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for DomainMappingSslSettings {
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
                    "certificate_id",
                    &self.r#certificate_id,
                ),
                to_pulumi_object_field(
                    "pending_managed_certificate_id",
                    &self.r#pending_managed_certificate_id,
                ),
                to_pulumi_object_field(
                    "ssl_management_type",
                    &self.r#ssl_management_type,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for DomainMappingSslSettings {
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
                    r#certificate_id: {
                        let field_value = match fields_map.get("certificate_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'certificate_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#pending_managed_certificate_id: {
                        let field_value = match fields_map.get("pending_managed_certificate_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'pending_managed_certificate_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ssl_management_type: {
                        let field_value = match fields_map.get("ssl_management_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'ssl_management_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
