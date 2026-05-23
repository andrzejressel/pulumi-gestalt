#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RuntimeVirtualMachineVirtualMachineConfigShieldedInstanceConfig {
    /// Defines whether the instance has integrity monitoring enabled.
    /// Enables monitoring and attestation of the boot integrity of
    /// the instance. The attestation is performed against the
    /// integrity policy baseline. This baseline is initially derived
    /// from the implicitly trusted boot image when the instance is
    /// created. Enabled by default.
    #[builder(into)]
    pub r#enable_integrity_monitoring: Option<bool>,
    /// Defines whether the instance has Secure Boot enabled.Secure
    /// Boot helps ensure that the system only runs authentic software
    /// by verifying the digital signature of all boot components, and
    /// halting the boot process if signature verification fails.
    /// Disabled by default.
    #[builder(into)]
    pub r#enable_secure_boot: Option<bool>,
    /// Defines whether the instance has the vTPM enabled. Enabled by
    /// default.
    #[builder(into)]
    pub r#enable_vtpm: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for RuntimeVirtualMachineVirtualMachineConfigShieldedInstanceConfig {
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
                    "enableIntegrityMonitoring",
                    &self.r#enable_integrity_monitoring,
                ),
                to_pulumi_object_field(
                    "enableSecureBoot",
                    &self.r#enable_secure_boot,
                ),
                to_pulumi_object_field(
                    "enableVtpm",
                    &self.r#enable_vtpm,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for RuntimeVirtualMachineVirtualMachineConfigShieldedInstanceConfig {
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
                    r#enable_integrity_monitoring: {
                        let field_value = match fields_map.get("enableIntegrityMonitoring") {
                            Some(value) => value,
                            None => bail!("Missing field 'enableIntegrityMonitoring' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#enable_secure_boot: {
                        let field_value = match fields_map.get("enableSecureBoot") {
                            Some(value) => value,
                            None => bail!("Missing field 'enableSecureBoot' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#enable_vtpm: {
                        let field_value = match fields_map.get("enableVtpm") {
                            Some(value) => value,
                            None => bail!("Missing field 'enableVtpm' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
