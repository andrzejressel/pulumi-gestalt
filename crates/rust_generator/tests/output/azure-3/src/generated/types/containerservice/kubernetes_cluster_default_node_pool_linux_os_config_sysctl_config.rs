#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct KubernetesClusterDefaultNodePoolLinuxOsConfigSysctlConfig {
    /// The sysctl setting fs.aio-max-nr. Must be between `65536` and `6553500`.
    #[builder(into)]
    #[serde(rename = "fsAioMaxNr")]
    pub r#fs_aio_max_nr: Option<i32>,
    /// The sysctl setting fs.file-max. Must be between `8192` and `12000500`.
    #[builder(into)]
    #[serde(rename = "fsFileMax")]
    pub r#fs_file_max: Option<i32>,
    /// The sysctl setting fs.inotify.max_user_watches. Must be between `781250` and `2097152`.
    #[builder(into)]
    #[serde(rename = "fsInotifyMaxUserWatches")]
    pub r#fs_inotify_max_user_watches: Option<i32>,
    /// The sysctl setting fs.nr_open. Must be between `8192` and `20000500`.
    #[builder(into)]
    #[serde(rename = "fsNrOpen")]
    pub r#fs_nr_open: Option<i32>,
    /// The sysctl setting kernel.threads-max. Must be between `20` and `513785`.
    #[builder(into)]
    #[serde(rename = "kernelThreadsMax")]
    pub r#kernel_threads_max: Option<i32>,
    /// The sysctl setting net.core.netdev_max_backlog. Must be between `1000` and `3240000`.
    #[builder(into)]
    #[serde(rename = "netCoreNetdevMaxBacklog")]
    pub r#net_core_netdev_max_backlog: Option<i32>,
    /// The sysctl setting net.core.optmem_max. Must be between `20480` and `4194304`.
    #[builder(into)]
    #[serde(rename = "netCoreOptmemMax")]
    pub r#net_core_optmem_max: Option<i32>,
    /// The sysctl setting net.core.rmem_default. Must be between `212992` and `134217728`.
    #[builder(into)]
    #[serde(rename = "netCoreRmemDefault")]
    pub r#net_core_rmem_default: Option<i32>,
    /// The sysctl setting net.core.rmem_max. Must be between `212992` and `134217728`.
    #[builder(into)]
    #[serde(rename = "netCoreRmemMax")]
    pub r#net_core_rmem_max: Option<i32>,
    /// The sysctl setting net.core.somaxconn. Must be between `4096` and `3240000`.
    #[builder(into)]
    #[serde(rename = "netCoreSomaxconn")]
    pub r#net_core_somaxconn: Option<i32>,
    /// The sysctl setting net.core.wmem_default. Must be between `212992` and `134217728`.
    #[builder(into)]
    #[serde(rename = "netCoreWmemDefault")]
    pub r#net_core_wmem_default: Option<i32>,
    /// The sysctl setting net.core.wmem_max. Must be between `212992` and `134217728`.
    #[builder(into)]
    #[serde(rename = "netCoreWmemMax")]
    pub r#net_core_wmem_max: Option<i32>,
    /// The sysctl setting net.ipv4.ip_local_port_range max value. Must be between `32768` and `65535`.
    #[builder(into)]
    #[serde(rename = "netIpv4IpLocalPortRangeMax")]
    pub r#net_ipv_4_ip_local_port_range_max: Option<i32>,
    /// The sysctl setting net.ipv4.ip_local_port_range min value. Must be between `1024` and `60999`.
    #[builder(into)]
    #[serde(rename = "netIpv4IpLocalPortRangeMin")]
    pub r#net_ipv_4_ip_local_port_range_min: Option<i32>,
    /// The sysctl setting net.ipv4.neigh.default.gc_thresh1. Must be between `128` and `80000`.
    #[builder(into)]
    #[serde(rename = "netIpv4NeighDefaultGcThresh1")]
    pub r#net_ipv_4_neigh_default_gc_thresh_1: Option<i32>,
    /// The sysctl setting net.ipv4.neigh.default.gc_thresh2. Must be between `512` and `90000`.
    #[builder(into)]
    #[serde(rename = "netIpv4NeighDefaultGcThresh2")]
    pub r#net_ipv_4_neigh_default_gc_thresh_2: Option<i32>,
    /// The sysctl setting net.ipv4.neigh.default.gc_thresh3. Must be between `1024` and `100000`.
    #[builder(into)]
    #[serde(rename = "netIpv4NeighDefaultGcThresh3")]
    pub r#net_ipv_4_neigh_default_gc_thresh_3: Option<i32>,
    /// The sysctl setting net.ipv4.tcp_fin_timeout. Must be between `5` and `120`.
    #[builder(into)]
    #[serde(rename = "netIpv4TcpFinTimeout")]
    pub r#net_ipv_4_tcp_fin_timeout: Option<i32>,
    /// The sysctl setting net.ipv4.tcp_keepalive_intvl. Must be between `10` and `90`.
    #[builder(into)]
    #[serde(rename = "netIpv4TcpKeepaliveIntvl")]
    pub r#net_ipv_4_tcp_keepalive_intvl: Option<i32>,
    /// The sysctl setting net.ipv4.tcp_keepalive_probes. Must be between `1` and `15`.
    #[builder(into)]
    #[serde(rename = "netIpv4TcpKeepaliveProbes")]
    pub r#net_ipv_4_tcp_keepalive_probes: Option<i32>,
    /// The sysctl setting net.ipv4.tcp_keepalive_time. Must be between `30` and `432000`.
    #[builder(into)]
    #[serde(rename = "netIpv4TcpKeepaliveTime")]
    pub r#net_ipv_4_tcp_keepalive_time: Option<i32>,
    /// The sysctl setting net.ipv4.tcp_max_syn_backlog. Must be between `128` and `3240000`.
    #[builder(into)]
    #[serde(rename = "netIpv4TcpMaxSynBacklog")]
    pub r#net_ipv_4_tcp_max_syn_backlog: Option<i32>,
    /// The sysctl setting net.ipv4.tcp_max_tw_buckets. Must be between `8000` and `1440000`.
    #[builder(into)]
    #[serde(rename = "netIpv4TcpMaxTwBuckets")]
    pub r#net_ipv_4_tcp_max_tw_buckets: Option<i32>,
    /// The sysctl setting net.ipv4.tcp_tw_reuse.
    #[builder(into)]
    #[serde(rename = "netIpv4TcpTwReuse")]
    pub r#net_ipv_4_tcp_tw_reuse: Option<bool>,
    /// The sysctl setting net.netfilter.nf_conntrack_buckets. Must be between `65536` and `524288`.
    #[builder(into)]
    #[serde(rename = "netNetfilterNfConntrackBuckets")]
    pub r#net_netfilter_nf_conntrack_buckets: Option<i32>,
    /// The sysctl setting net.netfilter.nf_conntrack_max. Must be between `131072` and `2097152`.
    #[builder(into)]
    #[serde(rename = "netNetfilterNfConntrackMax")]
    pub r#net_netfilter_nf_conntrack_max: Option<i32>,
    /// The sysctl setting vm.max_map_count. Must be between `65530` and `262144`.
    #[builder(into)]
    #[serde(rename = "vmMaxMapCount")]
    pub r#vm_max_map_count: Option<i32>,
    /// The sysctl setting vm.swappiness. Must be between `0` and `100`.
    #[builder(into)]
    #[serde(rename = "vmSwappiness")]
    pub r#vm_swappiness: Option<i32>,
    /// The sysctl setting vm.vfs_cache_pressure. Must be between `0` and `100`.
    #[builder(into)]
    #[serde(rename = "vmVfsCachePressure")]
    pub r#vm_vfs_cache_pressure: Option<i32>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for KubernetesClusterDefaultNodePoolLinuxOsConfigSysctlConfig {
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
                    "fs_aio_max_nr",
                    &self.r#fs_aio_max_nr,
                ),
                to_pulumi_object_field(
                    "fs_file_max",
                    &self.r#fs_file_max,
                ),
                to_pulumi_object_field(
                    "fs_inotify_max_user_watches",
                    &self.r#fs_inotify_max_user_watches,
                ),
                to_pulumi_object_field(
                    "fs_nr_open",
                    &self.r#fs_nr_open,
                ),
                to_pulumi_object_field(
                    "kernel_threads_max",
                    &self.r#kernel_threads_max,
                ),
                to_pulumi_object_field(
                    "net_core_netdev_max_backlog",
                    &self.r#net_core_netdev_max_backlog,
                ),
                to_pulumi_object_field(
                    "net_core_optmem_max",
                    &self.r#net_core_optmem_max,
                ),
                to_pulumi_object_field(
                    "net_core_rmem_default",
                    &self.r#net_core_rmem_default,
                ),
                to_pulumi_object_field(
                    "net_core_rmem_max",
                    &self.r#net_core_rmem_max,
                ),
                to_pulumi_object_field(
                    "net_core_somaxconn",
                    &self.r#net_core_somaxconn,
                ),
                to_pulumi_object_field(
                    "net_core_wmem_default",
                    &self.r#net_core_wmem_default,
                ),
                to_pulumi_object_field(
                    "net_core_wmem_max",
                    &self.r#net_core_wmem_max,
                ),
                to_pulumi_object_field(
                    "net_ipv_4_ip_local_port_range_max",
                    &self.r#net_ipv_4_ip_local_port_range_max,
                ),
                to_pulumi_object_field(
                    "net_ipv_4_ip_local_port_range_min",
                    &self.r#net_ipv_4_ip_local_port_range_min,
                ),
                to_pulumi_object_field(
                    "net_ipv_4_neigh_default_gc_thresh_1",
                    &self.r#net_ipv_4_neigh_default_gc_thresh_1,
                ),
                to_pulumi_object_field(
                    "net_ipv_4_neigh_default_gc_thresh_2",
                    &self.r#net_ipv_4_neigh_default_gc_thresh_2,
                ),
                to_pulumi_object_field(
                    "net_ipv_4_neigh_default_gc_thresh_3",
                    &self.r#net_ipv_4_neigh_default_gc_thresh_3,
                ),
                to_pulumi_object_field(
                    "net_ipv_4_tcp_fin_timeout",
                    &self.r#net_ipv_4_tcp_fin_timeout,
                ),
                to_pulumi_object_field(
                    "net_ipv_4_tcp_keepalive_intvl",
                    &self.r#net_ipv_4_tcp_keepalive_intvl,
                ),
                to_pulumi_object_field(
                    "net_ipv_4_tcp_keepalive_probes",
                    &self.r#net_ipv_4_tcp_keepalive_probes,
                ),
                to_pulumi_object_field(
                    "net_ipv_4_tcp_keepalive_time",
                    &self.r#net_ipv_4_tcp_keepalive_time,
                ),
                to_pulumi_object_field(
                    "net_ipv_4_tcp_max_syn_backlog",
                    &self.r#net_ipv_4_tcp_max_syn_backlog,
                ),
                to_pulumi_object_field(
                    "net_ipv_4_tcp_max_tw_buckets",
                    &self.r#net_ipv_4_tcp_max_tw_buckets,
                ),
                to_pulumi_object_field(
                    "net_ipv_4_tcp_tw_reuse",
                    &self.r#net_ipv_4_tcp_tw_reuse,
                ),
                to_pulumi_object_field(
                    "net_netfilter_nf_conntrack_buckets",
                    &self.r#net_netfilter_nf_conntrack_buckets,
                ),
                to_pulumi_object_field(
                    "net_netfilter_nf_conntrack_max",
                    &self.r#net_netfilter_nf_conntrack_max,
                ),
                to_pulumi_object_field(
                    "vm_max_map_count",
                    &self.r#vm_max_map_count,
                ),
                to_pulumi_object_field(
                    "vm_swappiness",
                    &self.r#vm_swappiness,
                ),
                to_pulumi_object_field(
                    "vm_vfs_cache_pressure",
                    &self.r#vm_vfs_cache_pressure,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for KubernetesClusterDefaultNodePoolLinuxOsConfigSysctlConfig {
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
                    r#fs_aio_max_nr: {
                        let field_value = match fields_map.get("fs_aio_max_nr") {
                            Some(value) => value,
                            None => bail!("Missing field 'fs_aio_max_nr' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#fs_file_max: {
                        let field_value = match fields_map.get("fs_file_max") {
                            Some(value) => value,
                            None => bail!("Missing field 'fs_file_max' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#fs_inotify_max_user_watches: {
                        let field_value = match fields_map.get("fs_inotify_max_user_watches") {
                            Some(value) => value,
                            None => bail!("Missing field 'fs_inotify_max_user_watches' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#fs_nr_open: {
                        let field_value = match fields_map.get("fs_nr_open") {
                            Some(value) => value,
                            None => bail!("Missing field 'fs_nr_open' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#kernel_threads_max: {
                        let field_value = match fields_map.get("kernel_threads_max") {
                            Some(value) => value,
                            None => bail!("Missing field 'kernel_threads_max' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#net_core_netdev_max_backlog: {
                        let field_value = match fields_map.get("net_core_netdev_max_backlog") {
                            Some(value) => value,
                            None => bail!("Missing field 'net_core_netdev_max_backlog' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#net_core_optmem_max: {
                        let field_value = match fields_map.get("net_core_optmem_max") {
                            Some(value) => value,
                            None => bail!("Missing field 'net_core_optmem_max' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#net_core_rmem_default: {
                        let field_value = match fields_map.get("net_core_rmem_default") {
                            Some(value) => value,
                            None => bail!("Missing field 'net_core_rmem_default' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#net_core_rmem_max: {
                        let field_value = match fields_map.get("net_core_rmem_max") {
                            Some(value) => value,
                            None => bail!("Missing field 'net_core_rmem_max' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#net_core_somaxconn: {
                        let field_value = match fields_map.get("net_core_somaxconn") {
                            Some(value) => value,
                            None => bail!("Missing field 'net_core_somaxconn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#net_core_wmem_default: {
                        let field_value = match fields_map.get("net_core_wmem_default") {
                            Some(value) => value,
                            None => bail!("Missing field 'net_core_wmem_default' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#net_core_wmem_max: {
                        let field_value = match fields_map.get("net_core_wmem_max") {
                            Some(value) => value,
                            None => bail!("Missing field 'net_core_wmem_max' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#net_ipv_4_ip_local_port_range_max: {
                        let field_value = match fields_map.get("net_ipv_4_ip_local_port_range_max") {
                            Some(value) => value,
                            None => bail!("Missing field 'net_ipv_4_ip_local_port_range_max' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#net_ipv_4_ip_local_port_range_min: {
                        let field_value = match fields_map.get("net_ipv_4_ip_local_port_range_min") {
                            Some(value) => value,
                            None => bail!("Missing field 'net_ipv_4_ip_local_port_range_min' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#net_ipv_4_neigh_default_gc_thresh_1: {
                        let field_value = match fields_map.get("net_ipv_4_neigh_default_gc_thresh_1") {
                            Some(value) => value,
                            None => bail!("Missing field 'net_ipv_4_neigh_default_gc_thresh_1' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#net_ipv_4_neigh_default_gc_thresh_2: {
                        let field_value = match fields_map.get("net_ipv_4_neigh_default_gc_thresh_2") {
                            Some(value) => value,
                            None => bail!("Missing field 'net_ipv_4_neigh_default_gc_thresh_2' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#net_ipv_4_neigh_default_gc_thresh_3: {
                        let field_value = match fields_map.get("net_ipv_4_neigh_default_gc_thresh_3") {
                            Some(value) => value,
                            None => bail!("Missing field 'net_ipv_4_neigh_default_gc_thresh_3' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#net_ipv_4_tcp_fin_timeout: {
                        let field_value = match fields_map.get("net_ipv_4_tcp_fin_timeout") {
                            Some(value) => value,
                            None => bail!("Missing field 'net_ipv_4_tcp_fin_timeout' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#net_ipv_4_tcp_keepalive_intvl: {
                        let field_value = match fields_map.get("net_ipv_4_tcp_keepalive_intvl") {
                            Some(value) => value,
                            None => bail!("Missing field 'net_ipv_4_tcp_keepalive_intvl' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#net_ipv_4_tcp_keepalive_probes: {
                        let field_value = match fields_map.get("net_ipv_4_tcp_keepalive_probes") {
                            Some(value) => value,
                            None => bail!("Missing field 'net_ipv_4_tcp_keepalive_probes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#net_ipv_4_tcp_keepalive_time: {
                        let field_value = match fields_map.get("net_ipv_4_tcp_keepalive_time") {
                            Some(value) => value,
                            None => bail!("Missing field 'net_ipv_4_tcp_keepalive_time' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#net_ipv_4_tcp_max_syn_backlog: {
                        let field_value = match fields_map.get("net_ipv_4_tcp_max_syn_backlog") {
                            Some(value) => value,
                            None => bail!("Missing field 'net_ipv_4_tcp_max_syn_backlog' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#net_ipv_4_tcp_max_tw_buckets: {
                        let field_value = match fields_map.get("net_ipv_4_tcp_max_tw_buckets") {
                            Some(value) => value,
                            None => bail!("Missing field 'net_ipv_4_tcp_max_tw_buckets' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#net_ipv_4_tcp_tw_reuse: {
                        let field_value = match fields_map.get("net_ipv_4_tcp_tw_reuse") {
                            Some(value) => value,
                            None => bail!("Missing field 'net_ipv_4_tcp_tw_reuse' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#net_netfilter_nf_conntrack_buckets: {
                        let field_value = match fields_map.get("net_netfilter_nf_conntrack_buckets") {
                            Some(value) => value,
                            None => bail!("Missing field 'net_netfilter_nf_conntrack_buckets' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#net_netfilter_nf_conntrack_max: {
                        let field_value = match fields_map.get("net_netfilter_nf_conntrack_max") {
                            Some(value) => value,
                            None => bail!("Missing field 'net_netfilter_nf_conntrack_max' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#vm_max_map_count: {
                        let field_value = match fields_map.get("vm_max_map_count") {
                            Some(value) => value,
                            None => bail!("Missing field 'vm_max_map_count' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#vm_swappiness: {
                        let field_value = match fields_map.get("vm_swappiness") {
                            Some(value) => value,
                            None => bail!("Missing field 'vm_swappiness' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#vm_vfs_cache_pressure: {
                        let field_value = match fields_map.get("vm_vfs_cache_pressure") {
                            Some(value) => value,
                            None => bail!("Missing field 'vm_vfs_cache_pressure' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
