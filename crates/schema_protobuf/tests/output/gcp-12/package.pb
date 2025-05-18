
gcpGoogle Cloud"8.12.1*û≈
9
vmwareengineCluster gcp:vmwareengine/cluster:ClusteríµA cluster in a private cloud.


To get more information about Cluster, see:

* [API documentation](https://cloud.google.com/vmware-engine/docs/reference/rest/v1/projects.locations.privateClouds.clusters)

## Example Usage

### Vmware Engine Cluster Basic


<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as gcp from "@pulumi/gcp";

const cluster_nw = new gcp.vmwareengine.Network("cluster-nw", {
    name: "pc-nw",
    type: "STANDARD",
    location: "global",
    description: "PC network description.",
});
const cluster_pc = new gcp.vmwareengine.PrivateCloud("cluster-pc", {
    location: "us-west1-a",
    name: "sample-pc",
    description: "Sample test PC.",
    networkConfig: {
        managementCidr: "192.168.30.0/24",
        vmwareEngineNetwork: cluster_nw.id,
    },
    managementCluster: {
        clusterId: "sample-mgmt-cluster",
        nodeTypeConfigs: [{
            nodeTypeId: "standard-72",
            nodeCount: 3,
        }],
    },
});
const vmw_engine_ext_cluster = new gcp.vmwareengine.Cluster("vmw-engine-ext-cluster", {
    name: "ext-cluster",
    parent: cluster_pc.id,
    nodeTypeConfigs: [{
        nodeTypeId: "standard-72",
        nodeCount: 3,
    }],
});
```
```python
import pulumi
import pulumi_gcp as gcp

cluster_nw = gcp.vmwareengine.Network("cluster-nw",
    name="pc-nw",
    type="STANDARD",
    location="global",
    description="PC network description.")
cluster_pc = gcp.vmwareengine.PrivateCloud("cluster-pc",
    location="us-west1-a",
    name="sample-pc",
    description="Sample test PC.",
    network_config={
        "management_cidr": "192.168.30.0/24",
        "vmware_engine_network": cluster_nw.id,
    },
    management_cluster={
        "cluster_id": "sample-mgmt-cluster",
        "node_type_configs": [{
            "node_type_id": "standard-72",
            "node_count": 3,
        }],
    })
vmw_engine_ext_cluster = gcp.vmwareengine.Cluster("vmw-engine-ext-cluster",
    name="ext-cluster",
    parent=cluster_pc.id,
    node_type_configs=[{
        "node_type_id": "standard-72",
        "node_count": 3,
    }])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Gcp = Pulumi.Gcp;

return await Deployment.RunAsync(() => 
{
    var cluster_nw = new Gcp.VMwareEngine.Network("cluster-nw", new()
    {
        Name = "pc-nw",
        Type = "STANDARD",
        Location = "global",
        Description = "PC network description.",
    });

    var cluster_pc = new Gcp.VMwareEngine.PrivateCloud("cluster-pc", new()
    {
        Location = "us-west1-a",
        Name = "sample-pc",
        Description = "Sample test PC.",
        NetworkConfig = new Gcp.VMwareEngine.Inputs.PrivateCloudNetworkConfigArgs
        {
            ManagementCidr = "192.168.30.0/24",
            VmwareEngineNetwork = cluster_nw.Id,
        },
        ManagementCluster = new Gcp.VMwareEngine.Inputs.PrivateCloudManagementClusterArgs
        {
            ClusterId = "sample-mgmt-cluster",
            NodeTypeConfigs = new[]
            {
                new Gcp.VMwareEngine.Inputs.PrivateCloudManagementClusterNodeTypeConfigArgs
                {
                    NodeTypeId = "standard-72",
                    NodeCount = 3,
                },
            },
        },
    });

    var vmw_engine_ext_cluster = new Gcp.VMwareEngine.Cluster("vmw-engine-ext-cluster", new()
    {
        Name = "ext-cluster",
        Parent = cluster_pc.Id,
        NodeTypeConfigs = new[]
        {
            new Gcp.VMwareEngine.Inputs.ClusterNodeTypeConfigArgs
            {
                NodeTypeId = "standard-72",
                NodeCount = 3,
            },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-gcp/sdk/v8/go/gcp/vmwareengine"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := vmwareengine.NewNetwork(ctx, "cluster-nw", &vmwareengine.NetworkArgs{
			Name:        pulumi.String("pc-nw"),
			Type:        pulumi.String("STANDARD"),
			Location:    pulumi.String("global"),
			Description: pulumi.String("PC network description."),
		})
		if err != nil {
			return err
		}
		_, err = vmwareengine.NewPrivateCloud(ctx, "cluster-pc", &vmwareengine.PrivateCloudArgs{
			Location:    pulumi.String("us-west1-a"),
			Name:        pulumi.String("sample-pc"),
			Description: pulumi.String("Sample test PC."),
			NetworkConfig: &vmwareengine.PrivateCloudNetworkConfigArgs{
				ManagementCidr:      pulumi.String("192.168.30.0/24"),
				VmwareEngineNetwork: cluster_nw.ID(),
			},
			ManagementCluster: &vmwareengine.PrivateCloudManagementClusterArgs{
				ClusterId: pulumi.String("sample-mgmt-cluster"),
				NodeTypeConfigs: vmwareengine.PrivateCloudManagementClusterNodeTypeConfigArray{
					&vmwareengine.PrivateCloudManagementClusterNodeTypeConfigArgs{
						NodeTypeId: pulumi.String("standard-72"),
						NodeCount:  pulumi.Int(3),
					},
				},
			},
		})
		if err != nil {
			return err
		}
		_, err = vmwareengine.NewCluster(ctx, "vmw-engine-ext-cluster", &vmwareengine.ClusterArgs{
			Name:   pulumi.String("ext-cluster"),
			Parent: cluster_pc.ID(),
			NodeTypeConfigs: vmwareengine.ClusterNodeTypeConfigArray{
				&vmwareengine.ClusterNodeTypeConfigArgs{
					NodeTypeId: pulumi.String("standard-72"),
					NodeCount:  pulumi.Int(3),
				},
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.gcp.vmwareengine.Network;
import com.pulumi.gcp.vmwareengine.NetworkArgs;
import com.pulumi.gcp.vmwareengine.PrivateCloud;
import com.pulumi.gcp.vmwareengine.PrivateCloudArgs;
import com.pulumi.gcp.vmwareengine.inputs.PrivateCloudNetworkConfigArgs;
import com.pulumi.gcp.vmwareengine.inputs.PrivateCloudManagementClusterArgs;
import com.pulumi.gcp.vmwareengine.Cluster;
import com.pulumi.gcp.vmwareengine.ClusterArgs;
import com.pulumi.gcp.vmwareengine.inputs.ClusterNodeTypeConfigArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var cluster_nw = new Network("cluster-nw", NetworkArgs.builder()
            .name("pc-nw")
            .type("STANDARD")
            .location("global")
            .description("PC network description.")
            .build());

        var cluster_pc = new PrivateCloud("cluster-pc", PrivateCloudArgs.builder()
            .location("us-west1-a")
            .name("sample-pc")
            .description("Sample test PC.")
            .networkConfig(PrivateCloudNetworkConfigArgs.builder()
                .managementCidr("192.168.30.0/24")
                .vmwareEngineNetwork(cluster_nw.id())
                .build())
            .managementCluster(PrivateCloudManagementClusterArgs.builder()
                .clusterId("sample-mgmt-cluster")
                .nodeTypeConfigs(PrivateCloudManagementClusterNodeTypeConfigArgs.builder()
                    .nodeTypeId("standard-72")
                    .nodeCount(3)
                    .build())
                .build())
            .build());

        var vmw_engine_ext_cluster = new Cluster("vmw-engine-ext-cluster", ClusterArgs.builder()
            .name("ext-cluster")
            .parent(cluster_pc.id())
            .nodeTypeConfigs(ClusterNodeTypeConfigArgs.builder()
                .nodeTypeId("standard-72")
                .nodeCount(3)
                .build())
            .build());

    }
}
```
```yaml
resources:
  vmw-engine-ext-cluster:
    type: gcp:vmwareengine:Cluster
    properties:
      name: ext-cluster
      parent: ${["cluster-pc"].id}
      nodeTypeConfigs:
        - nodeTypeId: standard-72
          nodeCount: 3
  cluster-pc:
    type: gcp:vmwareengine:PrivateCloud
    properties:
      location: us-west1-a
      name: sample-pc
      description: Sample test PC.
      networkConfig:
        managementCidr: 192.168.30.0/24
        vmwareEngineNetwork: ${["cluster-nw"].id}
      managementCluster:
        clusterId: sample-mgmt-cluster
        nodeTypeConfigs:
          - nodeTypeId: standard-72
            nodeCount: 3
  cluster-nw:
    type: gcp:vmwareengine:Network
    properties:
      name: pc-nw
      type: STANDARD
      location: global
      description: PC network description.
```
<!--End PulumiCodeChooser -->
### Vmware Engine Cluster Full


<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as gcp from "@pulumi/gcp";

const cluster_nw = new gcp.vmwareengine.Network("cluster-nw", {
    name: "pc-nw",
    type: "STANDARD",
    location: "global",
    description: "PC network description.",
});
const cluster_pc = new gcp.vmwareengine.PrivateCloud("cluster-pc", {
    location: "us-west1-a",
    name: "sample-pc",
    description: "Sample test PC.",
    networkConfig: {
        managementCidr: "192.168.30.0/24",
        vmwareEngineNetwork: cluster_nw.id,
    },
    managementCluster: {
        clusterId: "sample-mgmt-cluster",
        nodeTypeConfigs: [{
            nodeTypeId: "standard-72",
            nodeCount: 3,
            customCoreCount: 32,
        }],
    },
});
const vmw_ext_cluster = new gcp.vmwareengine.Cluster("vmw-ext-cluster", {
    name: "ext-cluster",
    parent: cluster_pc.id,
    nodeTypeConfigs: [{
        nodeTypeId: "standard-72",
        nodeCount: 3,
        customCoreCount: 32,
    }],
    autoscalingSettings: {
        autoscalingPolicies: [{
            autoscalePolicyId: "autoscaling-policy",
            nodeTypeId: "standard-72",
            scaleOutSize: 1,
            cpuThresholds: {
                scaleOut: 80,
                scaleIn: 15,
            },
            consumedMemoryThresholds: {
                scaleOut: 75,
                scaleIn: 20,
            },
            storageThresholds: {
                scaleOut: 80,
                scaleIn: 20,
            },
        }],
        minClusterNodeCount: 3,
        maxClusterNodeCount: 8,
        coolDownPeriod: "1800s",
    },
});
```
```python
import pulumi
import pulumi_gcp as gcp

cluster_nw = gcp.vmwareengine.Network("cluster-nw",
    name="pc-nw",
    type="STANDARD",
    location="global",
    description="PC network description.")
cluster_pc = gcp.vmwareengine.PrivateCloud("cluster-pc",
    location="us-west1-a",
    name="sample-pc",
    description="Sample test PC.",
    network_config={
        "management_cidr": "192.168.30.0/24",
        "vmware_engine_network": cluster_nw.id,
    },
    management_cluster={
        "cluster_id": "sample-mgmt-cluster",
        "node_type_configs": [{
            "node_type_id": "standard-72",
            "node_count": 3,
            "custom_core_count": 32,
        }],
    })
vmw_ext_cluster = gcp.vmwareengine.Cluster("vmw-ext-cluster",
    name="ext-cluster",
    parent=cluster_pc.id,
    node_type_configs=[{
        "node_type_id": "standard-72",
        "node_count": 3,
        "custom_core_count": 32,
    }],
    autoscaling_settings={
        "autoscaling_policies": [{
            "autoscale_policy_id": "autoscaling-policy",
            "node_type_id": "standard-72",
            "scale_out_size": 1,
            "cpu_thresholds": {
                "scale_out": 80,
                "scale_in": 15,
            },
            "consumed_memory_thresholds": {
                "scale_out": 75,
                "scale_in": 20,
            },
            "storage_thresholds": {
                "scale_out": 80,
                "scale_in": 20,
            },
        }],
        "min_cluster_node_count": 3,
        "max_cluster_node_count": 8,
        "cool_down_period": "1800s",
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Gcp = Pulumi.Gcp;

return await Deployment.RunAsync(() => 
{
    var cluster_nw = new Gcp.VMwareEngine.Network("cluster-nw", new()
    {
        Name = "pc-nw",
        Type = "STANDARD",
        Location = "global",
        Description = "PC network description.",
    });

    var cluster_pc = new Gcp.VMwareEngine.PrivateCloud("cluster-pc", new()
    {
        Location = "us-west1-a",
        Name = "sample-pc",
        Description = "Sample test PC.",
        NetworkConfig = new Gcp.VMwareEngine.Inputs.PrivateCloudNetworkConfigArgs
        {
            ManagementCidr = "192.168.30.0/24",
            VmwareEngineNetwork = cluster_nw.Id,
        },
        ManagementCluster = new Gcp.VMwareEngine.Inputs.PrivateCloudManagementClusterArgs
        {
            ClusterId = "sample-mgmt-cluster",
            NodeTypeConfigs = new[]
            {
                new Gcp.VMwareEngine.Inputs.PrivateCloudManagementClusterNodeTypeConfigArgs
                {
                    NodeTypeId = "standard-72",
                    NodeCount = 3,
                    CustomCoreCount = 32,
                },
            },
        },
    });

    var vmw_ext_cluster = new Gcp.VMwareEngine.Cluster("vmw-ext-cluster", new()
    {
        Name = "ext-cluster",
        Parent = cluster_pc.Id,
        NodeTypeConfigs = new[]
        {
            new Gcp.VMwareEngine.Inputs.ClusterNodeTypeConfigArgs
            {
                NodeTypeId = "standard-72",
                NodeCount = 3,
                CustomCoreCount = 32,
            },
        },
        AutoscalingSettings = new Gcp.VMwareEngine.Inputs.ClusterAutoscalingSettingsArgs
        {
            AutoscalingPolicies = new[]
            {
                new Gcp.VMwareEngine.Inputs.ClusterAutoscalingSettingsAutoscalingPolicyArgs
                {
                    AutoscalePolicyId = "autoscaling-policy",
                    NodeTypeId = "standard-72",
                    ScaleOutSize = 1,
                    CpuThresholds = new Gcp.VMwareEngine.Inputs.ClusterAutoscalingSettingsAutoscalingPolicyCpuThresholdsArgs
                    {
                        ScaleOut = 80,
                        ScaleIn = 15,
                    },
                    ConsumedMemoryThresholds = new Gcp.VMwareEngine.Inputs.ClusterAutoscalingSettingsAutoscalingPolicyConsumedMemoryThresholdsArgs
                    {
                        ScaleOut = 75,
                        ScaleIn = 20,
                    },
                    StorageThresholds = new Gcp.VMwareEngine.Inputs.ClusterAutoscalingSettingsAutoscalingPolicyStorageThresholdsArgs
                    {
                        ScaleOut = 80,
                        ScaleIn = 20,
                    },
                },
            },
            MinClusterNodeCount = 3,
            MaxClusterNodeCount = 8,
            CoolDownPeriod = "1800s",
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-gcp/sdk/v8/go/gcp/vmwareengine"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := vmwareengine.NewNetwork(ctx, "cluster-nw", &vmwareengine.NetworkArgs{
			Name:        pulumi.String("pc-nw"),
			Type:        pulumi.String("STANDARD"),
			Location:    pulumi.String("global"),
			Description: pulumi.String("PC network description."),
		})
		if err != nil {
			return err
		}
		_, err = vmwareengine.NewPrivateCloud(ctx, "cluster-pc", &vmwareengine.PrivateCloudArgs{
			Location:    pulumi.String("us-west1-a"),
			Name:        pulumi.String("sample-pc"),
			Description: pulumi.String("Sample test PC."),
			NetworkConfig: &vmwareengine.PrivateCloudNetworkConfigArgs{
				ManagementCidr:      pulumi.String("192.168.30.0/24"),
				VmwareEngineNetwork: cluster_nw.ID(),
			},
			ManagementCluster: &vmwareengine.PrivateCloudManagementClusterArgs{
				ClusterId: pulumi.String("sample-mgmt-cluster"),
				NodeTypeConfigs: vmwareengine.PrivateCloudManagementClusterNodeTypeConfigArray{
					&vmwareengine.PrivateCloudManagementClusterNodeTypeConfigArgs{
						NodeTypeId:      pulumi.String("standard-72"),
						NodeCount:       pulumi.Int(3),
						CustomCoreCount: pulumi.Int(32),
					},
				},
			},
		})
		if err != nil {
			return err
		}
		_, err = vmwareengine.NewCluster(ctx, "vmw-ext-cluster", &vmwareengine.ClusterArgs{
			Name:   pulumi.String("ext-cluster"),
			Parent: cluster_pc.ID(),
			NodeTypeConfigs: vmwareengine.ClusterNodeTypeConfigArray{
				&vmwareengine.ClusterNodeTypeConfigArgs{
					NodeTypeId:      pulumi.String("standard-72"),
					NodeCount:       pulumi.Int(3),
					CustomCoreCount: pulumi.Int(32),
				},
			},
			AutoscalingSettings: &vmwareengine.ClusterAutoscalingSettingsArgs{
				AutoscalingPolicies: vmwareengine.ClusterAutoscalingSettingsAutoscalingPolicyArray{
					&vmwareengine.ClusterAutoscalingSettingsAutoscalingPolicyArgs{
						AutoscalePolicyId: pulumi.String("autoscaling-policy"),
						NodeTypeId:        pulumi.String("standard-72"),
						ScaleOutSize:      pulumi.Int(1),
						CpuThresholds: &vmwareengine.ClusterAutoscalingSettingsAutoscalingPolicyCpuThresholdsArgs{
							ScaleOut: pulumi.Int(80),
							ScaleIn:  pulumi.Int(15),
						},
						ConsumedMemoryThresholds: &vmwareengine.ClusterAutoscalingSettingsAutoscalingPolicyConsumedMemoryThresholdsArgs{
							ScaleOut: pulumi.Int(75),
							ScaleIn:  pulumi.Int(20),
						},
						StorageThresholds: &vmwareengine.ClusterAutoscalingSettingsAutoscalingPolicyStorageThresholdsArgs{
							ScaleOut: pulumi.Int(80),
							ScaleIn:  pulumi.Int(20),
						},
					},
				},
				MinClusterNodeCount: pulumi.Int(3),
				MaxClusterNodeCount: pulumi.Int(8),
				CoolDownPeriod:      pulumi.String("1800s"),
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.gcp.vmwareengine.Network;
import com.pulumi.gcp.vmwareengine.NetworkArgs;
import com.pulumi.gcp.vmwareengine.PrivateCloud;
import com.pulumi.gcp.vmwareengine.PrivateCloudArgs;
import com.pulumi.gcp.vmwareengine.inputs.PrivateCloudNetworkConfigArgs;
import com.pulumi.gcp.vmwareengine.inputs.PrivateCloudManagementClusterArgs;
import com.pulumi.gcp.vmwareengine.Cluster;
import com.pulumi.gcp.vmwareengine.ClusterArgs;
import com.pulumi.gcp.vmwareengine.inputs.ClusterNodeTypeConfigArgs;
import com.pulumi.gcp.vmwareengine.inputs.ClusterAutoscalingSettingsArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var cluster_nw = new Network("cluster-nw", NetworkArgs.builder()
            .name("pc-nw")
            .type("STANDARD")
            .location("global")
            .description("PC network description.")
            .build());

        var cluster_pc = new PrivateCloud("cluster-pc", PrivateCloudArgs.builder()
            .location("us-west1-a")
            .name("sample-pc")
            .description("Sample test PC.")
            .networkConfig(PrivateCloudNetworkConfigArgs.builder()
                .managementCidr("192.168.30.0/24")
                .vmwareEngineNetwork(cluster_nw.id())
                .build())
            .managementCluster(PrivateCloudManagementClusterArgs.builder()
                .clusterId("sample-mgmt-cluster")
                .nodeTypeConfigs(PrivateCloudManagementClusterNodeTypeConfigArgs.builder()
                    .nodeTypeId("standard-72")
                    .nodeCount(3)
                    .customCoreCount(32)
                    .build())
                .build())
            .build());

        var vmw_ext_cluster = new Cluster("vmw-ext-cluster", ClusterArgs.builder()
            .name("ext-cluster")
            .parent(cluster_pc.id())
            .nodeTypeConfigs(ClusterNodeTypeConfigArgs.builder()
                .nodeTypeId("standard-72")
                .nodeCount(3)
                .customCoreCount(32)
                .build())
            .autoscalingSettings(ClusterAutoscalingSettingsArgs.builder()
                .autoscalingPolicies(ClusterAutoscalingSettingsAutoscalingPolicyArgs.builder()
                    .autoscalePolicyId("autoscaling-policy")
                    .nodeTypeId("standard-72")
                    .scaleOutSize(1)
                    .cpuThresholds(ClusterAutoscalingSettingsAutoscalingPolicyCpuThresholdsArgs.builder()
                        .scaleOut(80)
                        .scaleIn(15)
                        .build())
                    .consumedMemoryThresholds(ClusterAutoscalingSettingsAutoscalingPolicyConsumedMemoryThresholdsArgs.builder()
                        .scaleOut(75)
                        .scaleIn(20)
                        .build())
                    .storageThresholds(ClusterAutoscalingSettingsAutoscalingPolicyStorageThresholdsArgs.builder()
                        .scaleOut(80)
                        .scaleIn(20)
                        .build())
                    .build())
                .minClusterNodeCount(3)
                .maxClusterNodeCount(8)
                .coolDownPeriod("1800s")
                .build())
            .build());

    }
}
```
```yaml
resources:
  vmw-ext-cluster:
    type: gcp:vmwareengine:Cluster
    properties:
      name: ext-cluster
      parent: ${["cluster-pc"].id}
      nodeTypeConfigs:
        - nodeTypeId: standard-72
          nodeCount: 3
          customCoreCount: 32
      autoscalingSettings:
        autoscalingPolicies:
          - autoscalePolicyId: autoscaling-policy
            nodeTypeId: standard-72
            scaleOutSize: 1
            cpuThresholds:
              scaleOut: 80
              scaleIn: 15
            consumedMemoryThresholds:
              scaleOut: 75
              scaleIn: 20
            storageThresholds:
              scaleOut: 80
              scaleIn: 20
        minClusterNodeCount: 3
        maxClusterNodeCount: 8
        coolDownPeriod: 1800s
  cluster-pc:
    type: gcp:vmwareengine:PrivateCloud
    properties:
      location: us-west1-a
      name: sample-pc
      description: Sample test PC.
      networkConfig:
        managementCidr: 192.168.30.0/24
        vmwareEngineNetwork: ${["cluster-nw"].id}
      managementCluster:
        clusterId: sample-mgmt-cluster
        nodeTypeConfigs:
          - nodeTypeId: standard-72
            nodeCount: 3
            customCoreCount: 32
  cluster-nw:
    type: gcp:vmwareengine:Network
    properties:
      name: pc-nw
      type: STANDARD
      location: global
      description: PC network description.
```
<!--End PulumiCodeChooser -->

## Import

Cluster can be imported using any of these accepted formats:

* `{{parent}}/clusters/{{name}}`

When using the `pulumi import` command, Cluster can be imported using one of the formats above. For example:

```sh
$ pulumi import gcp:vmwareengine/cluster:Cluster default {{parent}}/clusters/{{name}}
```

È
autoscalingSettingsxBv:t
r
vmwareengineClusterAutoscalingSettingsFgcp:vmwareengine/ClusterAutoscalingSettings:ClusterAutoscalingSettingsXConfiguration of the autoscaling applied to this cluster
Structure is documented below.
-
nameB" The ID of the Cluster.


- - -
•
nodeTypeConfigskBi*g:e
c
vmwareengineClusterNodeTypeConfig<gcp:vmwareengine/ClusterNodeTypeConfig:ClusterNodeTypeConfig§The map of cluster node types in this cluster,
where the key is canonical identifier of the node type (corresponds to the NodeType).
Structure is documented below.
ñ
parent" áThe resource name of the private cloud to create a new cluster in.
Resource names are schemeless URIs that follow the conventions in https://cloud.google.com/apis/design/resource_names.
For example: projects/my-project/locations/us-west1-a/privateClouds/my-cloud
"È
autoscalingSettingsxBv:t
r
vmwareengineClusterAutoscalingSettingsFgcp:vmwareengine/ClusterAutoscalingSettings:ClusterAutoscalingSettingsXConfiguration of the autoscaling applied to this cluster
Structure is documented below.
"≠

management
 öTrue if the cluster is a management cluster; false otherwise.
There can only be one management cluster in a private cloud and it has to be the first one.
"+
name" The ID of the Cluster.


- - -
"•
nodeTypeConfigskBi*g:e
c
vmwareengineClusterNodeTypeConfig<gcp:vmwareengine/ClusterNodeTypeConfig:ClusterNodeTypeConfig§The map of cluster node types in this cluster,
where the key is canonical identifier of the node type (corresponds to the NodeType).
Structure is documented below.
"ñ
parent" áThe resource name of the private cloud to create a new cluster in.
Resource names are schemeless URIs that follow the conventions in https://cloud.google.com/apis/design/resource_names.
For example: projects/my-project/locations/us-west1-a/privateClouds/my-cloud
"#
state" State of the Cluster.
"@
uid" 5System-generated unique identifier for the resource.
*ë÷
Z
vmwareengineExternalAccessRule6gcp:vmwareengine/externalAccessRule:ExternalAccessRule‘∏External access firewall rules for filtering incoming traffic destined to `ExternalAddress` resources.


To get more information about ExternalAccessRule, see:

* [API documentation](https://cloud.google.com/vmware-engine/docs/reference/rest/v1/projects.locations.networkPolicies.externalAccessRules)

## Example Usage

### Vmware Engine External Access Rule Basic


<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as gcp from "@pulumi/gcp";

const external_access_rule_nw = new gcp.vmwareengine.Network("external-access-rule-nw", {
    name: "sample-nw",
    location: "global",
    type: "STANDARD",
    description: "PC network description.",
});
const external_access_rule_np = new gcp.vmwareengine.NetworkPolicy("external-access-rule-np", {
    location: "us-west1",
    name: "sample-np",
    edgeServicesCidr: "192.168.30.0/26",
    vmwareEngineNetwork: external_access_rule_nw.id,
});
const vmw_engine_external_access_rule = new gcp.vmwareengine.ExternalAccessRule("vmw-engine-external-access-rule", {
    name: "sample-external-access-rule",
    parent: external_access_rule_np.id,
    priority: 101,
    action: "DENY",
    ipProtocol: "TCP",
    sourceIpRanges: [{
        ipAddressRange: "0.0.0.0/0",
    }],
    sourcePorts: ["80"],
    destinationIpRanges: [{
        ipAddressRange: "0.0.0.0/0",
    }],
    destinationPorts: ["433"],
});
```
```python
import pulumi
import pulumi_gcp as gcp

external_access_rule_nw = gcp.vmwareengine.Network("external-access-rule-nw",
    name="sample-nw",
    location="global",
    type="STANDARD",
    description="PC network description.")
external_access_rule_np = gcp.vmwareengine.NetworkPolicy("external-access-rule-np",
    location="us-west1",
    name="sample-np",
    edge_services_cidr="192.168.30.0/26",
    vmware_engine_network=external_access_rule_nw.id)
vmw_engine_external_access_rule = gcp.vmwareengine.ExternalAccessRule("vmw-engine-external-access-rule",
    name="sample-external-access-rule",
    parent=external_access_rule_np.id,
    priority=101,
    action="DENY",
    ip_protocol="TCP",
    source_ip_ranges=[{
        "ip_address_range": "0.0.0.0/0",
    }],
    source_ports=["80"],
    destination_ip_ranges=[{
        "ip_address_range": "0.0.0.0/0",
    }],
    destination_ports=["433"])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Gcp = Pulumi.Gcp;

return await Deployment.RunAsync(() => 
{
    var external_access_rule_nw = new Gcp.VMwareEngine.Network("external-access-rule-nw", new()
    {
        Name = "sample-nw",
        Location = "global",
        Type = "STANDARD",
        Description = "PC network description.",
    });

    var external_access_rule_np = new Gcp.VMwareEngine.NetworkPolicy("external-access-rule-np", new()
    {
        Location = "us-west1",
        Name = "sample-np",
        EdgeServicesCidr = "192.168.30.0/26",
        VmwareEngineNetwork = external_access_rule_nw.Id,
    });

    var vmw_engine_external_access_rule = new Gcp.VMwareEngine.ExternalAccessRule("vmw-engine-external-access-rule", new()
    {
        Name = "sample-external-access-rule",
        Parent = external_access_rule_np.Id,
        Priority = 101,
        Action = "DENY",
        IpProtocol = "TCP",
        SourceIpRanges = new[]
        {
            new Gcp.VMwareEngine.Inputs.ExternalAccessRuleSourceIpRangeArgs
            {
                IpAddressRange = "0.0.0.0/0",
            },
        },
        SourcePorts = new[]
        {
            "80",
        },
        DestinationIpRanges = new[]
        {
            new Gcp.VMwareEngine.Inputs.ExternalAccessRuleDestinationIpRangeArgs
            {
                IpAddressRange = "0.0.0.0/0",
            },
        },
        DestinationPorts = new[]
        {
            "433",
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-gcp/sdk/v8/go/gcp/vmwareengine"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := vmwareengine.NewNetwork(ctx, "external-access-rule-nw", &vmwareengine.NetworkArgs{
			Name:        pulumi.String("sample-nw"),
			Location:    pulumi.String("global"),
			Type:        pulumi.String("STANDARD"),
			Description: pulumi.String("PC network description."),
		})
		if err != nil {
			return err
		}
		_, err = vmwareengine.NewNetworkPolicy(ctx, "external-access-rule-np", &vmwareengine.NetworkPolicyArgs{
			Location:            pulumi.String("us-west1"),
			Name:                pulumi.String("sample-np"),
			EdgeServicesCidr:    pulumi.String("192.168.30.0/26"),
			VmwareEngineNetwork: external_access_rule_nw.ID(),
		})
		if err != nil {
			return err
		}
		_, err = vmwareengine.NewExternalAccessRule(ctx, "vmw-engine-external-access-rule", &vmwareengine.ExternalAccessRuleArgs{
			Name:       pulumi.String("sample-external-access-rule"),
			Parent:     external_access_rule_np.ID(),
			Priority:   pulumi.Int(101),
			Action:     pulumi.String("DENY"),
			IpProtocol: pulumi.String("TCP"),
			SourceIpRanges: vmwareengine.ExternalAccessRuleSourceIpRangeArray{
				&vmwareengine.ExternalAccessRuleSourceIpRangeArgs{
					IpAddressRange: pulumi.String("0.0.0.0/0"),
				},
			},
			SourcePorts: pulumi.StringArray{
				pulumi.String("80"),
			},
			DestinationIpRanges: vmwareengine.ExternalAccessRuleDestinationIpRangeArray{
				&vmwareengine.ExternalAccessRuleDestinationIpRangeArgs{
					IpAddressRange: pulumi.String("0.0.0.0/0"),
				},
			},
			DestinationPorts: pulumi.StringArray{
				pulumi.String("433"),
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.gcp.vmwareengine.Network;
import com.pulumi.gcp.vmwareengine.NetworkArgs;
import com.pulumi.gcp.vmwareengine.NetworkPolicy;
import com.pulumi.gcp.vmwareengine.NetworkPolicyArgs;
import com.pulumi.gcp.vmwareengine.ExternalAccessRule;
import com.pulumi.gcp.vmwareengine.ExternalAccessRuleArgs;
import com.pulumi.gcp.vmwareengine.inputs.ExternalAccessRuleSourceIpRangeArgs;
import com.pulumi.gcp.vmwareengine.inputs.ExternalAccessRuleDestinationIpRangeArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var external_access_rule_nw = new Network("external-access-rule-nw", NetworkArgs.builder()
            .name("sample-nw")
            .location("global")
            .type("STANDARD")
            .description("PC network description.")
            .build());

        var external_access_rule_np = new NetworkPolicy("external-access-rule-np", NetworkPolicyArgs.builder()
            .location("us-west1")
            .name("sample-np")
            .edgeServicesCidr("192.168.30.0/26")
            .vmwareEngineNetwork(external_access_rule_nw.id())
            .build());

        var vmw_engine_external_access_rule = new ExternalAccessRule("vmw-engine-external-access-rule", ExternalAccessRuleArgs.builder()
            .name("sample-external-access-rule")
            .parent(external_access_rule_np.id())
            .priority(101)
            .action("DENY")
            .ipProtocol("TCP")
            .sourceIpRanges(ExternalAccessRuleSourceIpRangeArgs.builder()
                .ipAddressRange("0.0.0.0/0")
                .build())
            .sourcePorts("80")
            .destinationIpRanges(ExternalAccessRuleDestinationIpRangeArgs.builder()
                .ipAddressRange("0.0.0.0/0")
                .build())
            .destinationPorts("433")
            .build());

    }
}
```
```yaml
resources:
  external-access-rule-nw:
    type: gcp:vmwareengine:Network
    properties:
      name: sample-nw
      location: global
      type: STANDARD
      description: PC network description.
  external-access-rule-np:
    type: gcp:vmwareengine:NetworkPolicy
    properties:
      location: us-west1
      name: sample-np
      edgeServicesCidr: 192.168.30.0/26
      vmwareEngineNetwork: ${["external-access-rule-nw"].id}
  vmw-engine-external-access-rule:
    type: gcp:vmwareengine:ExternalAccessRule
    properties:
      name: sample-external-access-rule
      parent: ${["external-access-rule-np"].id}
      priority: 101
      action: DENY
      ipProtocol: TCP
      sourceIpRanges:
        - ipAddressRange: 0.0.0.0/0
      sourcePorts:
        - '80'
      destinationIpRanges:
        - ipAddressRange: 0.0.0.0/0
      destinationPorts:
        - '433'
```
<!--End PulumiCodeChooser -->
### Vmware Engine External Access Rule Full


<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as gcp from "@pulumi/gcp";

const external_access_rule_nw = new gcp.vmwareengine.Network("external-access-rule-nw", {
    name: "sample-nw",
    location: "global",
    type: "STANDARD",
    description: "PC network description.",
});
const external_access_rule_pc = new gcp.vmwareengine.PrivateCloud("external-access-rule-pc", {
    location: "us-west1-a",
    name: "sample-pc",
    description: "Sample test PC.",
    networkConfig: {
        managementCidr: "192.168.50.0/24",
        vmwareEngineNetwork: external_access_rule_nw.id,
    },
    managementCluster: {
        clusterId: "sample-mgmt-cluster",
        nodeTypeConfigs: [{
            nodeTypeId: "standard-72",
            nodeCount: 3,
        }],
    },
});
const external_access_rule_np = new gcp.vmwareengine.NetworkPolicy("external-access-rule-np", {
    location: "us-west1",
    name: "sample-np",
    edgeServicesCidr: "192.168.30.0/26",
    vmwareEngineNetwork: external_access_rule_nw.id,
});
const external_access_rule_ea = new gcp.vmwareengine.ExternalAddress("external-access-rule-ea", {
    name: "sample-ea",
    parent: external_access_rule_pc.id,
    internalIp: "192.168.0.65",
});
const vmw_engine_external_access_rule = new gcp.vmwareengine.ExternalAccessRule("vmw-engine-external-access-rule", {
    name: "sample-external-access-rule",
    parent: external_access_rule_np.id,
    description: "Sample Description",
    priority: 101,
    action: "ALLOW",
    ipProtocol: "tcp",
    sourceIpRanges: [{
        ipAddressRange: "0.0.0.0/0",
    }],
    sourcePorts: ["80"],
    destinationIpRanges: [{
        externalAddress: external_access_rule_ea.id,
    }],
    destinationPorts: ["433"],
});
```
```python
import pulumi
import pulumi_gcp as gcp

external_access_rule_nw = gcp.vmwareengine.Network("external-access-rule-nw",
    name="sample-nw",
    location="global",
    type="STANDARD",
    description="PC network description.")
external_access_rule_pc = gcp.vmwareengine.PrivateCloud("external-access-rule-pc",
    location="us-west1-a",
    name="sample-pc",
    description="Sample test PC.",
    network_config={
        "management_cidr": "192.168.50.0/24",
        "vmware_engine_network": external_access_rule_nw.id,
    },
    management_cluster={
        "cluster_id": "sample-mgmt-cluster",
        "node_type_configs": [{
            "node_type_id": "standard-72",
            "node_count": 3,
        }],
    })
external_access_rule_np = gcp.vmwareengine.NetworkPolicy("external-access-rule-np",
    location="us-west1",
    name="sample-np",
    edge_services_cidr="192.168.30.0/26",
    vmware_engine_network=external_access_rule_nw.id)
external_access_rule_ea = gcp.vmwareengine.ExternalAddress("external-access-rule-ea",
    name="sample-ea",
    parent=external_access_rule_pc.id,
    internal_ip="192.168.0.65")
vmw_engine_external_access_rule = gcp.vmwareengine.ExternalAccessRule("vmw-engine-external-access-rule",
    name="sample-external-access-rule",
    parent=external_access_rule_np.id,
    description="Sample Description",
    priority=101,
    action="ALLOW",
    ip_protocol="tcp",
    source_ip_ranges=[{
        "ip_address_range": "0.0.0.0/0",
    }],
    source_ports=["80"],
    destination_ip_ranges=[{
        "external_address": external_access_rule_ea.id,
    }],
    destination_ports=["433"])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Gcp = Pulumi.Gcp;

return await Deployment.RunAsync(() => 
{
    var external_access_rule_nw = new Gcp.VMwareEngine.Network("external-access-rule-nw", new()
    {
        Name = "sample-nw",
        Location = "global",
        Type = "STANDARD",
        Description = "PC network description.",
    });

    var external_access_rule_pc = new Gcp.VMwareEngine.PrivateCloud("external-access-rule-pc", new()
    {
        Location = "us-west1-a",
        Name = "sample-pc",
        Description = "Sample test PC.",
        NetworkConfig = new Gcp.VMwareEngine.Inputs.PrivateCloudNetworkConfigArgs
        {
            ManagementCidr = "192.168.50.0/24",
            VmwareEngineNetwork = external_access_rule_nw.Id,
        },
        ManagementCluster = new Gcp.VMwareEngine.Inputs.PrivateCloudManagementClusterArgs
        {
            ClusterId = "sample-mgmt-cluster",
            NodeTypeConfigs = new[]
            {
                new Gcp.VMwareEngine.Inputs.PrivateCloudManagementClusterNodeTypeConfigArgs
                {
                    NodeTypeId = "standard-72",
                    NodeCount = 3,
                },
            },
        },
    });

    var external_access_rule_np = new Gcp.VMwareEngine.NetworkPolicy("external-access-rule-np", new()
    {
        Location = "us-west1",
        Name = "sample-np",
        EdgeServicesCidr = "192.168.30.0/26",
        VmwareEngineNetwork = external_access_rule_nw.Id,
    });

    var external_access_rule_ea = new Gcp.VMwareEngine.ExternalAddress("external-access-rule-ea", new()
    {
        Name = "sample-ea",
        Parent = external_access_rule_pc.Id,
        InternalIp = "192.168.0.65",
    });

    var vmw_engine_external_access_rule = new Gcp.VMwareEngine.ExternalAccessRule("vmw-engine-external-access-rule", new()
    {
        Name = "sample-external-access-rule",
        Parent = external_access_rule_np.Id,
        Description = "Sample Description",
        Priority = 101,
        Action = "ALLOW",
        IpProtocol = "tcp",
        SourceIpRanges = new[]
        {
            new Gcp.VMwareEngine.Inputs.ExternalAccessRuleSourceIpRangeArgs
            {
                IpAddressRange = "0.0.0.0/0",
            },
        },
        SourcePorts = new[]
        {
            "80",
        },
        DestinationIpRanges = new[]
        {
            new Gcp.VMwareEngine.Inputs.ExternalAccessRuleDestinationIpRangeArgs
            {
                ExternalAddress = external_access_rule_ea.Id,
            },
        },
        DestinationPorts = new[]
        {
            "433",
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-gcp/sdk/v8/go/gcp/vmwareengine"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := vmwareengine.NewNetwork(ctx, "external-access-rule-nw", &vmwareengine.NetworkArgs{
			Name:        pulumi.String("sample-nw"),
			Location:    pulumi.String("global"),
			Type:        pulumi.String("STANDARD"),
			Description: pulumi.String("PC network description."),
		})
		if err != nil {
			return err
		}
		_, err = vmwareengine.NewPrivateCloud(ctx, "external-access-rule-pc", &vmwareengine.PrivateCloudArgs{
			Location:    pulumi.String("us-west1-a"),
			Name:        pulumi.String("sample-pc"),
			Description: pulumi.String("Sample test PC."),
			NetworkConfig: &vmwareengine.PrivateCloudNetworkConfigArgs{
				ManagementCidr:      pulumi.String("192.168.50.0/24"),
				VmwareEngineNetwork: external_access_rule_nw.ID(),
			},
			ManagementCluster: &vmwareengine.PrivateCloudManagementClusterArgs{
				ClusterId: pulumi.String("sample-mgmt-cluster"),
				NodeTypeConfigs: vmwareengine.PrivateCloudManagementClusterNodeTypeConfigArray{
					&vmwareengine.PrivateCloudManagementClusterNodeTypeConfigArgs{
						NodeTypeId: pulumi.String("standard-72"),
						NodeCount:  pulumi.Int(3),
					},
				},
			},
		})
		if err != nil {
			return err
		}
		_, err = vmwareengine.NewNetworkPolicy(ctx, "external-access-rule-np", &vmwareengine.NetworkPolicyArgs{
			Location:            pulumi.String("us-west1"),
			Name:                pulumi.String("sample-np"),
			EdgeServicesCidr:    pulumi.String("192.168.30.0/26"),
			VmwareEngineNetwork: external_access_rule_nw.ID(),
		})
		if err != nil {
			return err
		}
		_, err = vmwareengine.NewExternalAddress(ctx, "external-access-rule-ea", &vmwareengine.ExternalAddressArgs{
			Name:       pulumi.String("sample-ea"),
			Parent:     external_access_rule_pc.ID(),
			InternalIp: pulumi.String("192.168.0.65"),
		})
		if err != nil {
			return err
		}
		_, err = vmwareengine.NewExternalAccessRule(ctx, "vmw-engine-external-access-rule", &vmwareengine.ExternalAccessRuleArgs{
			Name:        pulumi.String("sample-external-access-rule"),
			Parent:      external_access_rule_np.ID(),
			Description: pulumi.String("Sample Description"),
			Priority:    pulumi.Int(101),
			Action:      pulumi.String("ALLOW"),
			IpProtocol:  pulumi.String("tcp"),
			SourceIpRanges: vmwareengine.ExternalAccessRuleSourceIpRangeArray{
				&vmwareengine.ExternalAccessRuleSourceIpRangeArgs{
					IpAddressRange: pulumi.String("0.0.0.0/0"),
				},
			},
			SourcePorts: pulumi.StringArray{
				pulumi.String("80"),
			},
			DestinationIpRanges: vmwareengine.ExternalAccessRuleDestinationIpRangeArray{
				&vmwareengine.ExternalAccessRuleDestinationIpRangeArgs{
					ExternalAddress: external_access_rule_ea.ID(),
				},
			},
			DestinationPorts: pulumi.StringArray{
				pulumi.String("433"),
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.gcp.vmwareengine.Network;
import com.pulumi.gcp.vmwareengine.NetworkArgs;
import com.pulumi.gcp.vmwareengine.PrivateCloud;
import com.pulumi.gcp.vmwareengine.PrivateCloudArgs;
import com.pulumi.gcp.vmwareengine.inputs.PrivateCloudNetworkConfigArgs;
import com.pulumi.gcp.vmwareengine.inputs.PrivateCloudManagementClusterArgs;
import com.pulumi.gcp.vmwareengine.NetworkPolicy;
import com.pulumi.gcp.vmwareengine.NetworkPolicyArgs;
import com.pulumi.gcp.vmwareengine.ExternalAddress;
import com.pulumi.gcp.vmwareengine.ExternalAddressArgs;
import com.pulumi.gcp.vmwareengine.ExternalAccessRule;
import com.pulumi.gcp.vmwareengine.ExternalAccessRuleArgs;
import com.pulumi.gcp.vmwareengine.inputs.ExternalAccessRuleSourceIpRangeArgs;
import com.pulumi.gcp.vmwareengine.inputs.ExternalAccessRuleDestinationIpRangeArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var external_access_rule_nw = new Network("external-access-rule-nw", NetworkArgs.builder()
            .name("sample-nw")
            .location("global")
            .type("STANDARD")
            .description("PC network description.")
            .build());

        var external_access_rule_pc = new PrivateCloud("external-access-rule-pc", PrivateCloudArgs.builder()
            .location("us-west1-a")
            .name("sample-pc")
            .description("Sample test PC.")
            .networkConfig(PrivateCloudNetworkConfigArgs.builder()
                .managementCidr("192.168.50.0/24")
                .vmwareEngineNetwork(external_access_rule_nw.id())
                .build())
            .managementCluster(PrivateCloudManagementClusterArgs.builder()
                .clusterId("sample-mgmt-cluster")
                .nodeTypeConfigs(PrivateCloudManagementClusterNodeTypeConfigArgs.builder()
                    .nodeTypeId("standard-72")
                    .nodeCount(3)
                    .build())
                .build())
            .build());

        var external_access_rule_np = new NetworkPolicy("external-access-rule-np", NetworkPolicyArgs.builder()
            .location("us-west1")
            .name("sample-np")
            .edgeServicesCidr("192.168.30.0/26")
            .vmwareEngineNetwork(external_access_rule_nw.id())
            .build());

        var external_access_rule_ea = new ExternalAddress("external-access-rule-ea", ExternalAddressArgs.builder()
            .name("sample-ea")
            .parent(external_access_rule_pc.id())
            .internalIp("192.168.0.65")
            .build());

        var vmw_engine_external_access_rule = new ExternalAccessRule("vmw-engine-external-access-rule", ExternalAccessRuleArgs.builder()
            .name("sample-external-access-rule")
            .parent(external_access_rule_np.id())
            .description("Sample Description")
            .priority(101)
            .action("ALLOW")
            .ipProtocol("tcp")
            .sourceIpRanges(ExternalAccessRuleSourceIpRangeArgs.builder()
                .ipAddressRange("0.0.0.0/0")
                .build())
            .sourcePorts("80")
            .destinationIpRanges(ExternalAccessRuleDestinationIpRangeArgs.builder()
                .externalAddress(external_access_rule_ea.id())
                .build())
            .destinationPorts("433")
            .build());

    }
}
```
```yaml
resources:
  external-access-rule-nw:
    type: gcp:vmwareengine:Network
    properties:
      name: sample-nw
      location: global
      type: STANDARD
      description: PC network description.
  external-access-rule-pc:
    type: gcp:vmwareengine:PrivateCloud
    properties:
      location: us-west1-a
      name: sample-pc
      description: Sample test PC.
      networkConfig:
        managementCidr: 192.168.50.0/24
        vmwareEngineNetwork: ${["external-access-rule-nw"].id}
      managementCluster:
        clusterId: sample-mgmt-cluster
        nodeTypeConfigs:
          - nodeTypeId: standard-72
            nodeCount: 3
  external-access-rule-np:
    type: gcp:vmwareengine:NetworkPolicy
    properties:
      location: us-west1
      name: sample-np
      edgeServicesCidr: 192.168.30.0/26
      vmwareEngineNetwork: ${["external-access-rule-nw"].id}
  external-access-rule-ea:
    type: gcp:vmwareengine:ExternalAddress
    properties:
      name: sample-ea
      parent: ${["external-access-rule-pc"].id}
      internalIp: 192.168.0.65
  vmw-engine-external-access-rule:
    type: gcp:vmwareengine:ExternalAccessRule
    properties:
      name: sample-external-access-rule
      parent: ${["external-access-rule-np"].id}
      description: Sample Description
      priority: 101
      action: ALLOW
      ipProtocol: tcp
      sourceIpRanges:
        - ipAddressRange: 0.0.0.0/0
      sourcePorts:
        - '80'
      destinationIpRanges:
        - externalAddress: ${["external-access-rule-ea"].id}
      destinationPorts:
        - '433'
```
<!--End PulumiCodeChooser -->

## Import

ExternalAccessRule can be imported using any of these accepted formats:

* `{{parent}}/externalAccessRules/{{name}}`

When using the `pulumi import` command, ExternalAccessRule can be imported using one of the formats above. For example:

```sh
$ pulumi import gcp:vmwareengine/externalAccessRule:ExternalAccessRule default {{parent}}/externalAccessRules/{{name}}
```

g
action" YThe action that the external access rule performs.
Possible values are: `ALLOW`, `DENY`.
M
descriptionB" 8User-provided description for the external access rule.
‹
destinationIpRangesô*ñ:ì
ê
vmwareengine$ExternalAccessRuleDestinationIpRangeZgcp:vmwareengine/ExternalAccessRuleDestinationIpRange:ExternalAccessRuleDestinationIpRange®If destination ranges are specified, the external access rule applies only to
traffic that has a destination IP address in these ranges.
Structure is documented below.
a
destinationPorts*" GA list of destination ports to which the external access rule applies.
M

ipProtocol" ;The IP protocol to which the external access rule applies.
2
nameB" $The ID of the external access rule.
ˇ
parent" The resource name of the network policy.
Resource names are schemeless URIs that follow the conventions in https://cloud.google.com/apis/design/resource_names.
For example: projects/my-project/locations/us-west1-a/networkPolicies/my-policy
{
priority kExternal access rule priority, which determines the external access rule to use when multiple rules apply.
æ
sourceIpRangesä*á:Ñ
Å
vmwareengineExternalAccessRuleSourceIpRangePgcp:vmwareengine/ExternalAccessRuleSourceIpRange:ExternalAccessRuleSourceIpRangeûIf source ranges are specified, the external access rule applies only to
traffic that has a source IP address in these ranges.
Structure is documented below.
W
sourcePorts*" BA list of source ports to which the external access rule applies.
"g
action" YThe action that the external access rule performs.
Possible values are: `ALLOW`, `DENY`.
"·

createTime" ŒCreation time of this resource.
A timestamp in RFC3339 UTC "Zulu" format, with nanosecond resolution and
up to nine fractional digits. Examples: "2014-10-02T15:01:23Z" and "2014-10-02T15:01:23.045123456Z".
"M
descriptionB" 8User-provided description for the external access rule.
"‹
destinationIpRangesô*ñ:ì
ê
vmwareengine$ExternalAccessRuleDestinationIpRangeZgcp:vmwareengine/ExternalAccessRuleDestinationIpRange:ExternalAccessRuleDestinationIpRange®If destination ranges are specified, the external access rule applies only to
traffic that has a destination IP address in these ranges.
Structure is documented below.
"a
destinationPorts*" GA list of destination ports to which the external access rule applies.
"M

ipProtocol" ;The IP protocol to which the external access rule applies.
"0
name" $The ID of the external access rule.
"ˇ
parent" The resource name of the network policy.
Resource names are schemeless URIs that follow the conventions in https://cloud.google.com/apis/design/resource_names.
For example: projects/my-project/locations/us-west1-a/networkPolicies/my-policy
"{
priority kExternal access rule priority, which determines the external access rule to use when multiple rules apply.
"æ
sourceIpRangesä*á:Ñ
Å
vmwareengineExternalAccessRuleSourceIpRangePgcp:vmwareengine/ExternalAccessRuleSourceIpRange:ExternalAccessRuleSourceIpRangeûIf source ranges are specified, the external access rule applies only to
traffic that has a source IP address in these ranges.
Structure is documented below.
"W
sourcePorts*" BA list of source ports to which the external access rule applies.
"#
state" State of the Cluster.
"@
uid" 5System-generated unique identifier for the resource.
"Â

updateTime" “Last updated time of this resource.
A timestamp in RFC3339 UTC "Zulu" format, with nanosecond resolution and up to nine
fractional digits. Examples: "2014-10-02T15:01:23Z" and "2014-10-02T15:01:23.045123456Z".
*ˇd
Q
vmwareengineExternalAddress0gcp:vmwareengine/externalAddress:ExternalAddress˚WAn allocated external IP address and its corresponding internal IP address in a private cloud.


To get more information about ExternalAddress, see:

* [API documentation](https://cloud.google.com/vmware-engine/docs/reference/rest/v1/projects.locations.privateClouds.externalAddresses)

## Example Usage

### Vmware Engine External Address Basic


<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as gcp from "@pulumi/gcp";

const external_address_nw = new gcp.vmwareengine.Network("external-address-nw", {
    name: "pc-nw",
    location: "global",
    type: "STANDARD",
    description: "PC network description.",
});
const external_address_pc = new gcp.vmwareengine.PrivateCloud("external-address-pc", {
    location: "-a",
    name: "sample-pc",
    description: "Sample test PC.",
    networkConfig: {
        managementCidr: "192.168.50.0/24",
        vmwareEngineNetwork: external_address_nw.id,
    },
    managementCluster: {
        clusterId: "sample-mgmt-cluster",
        nodeTypeConfigs: [{
            nodeTypeId: "standard-72",
            nodeCount: 3,
        }],
    },
});
const external_address_np = new gcp.vmwareengine.NetworkPolicy("external-address-np", {
    location: "",
    name: "sample-np",
    edgeServicesCidr: "192.168.30.0/26",
    vmwareEngineNetwork: external_address_nw.id,
});
const vmw_engine_external_address = new gcp.vmwareengine.ExternalAddress("vmw-engine-external-address", {
    name: "sample-external-address",
    parent: external_address_pc.id,
    internalIp: "192.168.0.66",
    description: "Sample description.",
}, {
    dependsOn: [external_address_np],
});
```
```python
import pulumi
import pulumi_gcp as gcp

external_address_nw = gcp.vmwareengine.Network("external-address-nw",
    name="pc-nw",
    location="global",
    type="STANDARD",
    description="PC network description.")
external_address_pc = gcp.vmwareengine.PrivateCloud("external-address-pc",
    location="-a",
    name="sample-pc",
    description="Sample test PC.",
    network_config={
        "management_cidr": "192.168.50.0/24",
        "vmware_engine_network": external_address_nw.id,
    },
    management_cluster={
        "cluster_id": "sample-mgmt-cluster",
        "node_type_configs": [{
            "node_type_id": "standard-72",
            "node_count": 3,
        }],
    })
external_address_np = gcp.vmwareengine.NetworkPolicy("external-address-np",
    location="",
    name="sample-np",
    edge_services_cidr="192.168.30.0/26",
    vmware_engine_network=external_address_nw.id)
vmw_engine_external_address = gcp.vmwareengine.ExternalAddress("vmw-engine-external-address",
    name="sample-external-address",
    parent=external_address_pc.id,
    internal_ip="192.168.0.66",
    description="Sample description.",
    opts = pulumi.ResourceOptions(depends_on=[external_address_np]))
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Gcp = Pulumi.Gcp;

return await Deployment.RunAsync(() => 
{
    var external_address_nw = new Gcp.VMwareEngine.Network("external-address-nw", new()
    {
        Name = "pc-nw",
        Location = "global",
        Type = "STANDARD",
        Description = "PC network description.",
    });

    var external_address_pc = new Gcp.VMwareEngine.PrivateCloud("external-address-pc", new()
    {
        Location = "-a",
        Name = "sample-pc",
        Description = "Sample test PC.",
        NetworkConfig = new Gcp.VMwareEngine.Inputs.PrivateCloudNetworkConfigArgs
        {
            ManagementCidr = "192.168.50.0/24",
            VmwareEngineNetwork = external_address_nw.Id,
        },
        ManagementCluster = new Gcp.VMwareEngine.Inputs.PrivateCloudManagementClusterArgs
        {
            ClusterId = "sample-mgmt-cluster",
            NodeTypeConfigs = new[]
            {
                new Gcp.VMwareEngine.Inputs.PrivateCloudManagementClusterNodeTypeConfigArgs
                {
                    NodeTypeId = "standard-72",
                    NodeCount = 3,
                },
            },
        },
    });

    var external_address_np = new Gcp.VMwareEngine.NetworkPolicy("external-address-np", new()
    {
        Location = "",
        Name = "sample-np",
        EdgeServicesCidr = "192.168.30.0/26",
        VmwareEngineNetwork = external_address_nw.Id,
    });

    var vmw_engine_external_address = new Gcp.VMwareEngine.ExternalAddress("vmw-engine-external-address", new()
    {
        Name = "sample-external-address",
        Parent = external_address_pc.Id,
        InternalIp = "192.168.0.66",
        Description = "Sample description.",
    }, new CustomResourceOptions
    {
        DependsOn =
        {
            external_address_np,
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-gcp/sdk/v8/go/gcp/vmwareengine"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := vmwareengine.NewNetwork(ctx, "external-address-nw", &vmwareengine.NetworkArgs{
			Name:        pulumi.String("pc-nw"),
			Location:    pulumi.String("global"),
			Type:        pulumi.String("STANDARD"),
			Description: pulumi.String("PC network description."),
		})
		if err != nil {
			return err
		}
		_, err = vmwareengine.NewPrivateCloud(ctx, "external-address-pc", &vmwareengine.PrivateCloudArgs{
			Location:    pulumi.String("-a"),
			Name:        pulumi.String("sample-pc"),
			Description: pulumi.String("Sample test PC."),
			NetworkConfig: &vmwareengine.PrivateCloudNetworkConfigArgs{
				ManagementCidr:      pulumi.String("192.168.50.0/24"),
				VmwareEngineNetwork: external_address_nw.ID(),
			},
			ManagementCluster: &vmwareengine.PrivateCloudManagementClusterArgs{
				ClusterId: pulumi.String("sample-mgmt-cluster"),
				NodeTypeConfigs: vmwareengine.PrivateCloudManagementClusterNodeTypeConfigArray{
					&vmwareengine.PrivateCloudManagementClusterNodeTypeConfigArgs{
						NodeTypeId: pulumi.String("standard-72"),
						NodeCount:  pulumi.Int(3),
					},
				},
			},
		})
		if err != nil {
			return err
		}
		_, err = vmwareengine.NewNetworkPolicy(ctx, "external-address-np", &vmwareengine.NetworkPolicyArgs{
			Location:            pulumi.String(""),
			Name:                pulumi.String("sample-np"),
			EdgeServicesCidr:    pulumi.String("192.168.30.0/26"),
			VmwareEngineNetwork: external_address_nw.ID(),
		})
		if err != nil {
			return err
		}
		_, err = vmwareengine.NewExternalAddress(ctx, "vmw-engine-external-address", &vmwareengine.ExternalAddressArgs{
			Name:        pulumi.String("sample-external-address"),
			Parent:      external_address_pc.ID(),
			InternalIp:  pulumi.String("192.168.0.66"),
			Description: pulumi.String("Sample description."),
		}, pulumi.DependsOn([]pulumi.Resource{
			external_address_np,
		}))
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.gcp.vmwareengine.Network;
import com.pulumi.gcp.vmwareengine.NetworkArgs;
import com.pulumi.gcp.vmwareengine.PrivateCloud;
import com.pulumi.gcp.vmwareengine.PrivateCloudArgs;
import com.pulumi.gcp.vmwareengine.inputs.PrivateCloudNetworkConfigArgs;
import com.pulumi.gcp.vmwareengine.inputs.PrivateCloudManagementClusterArgs;
import com.pulumi.gcp.vmwareengine.NetworkPolicy;
import com.pulumi.gcp.vmwareengine.NetworkPolicyArgs;
import com.pulumi.gcp.vmwareengine.ExternalAddress;
import com.pulumi.gcp.vmwareengine.ExternalAddressArgs;
import com.pulumi.resources.CustomResourceOptions;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var external_address_nw = new Network("external-address-nw", NetworkArgs.builder()
            .name("pc-nw")
            .location("global")
            .type("STANDARD")
            .description("PC network description.")
            .build());

        var external_address_pc = new PrivateCloud("external-address-pc", PrivateCloudArgs.builder()
            .location("-a")
            .name("sample-pc")
            .description("Sample test PC.")
            .networkConfig(PrivateCloudNetworkConfigArgs.builder()
                .managementCidr("192.168.50.0/24")
                .vmwareEngineNetwork(external_address_nw.id())
                .build())
            .managementCluster(PrivateCloudManagementClusterArgs.builder()
                .clusterId("sample-mgmt-cluster")
                .nodeTypeConfigs(PrivateCloudManagementClusterNodeTypeConfigArgs.builder()
                    .nodeTypeId("standard-72")
                    .nodeCount(3)
                    .build())
                .build())
            .build());

        var external_address_np = new NetworkPolicy("external-address-np", NetworkPolicyArgs.builder()
            .location("")
            .name("sample-np")
            .edgeServicesCidr("192.168.30.0/26")
            .vmwareEngineNetwork(external_address_nw.id())
            .build());

        var vmw_engine_external_address = new ExternalAddress("vmw-engine-external-address", ExternalAddressArgs.builder()
            .name("sample-external-address")
            .parent(external_address_pc.id())
            .internalIp("192.168.0.66")
            .description("Sample description.")
            .build(), CustomResourceOptions.builder()
                .dependsOn(external_address_np)
                .build());

    }
}
```
```yaml
resources:
  external-address-nw:
    type: gcp:vmwareengine:Network
    properties:
      name: pc-nw
      location: global
      type: STANDARD
      description: PC network description.
  external-address-pc:
    type: gcp:vmwareengine:PrivateCloud
    properties:
      location: -a
      name: sample-pc
      description: Sample test PC.
      networkConfig:
        managementCidr: 192.168.50.0/24
        vmwareEngineNetwork: ${["external-address-nw"].id}
      managementCluster:
        clusterId: sample-mgmt-cluster
        nodeTypeConfigs:
          - nodeTypeId: standard-72
            nodeCount: 3
  external-address-np:
    type: gcp:vmwareengine:NetworkPolicy
    properties:
      location: ""
      name: sample-np
      edgeServicesCidr: 192.168.30.0/26
      vmwareEngineNetwork: ${["external-address-nw"].id}
  vmw-engine-external-address:
    type: gcp:vmwareengine:ExternalAddress
    properties:
      name: sample-external-address
      parent: ${["external-address-pc"].id}
      internalIp: 192.168.0.66
      description: Sample description.
    options:
      dependsOn:
        - ${["external-address-np"]}
```
<!--End PulumiCodeChooser -->

## Import

ExternalAddress can be imported using any of these accepted formats:

* `{{parent}}/externalAddresses/{{name}}`

When using the `pulumi import` command, ExternalAddress can be imported using one of the formats above. For example:

```sh
$ pulumi import gcp:vmwareengine/externalAddress:ExternalAddress default {{parent}}/externalAddresses/{{name}}
```

B
descriptionB" -User-provided description for this resource.
<

internalIp" *The internal IP address of a workload VM.
9
nameB" +The ID of the external IP Address.


- - -
ü
parent" êThe resource name of the private cloud to create a new external address in.
Resource names are schemeless URIs that follow the conventions in https://cloud.google.com/apis/design/resource_names.
For example: projects/my-project/locations/us-west1-a/privateClouds/my-cloud
"·

createTime" ŒCreation time of this resource.
A timestamp in RFC3339 UTC "Zulu" format, with nanosecond resolution and
up to nine fractional digits. Examples: "2014-10-02T15:01:23Z" and "2014-10-02T15:01:23.045123456Z".
"B
descriptionB" -User-provided description for this resource.
"<

externalIp" *The external IP address of a workload VM.
"<

internalIp" *The internal IP address of a workload VM.
"7
name" +The ID of the external IP Address.


- - -
"ü
parent" êThe resource name of the private cloud to create a new external address in.
Resource names are schemeless URIs that follow the conventions in https://cloud.google.com/apis/design/resource_names.
For example: projects/my-project/locations/us-west1-a/privateClouds/my-cloud
"$
state" State of the resource.
"@
uid" 5System-generated unique identifier for the resource.
"Â

updateTime" “Last updated time of this resource.
A timestamp in RFC3339 UTC "Zulu" format, with nanosecond resolution and up to nine
fractional digits. Examples: "2014-10-02T15:01:23Z" and "2014-10-02T15:01:23.045123456Z".
*Æg
9
vmwareengineNetwork gcp:vmwareengine/network:NetworkÉ]Provides connectivity for VMware Engine private clouds.


To get more information about Network, see:

* [API documentation](https://cloud.google.com/vmware-engine/docs/reference/rest/v1/projects.locations.vmwareEngineNetworks)

## Example Usage

### Vmware Engine Network Standard


<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as gcp from "@pulumi/gcp";

const vmw_engine_network = new gcp.vmwareengine.Network("vmw-engine-network", {
    name: "standard-nw",
    location: "global",
    type: "STANDARD",
    description: "VMwareEngine standard network sample",
});
```
```python
import pulumi
import pulumi_gcp as gcp

vmw_engine_network = gcp.vmwareengine.Network("vmw-engine-network",
    name="standard-nw",
    location="global",
    type="STANDARD",
    description="VMwareEngine standard network sample")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Gcp = Pulumi.Gcp;

return await Deployment.RunAsync(() => 
{
    var vmw_engine_network = new Gcp.VMwareEngine.Network("vmw-engine-network", new()
    {
        Name = "standard-nw",
        Location = "global",
        Type = "STANDARD",
        Description = "VMwareEngine standard network sample",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-gcp/sdk/v8/go/gcp/vmwareengine"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := vmwareengine.NewNetwork(ctx, "vmw-engine-network", &vmwareengine.NetworkArgs{
			Name:        pulumi.String("standard-nw"),
			Location:    pulumi.String("global"),
			Type:        pulumi.String("STANDARD"),
			Description: pulumi.String("VMwareEngine standard network sample"),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.gcp.vmwareengine.Network;
import com.pulumi.gcp.vmwareengine.NetworkArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var vmw_engine_network = new Network("vmw-engine-network", NetworkArgs.builder()
            .name("standard-nw")
            .location("global")
            .type("STANDARD")
            .description("VMwareEngine standard network sample")
            .build());

    }
}
```
```yaml
resources:
  vmw-engine-network:
    type: gcp:vmwareengine:Network
    properties:
      name: standard-nw
      location: global
      type: STANDARD
      description: VMwareEngine standard network sample
```
<!--End PulumiCodeChooser -->
### Vmware Engine Network Legacy


<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as gcp from "@pulumi/gcp";
import * as time from "@pulumi/time";

// there can be only 1 Legacy network per region for a given project,
// so creating new project for isolation in CI.
const acceptanceProject = new gcp.organizations.Project("acceptance", {
    name: "vmw-proj",
    projectId: "vmw-proj",
    orgId: "123456789",
    billingAccount: "000000-0000000-0000000-000000",
    deletionPolicy: "DELETE",
});
const wait60Seconds = new time.index.Sleep("wait_60_seconds", {createDuration: "60s"}, {
    dependsOn: [acceptanceProject],
});
const acceptance = new gcp.projects.Service("acceptance", {
    project: acceptanceProject.projectId,
    service: "vmwareengine.googleapis.com",
}, {
    dependsOn: [wait60Seconds],
});
const vmw_engine_network = new gcp.vmwareengine.Network("vmw-engine-network", {
    project: acceptance.project,
    name: "us-west1-default",
    location: "us-west1",
    type: "LEGACY",
    description: "VMwareEngine legacy network sample",
});
```
```python
import pulumi
import pulumi_gcp as gcp
import pulumi_time as time

# there can be only 1 Legacy network per region for a given project,
# so creating new project for isolation in CI.
acceptance_project = gcp.organizations.Project("acceptance",
    name="vmw-proj",
    project_id="vmw-proj",
    org_id="123456789",
    billing_account="000000-0000000-0000000-000000",
    deletion_policy="DELETE")
wait60_seconds = time.index.Sleep("wait_60_seconds", create_duration=60s,
opts = pulumi.ResourceOptions(depends_on=[acceptance_project]))
acceptance = gcp.projects.Service("acceptance",
    project=acceptance_project.project_id,
    service="vmwareengine.googleapis.com",
    opts = pulumi.ResourceOptions(depends_on=[wait60_seconds]))
vmw_engine_network = gcp.vmwareengine.Network("vmw-engine-network",
    project=acceptance.project,
    name="us-west1-default",
    location="us-west1",
    type="LEGACY",
    description="VMwareEngine legacy network sample")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Gcp = Pulumi.Gcp;
using Time = Pulumi.Time;

return await Deployment.RunAsync(() => 
{
    // there can be only 1 Legacy network per region for a given project,
    // so creating new project for isolation in CI.
    var acceptanceProject = new Gcp.Organizations.Project("acceptance", new()
    {
        Name = "vmw-proj",
        ProjectId = "vmw-proj",
        OrgId = "123456789",
        BillingAccount = "000000-0000000-0000000-000000",
        DeletionPolicy = "DELETE",
    });

    var wait60Seconds = new Time.Index.Sleep("wait_60_seconds", new()
    {
        CreateDuration = "60s",
    }, new CustomResourceOptions
    {
        DependsOn =
        {
            acceptanceProject,
        },
    });

    var acceptance = new Gcp.Projects.Service("acceptance", new()
    {
        Project = acceptanceProject.ProjectId,
        ServiceName = "vmwareengine.googleapis.com",
    }, new CustomResourceOptions
    {
        DependsOn =
        {
            wait60Seconds,
        },
    });

    var vmw_engine_network = new Gcp.VMwareEngine.Network("vmw-engine-network", new()
    {
        Project = acceptance.Project,
        Name = "us-west1-default",
        Location = "us-west1",
        Type = "LEGACY",
        Description = "VMwareEngine legacy network sample",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-gcp/sdk/v8/go/gcp/organizations"
	"github.com/pulumi/pulumi-gcp/sdk/v8/go/gcp/projects"
	"github.com/pulumi/pulumi-gcp/sdk/v8/go/gcp/vmwareengine"
	"github.com/pulumi/pulumi-time/sdk/go/time"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		// there can be only 1 Legacy network per region for a given project,
		// so creating new project for isolation in CI.
		acceptanceProject, err := organizations.NewProject(ctx, "acceptance", &organizations.ProjectArgs{
			Name:           pulumi.String("vmw-proj"),
			ProjectId:      pulumi.String("vmw-proj"),
			OrgId:          pulumi.String("123456789"),
			BillingAccount: pulumi.String("000000-0000000-0000000-000000"),
			DeletionPolicy: pulumi.String("DELETE"),
		})
		if err != nil {
			return err
		}
		wait60Seconds, err := time.NewSleep(ctx, "wait_60_seconds", &time.SleepArgs{
			CreateDuration: "60s",
		}, pulumi.DependsOn([]pulumi.Resource{
			acceptanceProject,
		}))
		if err != nil {
			return err
		}
		acceptance, err := projects.NewService(ctx, "acceptance", &projects.ServiceArgs{
			Project: acceptanceProject.ProjectId,
			Service: pulumi.String("vmwareengine.googleapis.com"),
		}, pulumi.DependsOn([]pulumi.Resource{
			wait60Seconds,
		}))
		if err != nil {
			return err
		}
		_, err = vmwareengine.NewNetwork(ctx, "vmw-engine-network", &vmwareengine.NetworkArgs{
			Project:     acceptance.Project,
			Name:        pulumi.String("us-west1-default"),
			Location:    pulumi.String("us-west1"),
			Type:        pulumi.String("LEGACY"),
			Description: pulumi.String("VMwareEngine legacy network sample"),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.gcp.organizations.Project;
import com.pulumi.gcp.organizations.ProjectArgs;
import com.pulumi.time.sleep;
import com.pulumi.time.SleepArgs;
import com.pulumi.gcp.projects.Service;
import com.pulumi.gcp.projects.ServiceArgs;
import com.pulumi.gcp.vmwareengine.Network;
import com.pulumi.gcp.vmwareengine.NetworkArgs;
import com.pulumi.resources.CustomResourceOptions;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        // there can be only 1 Legacy network per region for a given project,
        // so creating new project for isolation in CI.
        var acceptanceProject = new Project("acceptanceProject", ProjectArgs.builder()
            .name("vmw-proj")
            .projectId("vmw-proj")
            .orgId("123456789")
            .billingAccount("000000-0000000-0000000-000000")
            .deletionPolicy("DELETE")
            .build());

        var wait60Seconds = new Sleep("wait60Seconds", SleepArgs.builder()
            .createDuration("60s")
            .build(), CustomResourceOptions.builder()
                .dependsOn(acceptanceProject)
                .build());

        var acceptance = new Service("acceptance", ServiceArgs.builder()
            .project(acceptanceProject.projectId())
            .service("vmwareengine.googleapis.com")
            .build(), CustomResourceOptions.builder()
                .dependsOn(wait60Seconds)
                .build());

        var vmw_engine_network = new Network("vmw-engine-network", NetworkArgs.builder()
            .project(acceptance.project())
            .name("us-west1-default")
            .location("us-west1")
            .type("LEGACY")
            .description("VMwareEngine legacy network sample")
            .build());

    }
}
```
```yaml
resources:
  vmw-engine-network:
    type: gcp:vmwareengine:Network
    properties:
      project: ${acceptance.project}
      name: us-west1-default
      location: us-west1
      type: LEGACY
      description: VMwareEngine legacy network sample
  acceptance:
    type: gcp:projects:Service
    properties:
      project: ${acceptanceProject.projectId}
      service: vmwareengine.googleapis.com
    options:
      dependsOn:
        - ${wait60Seconds}
  # there can be only 1 Legacy network per region for a given project,
  # so creating new project for isolation in CI.
  acceptanceProject:
    type: gcp:organizations:Project
    name: acceptance
    properties:
      name: vmw-proj
      projectId: vmw-proj
      orgId: '123456789'
      billingAccount: 000000-0000000-0000000-000000
      deletionPolicy: DELETE
  wait60Seconds:
    type: time:sleep
    name: wait_60_seconds
    properties:
      createDuration: 60s
    options:
      dependsOn:
        - ${acceptanceProject}
```
<!--End PulumiCodeChooser -->

## Import

Network can be imported using any of these accepted formats:

* `projects/{{project}}/locations/{{location}}/vmwareEngineNetworks/{{name}}`

* `{{project}}/{{location}}/{{name}}`

* `{{location}}/{{name}}`

When using the `pulumi import` command, Network can be imported using one of the formats above. For example:

```sh
$ pulumi import gcp:vmwareengine/network:Network default projects/{{project}}/locations/{{location}}/vmwareEngineNetworks/{{name}}
```

```sh
$ pulumi import gcp:vmwareengine/network:Network default {{project}}/{{location}}/{{name}}
```

```sh
$ pulumi import gcp:vmwareengine/network:Network default {{location}}/{{name}}
```

O
descriptionB" :User-provided description for this VMware Engine network.
J
location" :The location where the VMwareEngineNetwork should reside.
9
nameB" +The ID of the VMwareEngineNetwork.


- - -
{
projectB" jThe ID of the project in which the resource belongs.
If it is not provided, the provider project is used.
S
type" GVMware Engine network type.
Possible values are: `LEGACY`, `STANDARD`.
"O
descriptionB" :User-provided description for this VMware Engine network.
"J
location" :The location where the VMwareEngineNetwork should reside.
"7
name" +The ID of the VMwareEngineNetwork.


- - -
"y
project" jThe ID of the project in which the resource belongs.
If it is not provided, the provider project is used.
"1
state" $State of the VMware Engine network.
"S
type" GVMware Engine network type.
Possible values are: `LEGACY`, `STANDARD`.
"@
uid" 5System-generated unique identifier for the resource.
"•
vpcNetworks]*[:Y
W
vmwareengineNetworkVpcNetwork4gcp:vmwareengine/NetworkVpcNetwork:NetworkVpcNetwork∂VMware Engine service VPC networks that provide connectivity from a private cloud to customer projects,
the internet, and other Google Cloud services.
Structure is documented below.
*åÖ
N
vmwareengineNetworkPeering.gcp:vmwareengine/networkPeering:NetworkPeeringòfRepresents a network peering resource. Network peerings are global resources.


To get more information about NetworkPeering, see:

* [API documentation](https://cloud.google.com/compute/docs/reference/rest/v1/networks/addPeering)

## Example Usage

### Vmware Engine Network Peering Ven


<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as gcp from "@pulumi/gcp";

const network_peering_nw = new gcp.vmwareengine.Network("network-peering-nw", {
    name: "default-np-nw",
    location: "global",
    type: "STANDARD",
});
const network_peering_peer_nw = new gcp.vmwareengine.Network("network-peering-peer-nw", {
    name: "peer-np-nw",
    location: "global",
    type: "STANDARD",
});
const vmw_engine_network_peering = new gcp.vmwareengine.NetworkPeering("vmw-engine-network-peering", {
    name: "sample-network-peering",
    description: "Sample description",
    vmwareEngineNetwork: network_peering_nw.id,
    peerNetwork: network_peering_peer_nw.id,
    peerNetworkType: "VMWARE_ENGINE_NETWORK",
    exportCustomRoutes: false,
    importCustomRoutes: false,
    exportCustomRoutesWithPublicIp: false,
    importCustomRoutesWithPublicIp: false,
});
```
```python
import pulumi
import pulumi_gcp as gcp

network_peering_nw = gcp.vmwareengine.Network("network-peering-nw",
    name="default-np-nw",
    location="global",
    type="STANDARD")
network_peering_peer_nw = gcp.vmwareengine.Network("network-peering-peer-nw",
    name="peer-np-nw",
    location="global",
    type="STANDARD")
vmw_engine_network_peering = gcp.vmwareengine.NetworkPeering("vmw-engine-network-peering",
    name="sample-network-peering",
    description="Sample description",
    vmware_engine_network=network_peering_nw.id,
    peer_network=network_peering_peer_nw.id,
    peer_network_type="VMWARE_ENGINE_NETWORK",
    export_custom_routes=False,
    import_custom_routes=False,
    export_custom_routes_with_public_ip=False,
    import_custom_routes_with_public_ip=False)
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Gcp = Pulumi.Gcp;

return await Deployment.RunAsync(() => 
{
    var network_peering_nw = new Gcp.VMwareEngine.Network("network-peering-nw", new()
    {
        Name = "default-np-nw",
        Location = "global",
        Type = "STANDARD",
    });

    var network_peering_peer_nw = new Gcp.VMwareEngine.Network("network-peering-peer-nw", new()
    {
        Name = "peer-np-nw",
        Location = "global",
        Type = "STANDARD",
    });

    var vmw_engine_network_peering = new Gcp.VMwareEngine.NetworkPeering("vmw-engine-network-peering", new()
    {
        Name = "sample-network-peering",
        Description = "Sample description",
        VmwareEngineNetwork = network_peering_nw.Id,
        PeerNetwork = network_peering_peer_nw.Id,
        PeerNetworkType = "VMWARE_ENGINE_NETWORK",
        ExportCustomRoutes = false,
        ImportCustomRoutes = false,
        ExportCustomRoutesWithPublicIp = false,
        ImportCustomRoutesWithPublicIp = false,
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-gcp/sdk/v8/go/gcp/vmwareengine"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := vmwareengine.NewNetwork(ctx, "network-peering-nw", &vmwareengine.NetworkArgs{
			Name:     pulumi.String("default-np-nw"),
			Location: pulumi.String("global"),
			Type:     pulumi.String("STANDARD"),
		})
		if err != nil {
			return err
		}
		_, err = vmwareengine.NewNetwork(ctx, "network-peering-peer-nw", &vmwareengine.NetworkArgs{
			Name:     pulumi.String("peer-np-nw"),
			Location: pulumi.String("global"),
			Type:     pulumi.String("STANDARD"),
		})
		if err != nil {
			return err
		}
		_, err = vmwareengine.NewNetworkPeering(ctx, "vmw-engine-network-peering", &vmwareengine.NetworkPeeringArgs{
			Name:                           pulumi.String("sample-network-peering"),
			Description:                    pulumi.String("Sample description"),
			VmwareEngineNetwork:            network_peering_nw.ID(),
			PeerNetwork:                    network_peering_peer_nw.ID(),
			PeerNetworkType:                pulumi.String("VMWARE_ENGINE_NETWORK"),
			ExportCustomRoutes:             pulumi.Bool(false),
			ImportCustomRoutes:             pulumi.Bool(false),
			ExportCustomRoutesWithPublicIp: pulumi.Bool(false),
			ImportCustomRoutesWithPublicIp: pulumi.Bool(false),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.gcp.vmwareengine.Network;
import com.pulumi.gcp.vmwareengine.NetworkArgs;
import com.pulumi.gcp.vmwareengine.NetworkPeering;
import com.pulumi.gcp.vmwareengine.NetworkPeeringArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var network_peering_nw = new Network("network-peering-nw", NetworkArgs.builder()
            .name("default-np-nw")
            .location("global")
            .type("STANDARD")
            .build());

        var network_peering_peer_nw = new Network("network-peering-peer-nw", NetworkArgs.builder()
            .name("peer-np-nw")
            .location("global")
            .type("STANDARD")
            .build());

        var vmw_engine_network_peering = new NetworkPeering("vmw-engine-network-peering", NetworkPeeringArgs.builder()
            .name("sample-network-peering")
            .description("Sample description")
            .vmwareEngineNetwork(network_peering_nw.id())
            .peerNetwork(network_peering_peer_nw.id())
            .peerNetworkType("VMWARE_ENGINE_NETWORK")
            .exportCustomRoutes(false)
            .importCustomRoutes(false)
            .exportCustomRoutesWithPublicIp(false)
            .importCustomRoutesWithPublicIp(false)
            .build());

    }
}
```
```yaml
resources:
  network-peering-nw:
    type: gcp:vmwareengine:Network
    properties:
      name: default-np-nw
      location: global
      type: STANDARD
  network-peering-peer-nw:
    type: gcp:vmwareengine:Network
    properties:
      name: peer-np-nw
      location: global
      type: STANDARD
  vmw-engine-network-peering:
    type: gcp:vmwareengine:NetworkPeering
    properties:
      name: sample-network-peering
      description: Sample description
      vmwareEngineNetwork: ${["network-peering-nw"].id}
      peerNetwork: ${["network-peering-peer-nw"].id}
      peerNetworkType: VMWARE_ENGINE_NETWORK
      exportCustomRoutes: false
      importCustomRoutes: false
      exportCustomRoutesWithPublicIp: false
      importCustomRoutesWithPublicIp: false
```
<!--End PulumiCodeChooser -->
### Vmware Engine Network Peering Standard


<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as gcp from "@pulumi/gcp";

const network_peering_vpc = new gcp.compute.Network("network-peering-vpc", {name: "default-vpc"});
const network_peering_standard_nw = new gcp.vmwareengine.Network("network-peering-standard-nw", {
    name: "default-standard-nw-np",
    location: "global",
    type: "STANDARD",
});
const vmw_engine_network_peering = new gcp.vmwareengine.NetworkPeering("vmw-engine-network-peering", {
    name: "sample-network-peering",
    description: "Sample description",
    peerNetwork: network_peering_vpc.id,
    peerNetworkType: "STANDARD",
    vmwareEngineNetwork: network_peering_standard_nw.id,
});
```
```python
import pulumi
import pulumi_gcp as gcp

network_peering_vpc = gcp.compute.Network("network-peering-vpc", name="default-vpc")
network_peering_standard_nw = gcp.vmwareengine.Network("network-peering-standard-nw",
    name="default-standard-nw-np",
    location="global",
    type="STANDARD")
vmw_engine_network_peering = gcp.vmwareengine.NetworkPeering("vmw-engine-network-peering",
    name="sample-network-peering",
    description="Sample description",
    peer_network=network_peering_vpc.id,
    peer_network_type="STANDARD",
    vmware_engine_network=network_peering_standard_nw.id)
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Gcp = Pulumi.Gcp;

return await Deployment.RunAsync(() => 
{
    var network_peering_vpc = new Gcp.Compute.Network("network-peering-vpc", new()
    {
        Name = "default-vpc",
    });

    var network_peering_standard_nw = new Gcp.VMwareEngine.Network("network-peering-standard-nw", new()
    {
        Name = "default-standard-nw-np",
        Location = "global",
        Type = "STANDARD",
    });

    var vmw_engine_network_peering = new Gcp.VMwareEngine.NetworkPeering("vmw-engine-network-peering", new()
    {
        Name = "sample-network-peering",
        Description = "Sample description",
        PeerNetwork = network_peering_vpc.Id,
        PeerNetworkType = "STANDARD",
        VmwareEngineNetwork = network_peering_standard_nw.Id,
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-gcp/sdk/v8/go/gcp/compute"
	"github.com/pulumi/pulumi-gcp/sdk/v8/go/gcp/vmwareengine"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := compute.NewNetwork(ctx, "network-peering-vpc", &compute.NetworkArgs{
			Name: pulumi.String("default-vpc"),
		})
		if err != nil {
			return err
		}
		_, err = vmwareengine.NewNetwork(ctx, "network-peering-standard-nw", &vmwareengine.NetworkArgs{
			Name:     pulumi.String("default-standard-nw-np"),
			Location: pulumi.String("global"),
			Type:     pulumi.String("STANDARD"),
		})
		if err != nil {
			return err
		}
		_, err = vmwareengine.NewNetworkPeering(ctx, "vmw-engine-network-peering", &vmwareengine.NetworkPeeringArgs{
			Name:                pulumi.String("sample-network-peering"),
			Description:         pulumi.String("Sample description"),
			PeerNetwork:         network_peering_vpc.ID(),
			PeerNetworkType:     pulumi.String("STANDARD"),
			VmwareEngineNetwork: network_peering_standard_nw.ID(),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.gcp.compute.Network;
import com.pulumi.gcp.compute.NetworkArgs;
import com.pulumi.gcp.vmwareengine.Network;
import com.pulumi.gcp.vmwareengine.NetworkArgs;
import com.pulumi.gcp.vmwareengine.NetworkPeering;
import com.pulumi.gcp.vmwareengine.NetworkPeeringArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var network_peering_vpc = new Network("network-peering-vpc", NetworkArgs.builder()
            .name("default-vpc")
            .build());

        var network_peering_standard_nw = new Network("network-peering-standard-nw", NetworkArgs.builder()
            .name("default-standard-nw-np")
            .location("global")
            .type("STANDARD")
            .build());

        var vmw_engine_network_peering = new NetworkPeering("vmw-engine-network-peering", NetworkPeeringArgs.builder()
            .name("sample-network-peering")
            .description("Sample description")
            .peerNetwork(network_peering_vpc.id())
            .peerNetworkType("STANDARD")
            .vmwareEngineNetwork(network_peering_standard_nw.id())
            .build());

    }
}
```
```yaml
resources:
  network-peering-vpc:
    type: gcp:compute:Network
    properties:
      name: default-vpc
  network-peering-standard-nw:
    type: gcp:vmwareengine:Network
    properties:
      name: default-standard-nw-np
      location: global
      type: STANDARD
  vmw-engine-network-peering:
    type: gcp:vmwareengine:NetworkPeering
    properties:
      name: sample-network-peering
      description: Sample description
      peerNetwork: ${["network-peering-vpc"].id}
      peerNetworkType: STANDARD
      vmwareEngineNetwork: ${["network-peering-standard-nw"].id}
```
<!--End PulumiCodeChooser -->

## Import

NetworkPeering can be imported using any of these accepted formats:

* `projects/{{project}}/locations/global/networkPeerings/{{name}}`

* `{{project}}/{{name}}`

* `{{name}}`

When using the `pulumi import` command, NetworkPeering can be imported using one of the formats above. For example:

```sh
$ pulumi import gcp:vmwareengine/networkPeering:NetworkPeering default projects/{{project}}/locations/global/networkPeerings/{{name}}
```

```sh
$ pulumi import gcp:vmwareengine/networkPeering:NetworkPeering default {{project}}/{{name}}
```

```sh
$ pulumi import gcp:vmwareengine/networkPeering:NetworkPeering default {{name}}
```

I
descriptionB" 4User-provided description for this network peering.
g
exportCustomRoutesB
 KTrue if custom routes are exported to the peered network; false otherwise.
Ä
exportCustomRoutesWithPublicIpB
 XTrue if all subnet routes with a public IP address range are exported; false otherwise.
i
importCustomRoutesB
 MTrue if custom routes are imported from the peered network; false otherwise.
u
importCustomRoutesWithPublicIpB
 MTrue if custom routes are imported from the peered network; false otherwise.
5
nameB" 'The ID of the Network Peering.


- - -
À
peerNetwork" ∑The relative resource name of the network to peer with a standard VMware Engine network.
The provided network can be a consumer VPC network or another standard VMware Engine network.
Ô
peerNetworkType" ◊The type of the network to peer with the VMware Engine network.
Possible values are: `STANDARD`, `VMWARE_ENGINE_NETWORK`, `PRIVATE_SERVICES_ACCESS`, `NETAPP_CLOUD_VOLUMES`, `THIRD_PARTY_SERVICE`, `DELL_POWERSCALE`.
{
projectB" jThe ID of the project in which the resource belongs.
If it is not provided, the provider project is used.
í
vmwareEngineNetwork" ˆThe relative resource name of the VMware Engine network. Specify the name in the following form:
projects/{project}/locations/{location}/vmwareEngineNetworks/{vmwareEngineNetworkId} where {project}
can either be a project number or a project ID.
"·

createTime" ŒCreation time of this resource.
A timestamp in RFC3339 UTC "Zulu" format, with nanosecond resolution and
up to nine fractional digits. Examples: "2014-10-02T15:01:23Z" and "2014-10-02T15:01:23.045123456Z".
"I
descriptionB" 4User-provided description for this network peering.
"g
exportCustomRoutesB
 KTrue if custom routes are exported to the peered network; false otherwise.
"Ä
exportCustomRoutesWithPublicIpB
 XTrue if all subnet routes with a public IP address range are exported; false otherwise.
"i
importCustomRoutesB
 MTrue if custom routes are imported from the peered network; false otherwise.
"u
importCustomRoutesWithPublicIpB
 MTrue if custom routes are imported from the peered network; false otherwise.
"3
name" 'The ID of the Network Peering.


- - -
"À
peerNetwork" ∑The relative resource name of the network to peer with a standard VMware Engine network.
The provided network can be a consumer VPC network or another standard VMware Engine network.
"Ô
peerNetworkType" ◊The type of the network to peer with the VMware Engine network.
Possible values are: `STANDARD`, `VMWARE_ENGINE_NETWORK`, `PRIVATE_SERVICES_ACCESS`, `NETAPP_CLOUD_VOLUMES`, `THIRD_PARTY_SERVICE`, `DELL_POWERSCALE`.
"y
project" jThe ID of the project in which the resource belongs.
If it is not provided, the provider project is used.
"¡
state" ≥State of the network peering.
This field has a value of 'ACTIVE' when there's a matching configuration in the peer network.
New values may be added to this enum when appropriate.
"L
stateDetails" 8Details about the current state of the network peering.
"@
uid" 5System-generated unique identifier for the resource.
"Â

updateTime" “Last updated time of this resource.
A timestamp in RFC3339 UTC "Zulu" format, with nanosecond resolution and up to nine
fractional digits. Examples: "2014-10-02T15:01:23Z" and "2014-10-02T15:01:23.045123456Z".
"í
vmwareEngineNetwork" ˆThe relative resource name of the VMware Engine network. Specify the name in the following form:
projects/{project}/locations/{location}/vmwareEngineNetworks/{vmwareEngineNetworkId} where {project}
can either be a project number or a project ID.
"æ
vmwareEngineNetworkCanonical" ôThe canonical name of the VMware Engine network in the form:
projects/{project_number}/locations/{location}/vmwareEngineNetworks/{vmwareEngineNetworkId}
*€x
K
vmwareengineNetworkPolicy,gcp:vmwareengine/networkPolicy:NetworkPolicyûXRepresents a network policy resource. Network policies are regional resources.


To get more information about NetworkPolicy, see:

* [API documentation](https://cloud.google.com/vmware-engine/docs/reference/rest/v1/projects.locations.networkPolicies)

## Example Usage

### Vmware Engine Network Policy Basic


<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as gcp from "@pulumi/gcp";

const network_policy_nw = new gcp.vmwareengine.Network("network-policy-nw", {
    name: "sample-network",
    location: "global",
    type: "STANDARD",
    description: "VMwareEngine standard network sample",
});
const vmw_engine_network_policy = new gcp.vmwareengine.NetworkPolicy("vmw-engine-network-policy", {
    location: "us-west1",
    name: "sample-network-policy",
    edgeServicesCidr: "192.168.30.0/26",
    vmwareEngineNetwork: network_policy_nw.id,
});
```
```python
import pulumi
import pulumi_gcp as gcp

network_policy_nw = gcp.vmwareengine.Network("network-policy-nw",
    name="sample-network",
    location="global",
    type="STANDARD",
    description="VMwareEngine standard network sample")
vmw_engine_network_policy = gcp.vmwareengine.NetworkPolicy("vmw-engine-network-policy",
    location="us-west1",
    name="sample-network-policy",
    edge_services_cidr="192.168.30.0/26",
    vmware_engine_network=network_policy_nw.id)
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Gcp = Pulumi.Gcp;

return await Deployment.RunAsync(() => 
{
    var network_policy_nw = new Gcp.VMwareEngine.Network("network-policy-nw", new()
    {
        Name = "sample-network",
        Location = "global",
        Type = "STANDARD",
        Description = "VMwareEngine standard network sample",
    });

    var vmw_engine_network_policy = new Gcp.VMwareEngine.NetworkPolicy("vmw-engine-network-policy", new()
    {
        Location = "us-west1",
        Name = "sample-network-policy",
        EdgeServicesCidr = "192.168.30.0/26",
        VmwareEngineNetwork = network_policy_nw.Id,
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-gcp/sdk/v8/go/gcp/vmwareengine"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := vmwareengine.NewNetwork(ctx, "network-policy-nw", &vmwareengine.NetworkArgs{
			Name:        pulumi.String("sample-network"),
			Location:    pulumi.String("global"),
			Type:        pulumi.String("STANDARD"),
			Description: pulumi.String("VMwareEngine standard network sample"),
		})
		if err != nil {
			return err
		}
		_, err = vmwareengine.NewNetworkPolicy(ctx, "vmw-engine-network-policy", &vmwareengine.NetworkPolicyArgs{
			Location:            pulumi.String("us-west1"),
			Name:                pulumi.String("sample-network-policy"),
			EdgeServicesCidr:    pulumi.String("192.168.30.0/26"),
			VmwareEngineNetwork: network_policy_nw.ID(),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.gcp.vmwareengine.Network;
import com.pulumi.gcp.vmwareengine.NetworkArgs;
import com.pulumi.gcp.vmwareengine.NetworkPolicy;
import com.pulumi.gcp.vmwareengine.NetworkPolicyArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var network_policy_nw = new Network("network-policy-nw", NetworkArgs.builder()
            .name("sample-network")
            .location("global")
            .type("STANDARD")
            .description("VMwareEngine standard network sample")
            .build());

        var vmw_engine_network_policy = new NetworkPolicy("vmw-engine-network-policy", NetworkPolicyArgs.builder()
            .location("us-west1")
            .name("sample-network-policy")
            .edgeServicesCidr("192.168.30.0/26")
            .vmwareEngineNetwork(network_policy_nw.id())
            .build());

    }
}
```
```yaml
resources:
  network-policy-nw:
    type: gcp:vmwareengine:Network
    properties:
      name: sample-network
      location: global
      type: STANDARD
      description: VMwareEngine standard network sample
  vmw-engine-network-policy:
    type: gcp:vmwareengine:NetworkPolicy
    properties:
      location: us-west1
      name: sample-network-policy
      edgeServicesCidr: 192.168.30.0/26
      vmwareEngineNetwork: ${["network-policy-nw"].id}
```
<!--End PulumiCodeChooser -->
### Vmware Engine Network Policy Full


<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as gcp from "@pulumi/gcp";

const network_policy_nw = new gcp.vmwareengine.Network("network-policy-nw", {
    name: "sample-network",
    location: "global",
    type: "STANDARD",
    description: "VMwareEngine standard network sample",
});
const vmw_engine_network_policy = new gcp.vmwareengine.NetworkPolicy("vmw-engine-network-policy", {
    location: "us-west1",
    name: "sample-network-policy",
    edgeServicesCidr: "192.168.30.0/26",
    vmwareEngineNetwork: network_policy_nw.id,
    description: "Sample Network Policy",
    internetAccess: {
        enabled: true,
    },
    externalIp: {
        enabled: true,
    },
});
```
```python
import pulumi
import pulumi_gcp as gcp

network_policy_nw = gcp.vmwareengine.Network("network-policy-nw",
    name="sample-network",
    location="global",
    type="STANDARD",
    description="VMwareEngine standard network sample")
vmw_engine_network_policy = gcp.vmwareengine.NetworkPolicy("vmw-engine-network-policy",
    location="us-west1",
    name="sample-network-policy",
    edge_services_cidr="192.168.30.0/26",
    vmware_engine_network=network_policy_nw.id,
    description="Sample Network Policy",
    internet_access={
        "enabled": True,
    },
    external_ip={
        "enabled": True,
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Gcp = Pulumi.Gcp;

return await Deployment.RunAsync(() => 
{
    var network_policy_nw = new Gcp.VMwareEngine.Network("network-policy-nw", new()
    {
        Name = "sample-network",
        Location = "global",
        Type = "STANDARD",
        Description = "VMwareEngine standard network sample",
    });

    var vmw_engine_network_policy = new Gcp.VMwareEngine.NetworkPolicy("vmw-engine-network-policy", new()
    {
        Location = "us-west1",
        Name = "sample-network-policy",
        EdgeServicesCidr = "192.168.30.0/26",
        VmwareEngineNetwork = network_policy_nw.Id,
        Description = "Sample Network Policy",
        InternetAccess = new Gcp.VMwareEngine.Inputs.NetworkPolicyInternetAccessArgs
        {
            Enabled = true,
        },
        ExternalIp = new Gcp.VMwareEngine.Inputs.NetworkPolicyExternalIpArgs
        {
            Enabled = true,
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-gcp/sdk/v8/go/gcp/vmwareengine"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := vmwareengine.NewNetwork(ctx, "network-policy-nw", &vmwareengine.NetworkArgs{
			Name:        pulumi.String("sample-network"),
			Location:    pulumi.String("global"),
			Type:        pulumi.String("STANDARD"),
			Description: pulumi.String("VMwareEngine standard network sample"),
		})
		if err != nil {
			return err
		}
		_, err = vmwareengine.NewNetworkPolicy(ctx, "vmw-engine-network-policy", &vmwareengine.NetworkPolicyArgs{
			Location:            pulumi.String("us-west1"),
			Name:                pulumi.String("sample-network-policy"),
			EdgeServicesCidr:    pulumi.String("192.168.30.0/26"),
			VmwareEngineNetwork: network_policy_nw.ID(),
			Description:         pulumi.String("Sample Network Policy"),
			InternetAccess: &vmwareengine.NetworkPolicyInternetAccessArgs{
				Enabled: pulumi.Bool(true),
			},
			ExternalIp: &vmwareengine.NetworkPolicyExternalIpArgs{
				Enabled: pulumi.Bool(true),
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.gcp.vmwareengine.Network;
import com.pulumi.gcp.vmwareengine.NetworkArgs;
import com.pulumi.gcp.vmwareengine.NetworkPolicy;
import com.pulumi.gcp.vmwareengine.NetworkPolicyArgs;
import com.pulumi.gcp.vmwareengine.inputs.NetworkPolicyInternetAccessArgs;
import com.pulumi.gcp.vmwareengine.inputs.NetworkPolicyExternalIpArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var network_policy_nw = new Network("network-policy-nw", NetworkArgs.builder()
            .name("sample-network")
            .location("global")
            .type("STANDARD")
            .description("VMwareEngine standard network sample")
            .build());

        var vmw_engine_network_policy = new NetworkPolicy("vmw-engine-network-policy", NetworkPolicyArgs.builder()
            .location("us-west1")
            .name("sample-network-policy")
            .edgeServicesCidr("192.168.30.0/26")
            .vmwareEngineNetwork(network_policy_nw.id())
            .description("Sample Network Policy")
            .internetAccess(NetworkPolicyInternetAccessArgs.builder()
                .enabled(true)
                .build())
            .externalIp(NetworkPolicyExternalIpArgs.builder()
                .enabled(true)
                .build())
            .build());

    }
}
```
```yaml
resources:
  network-policy-nw:
    type: gcp:vmwareengine:Network
    properties:
      name: sample-network
      location: global
      type: STANDARD
      description: VMwareEngine standard network sample
  vmw-engine-network-policy:
    type: gcp:vmwareengine:NetworkPolicy
    properties:
      location: us-west1
      name: sample-network-policy
      edgeServicesCidr: 192.168.30.0/26
      vmwareEngineNetwork: ${["network-policy-nw"].id}
      description: Sample Network Policy
      internetAccess:
        enabled: true
      externalIp:
        enabled: true
```
<!--End PulumiCodeChooser -->

## Import

NetworkPolicy can be imported using any of these accepted formats:

* `projects/{{project}}/locations/{{location}}/networkPolicies/{{name}}`

* `{{project}}/{{location}}/{{name}}`

* `{{location}}/{{name}}`

When using the `pulumi import` command, NetworkPolicy can be imported using one of the formats above. For example:

```sh
$ pulumi import gcp:vmwareengine/networkPolicy:NetworkPolicy default projects/{{project}}/locations/{{location}}/networkPolicies/{{name}}
```

```sh
$ pulumi import gcp:vmwareengine/networkPolicy:NetworkPolicy default {{project}}/{{location}}/{{name}}
```

```sh
$ pulumi import gcp:vmwareengine/networkPolicy:NetworkPolicy default {{location}}/{{name}}
```

H
descriptionB" 3User-provided description for this network policy.
∏
edgeServicesCidr" üIP address range in CIDR notation used to create internet access and external IP access.
An RFC 1918 CIDR block, with a "/26" prefix, is required. The range cannot overlap with any
prefixes either in the consumer VPC network or in use by the private clouds attached to that VPC network.
ª

externalIpoBm:k
i
vmwareengineNetworkPolicyExternalIp@gcp:vmwareengine/NetworkPolicyExternalIp:NetworkPolicyExternalIpªNetwork service that allows External IP addresses to be assigned to VMware workloads.
This service can only be enabled when internetAccess is also enabled.
Structure is documented below.
Û
internetAccess{By:w
u
vmwareengineNetworkPolicyInternetAccessHgcp:vmwareengine/NetworkPolicyInternetAccess:NetworkPolicyInternetAccessdNetwork service that allows VMware workloads to access the internet.
Structure is documented below.
è
location" ˛The resource name of the location (region) to create the new network policy in.
Resource names are schemeless URIs that follow the conventions in https://cloud.google.com/apis/design/resource_names.
For example: projects/my-project/locations/us-central1
4
nameB" &The ID of the Network Policy.


- - -
{
projectB" jThe ID of the project in which the resource belongs.
If it is not provided, the provider project is used.
í
vmwareEngineNetwork" ˆThe relative resource name of the VMware Engine network. Specify the name in the following form:
projects/{project}/locations/{location}/vmwareEngineNetworks/{vmwareEngineNetworkId} where {project}
can either be a project number or a project ID.
"·

createTime" ŒCreation time of this resource.
A timestamp in RFC3339 UTC "Zulu" format, with nanosecond resolution and
up to nine fractional digits. Examples: "2014-10-02T15:01:23Z" and "2014-10-02T15:01:23.045123456Z".
"H
descriptionB" 3User-provided description for this network policy.
"∏
edgeServicesCidr" üIP address range in CIDR notation used to create internet access and external IP access.
An RFC 1918 CIDR block, with a "/26" prefix, is required. The range cannot overlap with any
prefixes either in the consumer VPC network or in use by the private clouds attached to that VPC network.
"π

externalIpm:k
i
vmwareengineNetworkPolicyExternalIp@gcp:vmwareengine/NetworkPolicyExternalIp:NetworkPolicyExternalIpªNetwork service that allows External IP addresses to be assigned to VMware workloads.
This service can only be enabled when internetAccess is also enabled.
Structure is documented below.
"Ò
internetAccessy:w
u
vmwareengineNetworkPolicyInternetAccessHgcp:vmwareengine/NetworkPolicyInternetAccess:NetworkPolicyInternetAccessdNetwork service that allows VMware workloads to access the internet.
Structure is documented below.
"è
location" ˛The resource name of the location (region) to create the new network policy in.
Resource names are schemeless URIs that follow the conventions in https://cloud.google.com/apis/design/resource_names.
For example: projects/my-project/locations/us-central1
"2
name" &The ID of the Network Policy.


- - -
"y
project" jThe ID of the project in which the resource belongs.
If it is not provided, the provider project is used.
"@
uid" 5System-generated unique identifier for the resource.
"Â

updateTime" “Last updated time of this resource.
A timestamp in RFC3339 UTC "Zulu" format, with nanosecond resolution and up to nine
fractional digits. Examples: "2014-10-02T15:01:23Z" and "2014-10-02T15:01:23.045123456Z".
"í
vmwareEngineNetwork" ˆThe relative resource name of the VMware Engine network. Specify the name in the following form:
projects/{project}/locations/{location}/vmwareEngineNetworks/{vmwareEngineNetworkId} where {project}
can either be a project number or a project ID.
"æ
vmwareEngineNetworkCanonical" ôThe canonical name of the VMware Engine network in the form:
projects/{project_number}/locations/{location}/vmwareEngineNetworks/{vmwareEngineNetworkId}
*˘æ
H
vmwareenginePrivateCloud*gcp:vmwareengine/privateCloud:PrivateCloudã£Represents a private cloud resource. Private clouds are zonal resources.


To get more information about PrivateCloud, see:

* [API documentation](https://cloud.google.com/vmware-engine/docs/reference/rest/v1/projects.locations.privateClouds)

## Example Usage

### Vmware Engine Private Cloud Basic


<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as gcp from "@pulumi/gcp";

const pc_nw = new gcp.vmwareengine.Network("pc-nw", {
    name: "pc-nw",
    location: "global",
    type: "STANDARD",
    description: "PC network description.",
});
const vmw_engine_pc = new gcp.vmwareengine.PrivateCloud("vmw-engine-pc", {
    location: "us-west1-a",
    name: "sample-pc",
    description: "Sample test PC.",
    networkConfig: {
        managementCidr: "192.168.30.0/24",
        vmwareEngineNetwork: pc_nw.id,
    },
    managementCluster: {
        clusterId: "sample-mgmt-cluster",
        nodeTypeConfigs: [{
            nodeTypeId: "standard-72",
            nodeCount: 3,
        }],
    },
});
```
```python
import pulumi
import pulumi_gcp as gcp

pc_nw = gcp.vmwareengine.Network("pc-nw",
    name="pc-nw",
    location="global",
    type="STANDARD",
    description="PC network description.")
vmw_engine_pc = gcp.vmwareengine.PrivateCloud("vmw-engine-pc",
    location="us-west1-a",
    name="sample-pc",
    description="Sample test PC.",
    network_config={
        "management_cidr": "192.168.30.0/24",
        "vmware_engine_network": pc_nw.id,
    },
    management_cluster={
        "cluster_id": "sample-mgmt-cluster",
        "node_type_configs": [{
            "node_type_id": "standard-72",
            "node_count": 3,
        }],
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Gcp = Pulumi.Gcp;

return await Deployment.RunAsync(() => 
{
    var pc_nw = new Gcp.VMwareEngine.Network("pc-nw", new()
    {
        Name = "pc-nw",
        Location = "global",
        Type = "STANDARD",
        Description = "PC network description.",
    });

    var vmw_engine_pc = new Gcp.VMwareEngine.PrivateCloud("vmw-engine-pc", new()
    {
        Location = "us-west1-a",
        Name = "sample-pc",
        Description = "Sample test PC.",
        NetworkConfig = new Gcp.VMwareEngine.Inputs.PrivateCloudNetworkConfigArgs
        {
            ManagementCidr = "192.168.30.0/24",
            VmwareEngineNetwork = pc_nw.Id,
        },
        ManagementCluster = new Gcp.VMwareEngine.Inputs.PrivateCloudManagementClusterArgs
        {
            ClusterId = "sample-mgmt-cluster",
            NodeTypeConfigs = new[]
            {
                new Gcp.VMwareEngine.Inputs.PrivateCloudManagementClusterNodeTypeConfigArgs
                {
                    NodeTypeId = "standard-72",
                    NodeCount = 3,
                },
            },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-gcp/sdk/v8/go/gcp/vmwareengine"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := vmwareengine.NewNetwork(ctx, "pc-nw", &vmwareengine.NetworkArgs{
			Name:        pulumi.String("pc-nw"),
			Location:    pulumi.String("global"),
			Type:        pulumi.String("STANDARD"),
			Description: pulumi.String("PC network description."),
		})
		if err != nil {
			return err
		}
		_, err = vmwareengine.NewPrivateCloud(ctx, "vmw-engine-pc", &vmwareengine.PrivateCloudArgs{
			Location:    pulumi.String("us-west1-a"),
			Name:        pulumi.String("sample-pc"),
			Description: pulumi.String("Sample test PC."),
			NetworkConfig: &vmwareengine.PrivateCloudNetworkConfigArgs{
				ManagementCidr:      pulumi.String("192.168.30.0/24"),
				VmwareEngineNetwork: pc_nw.ID(),
			},
			ManagementCluster: &vmwareengine.PrivateCloudManagementClusterArgs{
				ClusterId: pulumi.String("sample-mgmt-cluster"),
				NodeTypeConfigs: vmwareengine.PrivateCloudManagementClusterNodeTypeConfigArray{
					&vmwareengine.PrivateCloudManagementClusterNodeTypeConfigArgs{
						NodeTypeId: pulumi.String("standard-72"),
						NodeCount:  pulumi.Int(3),
					},
				},
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.gcp.vmwareengine.Network;
import com.pulumi.gcp.vmwareengine.NetworkArgs;
import com.pulumi.gcp.vmwareengine.PrivateCloud;
import com.pulumi.gcp.vmwareengine.PrivateCloudArgs;
import com.pulumi.gcp.vmwareengine.inputs.PrivateCloudNetworkConfigArgs;
import com.pulumi.gcp.vmwareengine.inputs.PrivateCloudManagementClusterArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var pc_nw = new Network("pc-nw", NetworkArgs.builder()
            .name("pc-nw")
            .location("global")
            .type("STANDARD")
            .description("PC network description.")
            .build());

        var vmw_engine_pc = new PrivateCloud("vmw-engine-pc", PrivateCloudArgs.builder()
            .location("us-west1-a")
            .name("sample-pc")
            .description("Sample test PC.")
            .networkConfig(PrivateCloudNetworkConfigArgs.builder()
                .managementCidr("192.168.30.0/24")
                .vmwareEngineNetwork(pc_nw.id())
                .build())
            .managementCluster(PrivateCloudManagementClusterArgs.builder()
                .clusterId("sample-mgmt-cluster")
                .nodeTypeConfigs(PrivateCloudManagementClusterNodeTypeConfigArgs.builder()
                    .nodeTypeId("standard-72")
                    .nodeCount(3)
                    .build())
                .build())
            .build());

    }
}
```
```yaml
resources:
  vmw-engine-pc:
    type: gcp:vmwareengine:PrivateCloud
    properties:
      location: us-west1-a
      name: sample-pc
      description: Sample test PC.
      networkConfig:
        managementCidr: 192.168.30.0/24
        vmwareEngineNetwork: ${["pc-nw"].id}
      managementCluster:
        clusterId: sample-mgmt-cluster
        nodeTypeConfigs:
          - nodeTypeId: standard-72
            nodeCount: 3
  pc-nw:
    type: gcp:vmwareengine:Network
    properties:
      name: pc-nw
      location: global
      type: STANDARD
      description: PC network description.
```
<!--End PulumiCodeChooser -->
### Vmware Engine Private Cloud Full


<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as gcp from "@pulumi/gcp";

const pc_nw = new gcp.vmwareengine.Network("pc-nw", {
    name: "pc-nw",
    location: "global",
    type: "STANDARD",
    description: "PC network description.",
});
const vmw_engine_pc = new gcp.vmwareengine.PrivateCloud("vmw-engine-pc", {
    location: "us-west1-a",
    name: "sample-pc",
    description: "Sample test PC.",
    type: "TIME_LIMITED",
    networkConfig: {
        managementCidr: "192.168.30.0/24",
        vmwareEngineNetwork: pc_nw.id,
    },
    managementCluster: {
        clusterId: "sample-mgmt-cluster",
        nodeTypeConfigs: [{
            nodeTypeId: "standard-72",
            nodeCount: 1,
            customCoreCount: 32,
        }],
        autoscalingSettings: {
            autoscalingPolicies: [{
                autoscalePolicyId: "autoscaling-policy",
                nodeTypeId: "standard-72",
                scaleOutSize: 1,
                cpuThresholds: {
                    scaleOut: 80,
                    scaleIn: 15,
                },
                consumedMemoryThresholds: {
                    scaleOut: 75,
                    scaleIn: 20,
                },
                storageThresholds: {
                    scaleOut: 80,
                    scaleIn: 20,
                },
            }],
            minClusterNodeCount: 3,
            maxClusterNodeCount: 8,
            coolDownPeriod: "1800s",
        },
    },
    deletionDelayHours: 0,
    sendDeletionDelayHoursIfZero: true,
});
```
```python
import pulumi
import pulumi_gcp as gcp

pc_nw = gcp.vmwareengine.Network("pc-nw",
    name="pc-nw",
    location="global",
    type="STANDARD",
    description="PC network description.")
vmw_engine_pc = gcp.vmwareengine.PrivateCloud("vmw-engine-pc",
    location="us-west1-a",
    name="sample-pc",
    description="Sample test PC.",
    type="TIME_LIMITED",
    network_config={
        "management_cidr": "192.168.30.0/24",
        "vmware_engine_network": pc_nw.id,
    },
    management_cluster={
        "cluster_id": "sample-mgmt-cluster",
        "node_type_configs": [{
            "node_type_id": "standard-72",
            "node_count": 1,
            "custom_core_count": 32,
        }],
        "autoscaling_settings": {
            "autoscaling_policies": [{
                "autoscale_policy_id": "autoscaling-policy",
                "node_type_id": "standard-72",
                "scale_out_size": 1,
                "cpu_thresholds": {
                    "scale_out": 80,
                    "scale_in": 15,
                },
                "consumed_memory_thresholds": {
                    "scale_out": 75,
                    "scale_in": 20,
                },
                "storage_thresholds": {
                    "scale_out": 80,
                    "scale_in": 20,
                },
            }],
            "min_cluster_node_count": 3,
            "max_cluster_node_count": 8,
            "cool_down_period": "1800s",
        },
    },
    deletion_delay_hours=0,
    send_deletion_delay_hours_if_zero=True)
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Gcp = Pulumi.Gcp;

return await Deployment.RunAsync(() => 
{
    var pc_nw = new Gcp.VMwareEngine.Network("pc-nw", new()
    {
        Name = "pc-nw",
        Location = "global",
        Type = "STANDARD",
        Description = "PC network description.",
    });

    var vmw_engine_pc = new Gcp.VMwareEngine.PrivateCloud("vmw-engine-pc", new()
    {
        Location = "us-west1-a",
        Name = "sample-pc",
        Description = "Sample test PC.",
        Type = "TIME_LIMITED",
        NetworkConfig = new Gcp.VMwareEngine.Inputs.PrivateCloudNetworkConfigArgs
        {
            ManagementCidr = "192.168.30.0/24",
            VmwareEngineNetwork = pc_nw.Id,
        },
        ManagementCluster = new Gcp.VMwareEngine.Inputs.PrivateCloudManagementClusterArgs
        {
            ClusterId = "sample-mgmt-cluster",
            NodeTypeConfigs = new[]
            {
                new Gcp.VMwareEngine.Inputs.PrivateCloudManagementClusterNodeTypeConfigArgs
                {
                    NodeTypeId = "standard-72",
                    NodeCount = 1,
                    CustomCoreCount = 32,
                },
            },
            AutoscalingSettings = new Gcp.VMwareEngine.Inputs.PrivateCloudManagementClusterAutoscalingSettingsArgs
            {
                AutoscalingPolicies = new[]
                {
                    new Gcp.VMwareEngine.Inputs.PrivateCloudManagementClusterAutoscalingSettingsAutoscalingPolicyArgs
                    {
                        AutoscalePolicyId = "autoscaling-policy",
                        NodeTypeId = "standard-72",
                        ScaleOutSize = 1,
                        CpuThresholds = new Gcp.VMwareEngine.Inputs.PrivateCloudManagementClusterAutoscalingSettingsAutoscalingPolicyCpuThresholdsArgs
                        {
                            ScaleOut = 80,
                            ScaleIn = 15,
                        },
                        ConsumedMemoryThresholds = new Gcp.VMwareEngine.Inputs.PrivateCloudManagementClusterAutoscalingSettingsAutoscalingPolicyConsumedMemoryThresholdsArgs
                        {
                            ScaleOut = 75,
                            ScaleIn = 20,
                        },
                        StorageThresholds = new Gcp.VMwareEngine.Inputs.PrivateCloudManagementClusterAutoscalingSettingsAutoscalingPolicyStorageThresholdsArgs
                        {
                            ScaleOut = 80,
                            ScaleIn = 20,
                        },
                    },
                },
                MinClusterNodeCount = 3,
                MaxClusterNodeCount = 8,
                CoolDownPeriod = "1800s",
            },
        },
        DeletionDelayHours = 0,
        SendDeletionDelayHoursIfZero = true,
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-gcp/sdk/v8/go/gcp/vmwareengine"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := vmwareengine.NewNetwork(ctx, "pc-nw", &vmwareengine.NetworkArgs{
			Name:        pulumi.String("pc-nw"),
			Location:    pulumi.String("global"),
			Type:        pulumi.String("STANDARD"),
			Description: pulumi.String("PC network description."),
		})
		if err != nil {
			return err
		}
		_, err = vmwareengine.NewPrivateCloud(ctx, "vmw-engine-pc", &vmwareengine.PrivateCloudArgs{
			Location:    pulumi.String("us-west1-a"),
			Name:        pulumi.String("sample-pc"),
			Description: pulumi.String("Sample test PC."),
			Type:        pulumi.String("TIME_LIMITED"),
			NetworkConfig: &vmwareengine.PrivateCloudNetworkConfigArgs{
				ManagementCidr:      pulumi.String("192.168.30.0/24"),
				VmwareEngineNetwork: pc_nw.ID(),
			},
			ManagementCluster: &vmwareengine.PrivateCloudManagementClusterArgs{
				ClusterId: pulumi.String("sample-mgmt-cluster"),
				NodeTypeConfigs: vmwareengine.PrivateCloudManagementClusterNodeTypeConfigArray{
					&vmwareengine.PrivateCloudManagementClusterNodeTypeConfigArgs{
						NodeTypeId:      pulumi.String("standard-72"),
						NodeCount:       pulumi.Int(1),
						CustomCoreCount: pulumi.Int(32),
					},
				},
				AutoscalingSettings: &vmwareengine.PrivateCloudManagementClusterAutoscalingSettingsArgs{
					AutoscalingPolicies: vmwareengine.PrivateCloudManagementClusterAutoscalingSettingsAutoscalingPolicyArray{
						&vmwareengine.PrivateCloudManagementClusterAutoscalingSettingsAutoscalingPolicyArgs{
							AutoscalePolicyId: pulumi.String("autoscaling-policy"),
							NodeTypeId:        pulumi.String("standard-72"),
							ScaleOutSize:      pulumi.Int(1),
							CpuThresholds: &vmwareengine.PrivateCloudManagementClusterAutoscalingSettingsAutoscalingPolicyCpuThresholdsArgs{
								ScaleOut: pulumi.Int(80),
								ScaleIn:  pulumi.Int(15),
							},
							ConsumedMemoryThresholds: &vmwareengine.PrivateCloudManagementClusterAutoscalingSettingsAutoscalingPolicyConsumedMemoryThresholdsArgs{
								ScaleOut: pulumi.Int(75),
								ScaleIn:  pulumi.Int(20),
							},
							StorageThresholds: &vmwareengine.PrivateCloudManagementClusterAutoscalingSettingsAutoscalingPolicyStorageThresholdsArgs{
								ScaleOut: pulumi.Int(80),
								ScaleIn:  pulumi.Int(20),
							},
						},
					},
					MinClusterNodeCount: pulumi.Int(3),
					MaxClusterNodeCount: pulumi.Int(8),
					CoolDownPeriod:      pulumi.String("1800s"),
				},
			},
			DeletionDelayHours:           pulumi.Int(0),
			SendDeletionDelayHoursIfZero: pulumi.Bool(true),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.gcp.vmwareengine.Network;
import com.pulumi.gcp.vmwareengine.NetworkArgs;
import com.pulumi.gcp.vmwareengine.PrivateCloud;
import com.pulumi.gcp.vmwareengine.PrivateCloudArgs;
import com.pulumi.gcp.vmwareengine.inputs.PrivateCloudNetworkConfigArgs;
import com.pulumi.gcp.vmwareengine.inputs.PrivateCloudManagementClusterArgs;
import com.pulumi.gcp.vmwareengine.inputs.PrivateCloudManagementClusterAutoscalingSettingsArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var pc_nw = new Network("pc-nw", NetworkArgs.builder()
            .name("pc-nw")
            .location("global")
            .type("STANDARD")
            .description("PC network description.")
            .build());

        var vmw_engine_pc = new PrivateCloud("vmw-engine-pc", PrivateCloudArgs.builder()
            .location("us-west1-a")
            .name("sample-pc")
            .description("Sample test PC.")
            .type("TIME_LIMITED")
            .networkConfig(PrivateCloudNetworkConfigArgs.builder()
                .managementCidr("192.168.30.0/24")
                .vmwareEngineNetwork(pc_nw.id())
                .build())
            .managementCluster(PrivateCloudManagementClusterArgs.builder()
                .clusterId("sample-mgmt-cluster")
                .nodeTypeConfigs(PrivateCloudManagementClusterNodeTypeConfigArgs.builder()
                    .nodeTypeId("standard-72")
                    .nodeCount(1)
                    .customCoreCount(32)
                    .build())
                .autoscalingSettings(PrivateCloudManagementClusterAutoscalingSettingsArgs.builder()
                    .autoscalingPolicies(PrivateCloudManagementClusterAutoscalingSettingsAutoscalingPolicyArgs.builder()
                        .autoscalePolicyId("autoscaling-policy")
                        .nodeTypeId("standard-72")
                        .scaleOutSize(1)
                        .cpuThresholds(PrivateCloudManagementClusterAutoscalingSettingsAutoscalingPolicyCpuThresholdsArgs.builder()
                            .scaleOut(80)
                            .scaleIn(15)
                            .build())
                        .consumedMemoryThresholds(PrivateCloudManagementClusterAutoscalingSettingsAutoscalingPolicyConsumedMemoryThresholdsArgs.builder()
                            .scaleOut(75)
                            .scaleIn(20)
                            .build())
                        .storageThresholds(PrivateCloudManagementClusterAutoscalingSettingsAutoscalingPolicyStorageThresholdsArgs.builder()
                            .scaleOut(80)
                            .scaleIn(20)
                            .build())
                        .build())
                    .minClusterNodeCount(3)
                    .maxClusterNodeCount(8)
                    .coolDownPeriod("1800s")
                    .build())
                .build())
            .deletionDelayHours(0)
            .sendDeletionDelayHoursIfZero(true)
            .build());

    }
}
```
```yaml
resources:
  vmw-engine-pc:
    type: gcp:vmwareengine:PrivateCloud
    properties:
      location: us-west1-a
      name: sample-pc
      description: Sample test PC.
      type: TIME_LIMITED
      networkConfig:
        managementCidr: 192.168.30.0/24
        vmwareEngineNetwork: ${["pc-nw"].id}
      managementCluster:
        clusterId: sample-mgmt-cluster
        nodeTypeConfigs:
          - nodeTypeId: standard-72
            nodeCount: 1
            customCoreCount: 32
        autoscalingSettings:
          autoscalingPolicies:
            - autoscalePolicyId: autoscaling-policy
              nodeTypeId: standard-72
              scaleOutSize: 1
              cpuThresholds:
                scaleOut: 80
                scaleIn: 15
              consumedMemoryThresholds:
                scaleOut: 75
                scaleIn: 20
              storageThresholds:
                scaleOut: 80
                scaleIn: 20
          minClusterNodeCount: 3
          maxClusterNodeCount: 8
          coolDownPeriod: 1800s
      deletionDelayHours: 0
      sendDeletionDelayHoursIfZero: true
  pc-nw:
    type: gcp:vmwareengine:Network
    properties:
      name: pc-nw
      location: global
      type: STANDARD
      description: PC network description.
```
<!--End PulumiCodeChooser -->

## Import

PrivateCloud can be imported using any of these accepted formats:

* `projects/{{project}}/locations/{{location}}/privateClouds/{{name}}`

* `{{project}}/{{location}}/{{name}}`

* `{{location}}/{{name}}`

When using the `pulumi import` command, PrivateCloud can be imported using one of the formats above. For example:

```sh
$ pulumi import gcp:vmwareengine/privateCloud:PrivateCloud default projects/{{project}}/locations/{{location}}/privateClouds/{{name}}
```

```sh
$ pulumi import gcp:vmwareengine/privateCloud:PrivateCloud default {{project}}/{{location}}/{{name}}
```

```sh
$ pulumi import gcp:vmwareengine/privateCloud:PrivateCloud default {{location}}/{{name}}
```

ˆ
deletionDelayHoursB ŸThe number of hours to delay this request. You can set this value to an hour between 0 to 8, where setting it to 0
starts the deletion request immediately. If no value is set, a default value is set at the API Level.
G
descriptionB" 2User-provided description for this private cloud.
C
location" 3The location where the PrivateCloud should reside.
û
managementCluster:}
{
vmwareenginePrivateCloudManagementClusterLgcp:vmwareengine/PrivateCloudManagementCluster:PrivateCloudManagementClusteráThe management cluster for this private cloud. This used for creating and managing the default cluster.
Structure is documented below.
*
nameB" The ID of the PrivateCloud.
˙
networkConfigs:q
o
vmwareenginePrivateCloudNetworkConfigDgcp:vmwareengine/PrivateCloudNetworkConfig:PrivateCloudNetworkConfigtNetwork configuration in the consumer project with which the peering has to be done.
Structure is documented below.

projectB" £
sendDeletionDelayHoursIfZeroB
 ¸While set true, deletion_delay_hours value will be sent in the request even for zero value of the field. This field is
only useful for setting 0 value to the deletion_delay_hours field. It can be used both alone and together with
deletion_delay_hours.
l
typeB" ^Initial type of the private cloud. Possible values: ["STANDARD", "TIME_LIMITED", "STRETCHED"]
"ˆ
deletionDelayHoursB ŸThe number of hours to delay this request. You can set this value to an hour between 0 to 8, where setting it to 0
starts the deletion request immediately. If no value is set, a default value is set at the API Level.
"G
descriptionB" 2User-provided description for this private cloud.
"Æ
hcxesW*U:S
Q
vmwareenginePrivateCloudHcx0gcp:vmwareengine/PrivateCloudHcx:PrivateCloudHcxLDetails about a HCX Cloud Manager appliance.
Structure is documented below.
"C
location" 3The location where the PrivateCloud should reside.
"û
managementCluster:}
{
vmwareenginePrivateCloudManagementClusterLgcp:vmwareengine/PrivateCloudManagementCluster:PrivateCloudManagementClusteráThe management cluster for this private cloud. This used for creating and managing the default cluster.
Structure is documented below.
"(
name" The ID of the PrivateCloud.
"˙
networkConfigs:q
o
vmwareenginePrivateCloudNetworkConfigDgcp:vmwareengine/PrivateCloudNetworkConfig:PrivateCloudNetworkConfigtNetwork configuration in the consumer project with which the peering has to be done.
Structure is documented below.
"®
nsxesW*U:S
Q
vmwareenginePrivateCloudNsx0gcp:vmwareengine/PrivateCloudNsx:PrivateCloudNsxFDetails about a NSX Manager appliance.
Structure is documented below.
"
project" "£
sendDeletionDelayHoursIfZeroB
 ¸While set true, deletion_delay_hours value will be sent in the request even for zero value of the field. This field is
only useful for setting 0 value to the deletion_delay_hours field. It can be used both alone and together with
deletion_delay_hours.
"P
state" CState of the appliance.
Possible values are: `ACTIVE`, `CREATING`.
"l
typeB" ^Initial type of the private cloud. Possible values: ["STANDARD", "TIME_LIMITED", "STRETCHED"]
"@
uid" 5System-generated unique identifier for the resource.
"≈
vcentersc*a:_
]
vmwareenginePrivateCloudVcenter8gcp:vmwareengine/PrivateCloudVcenter:PrivateCloudVcenterTDetails about a vCenter Server management appliance.
Structure is documented below.
*æT
6
vmwareengineSubnetgcp:vmwareengine/subnet:SubnetäBSubnet in a private cloud. A Private Cloud contains two types of subnets: `management` subnets (such as vMotion) that
are read-only,and `userDefined`, which can also be updated. This resource should be used to read and update `userDefined`
subnets. To read `management` subnets, please utilize the subnet data source.


To get more information about Subnet, see:

* [API documentation](https://cloud.google.com/vmware-engine/docs/reference/rest/v1/projects.locations.privateClouds.subnets)

## Example Usage

### Vmware Engine Subnet User Defined


<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as gcp from "@pulumi/gcp";

const subnet_nw = new gcp.vmwareengine.Network("subnet-nw", {
    name: "pc-nw",
    location: "global",
    type: "STANDARD",
    description: "PC network description.",
});
const subnet_pc = new gcp.vmwareengine.PrivateCloud("subnet-pc", {
    location: "us-west1-a",
    name: "sample-pc",
    description: "Sample test PC.",
    networkConfig: {
        managementCidr: "192.168.50.0/24",
        vmwareEngineNetwork: subnet_nw.id,
    },
    managementCluster: {
        clusterId: "sample-mgmt-cluster",
        nodeTypeConfigs: [{
            nodeTypeId: "standard-72",
            nodeCount: 3,
        }],
    },
});
const vmw_engine_subnet = new gcp.vmwareengine.Subnet("vmw-engine-subnet", {
    name: "service-1",
    parent: subnet_pc.id,
    ipCidrRange: "192.168.100.0/26",
});
```
```python
import pulumi
import pulumi_gcp as gcp

subnet_nw = gcp.vmwareengine.Network("subnet-nw",
    name="pc-nw",
    location="global",
    type="STANDARD",
    description="PC network description.")
subnet_pc = gcp.vmwareengine.PrivateCloud("subnet-pc",
    location="us-west1-a",
    name="sample-pc",
    description="Sample test PC.",
    network_config={
        "management_cidr": "192.168.50.0/24",
        "vmware_engine_network": subnet_nw.id,
    },
    management_cluster={
        "cluster_id": "sample-mgmt-cluster",
        "node_type_configs": [{
            "node_type_id": "standard-72",
            "node_count": 3,
        }],
    })
vmw_engine_subnet = gcp.vmwareengine.Subnet("vmw-engine-subnet",
    name="service-1",
    parent=subnet_pc.id,
    ip_cidr_range="192.168.100.0/26")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Gcp = Pulumi.Gcp;

return await Deployment.RunAsync(() => 
{
    var subnet_nw = new Gcp.VMwareEngine.Network("subnet-nw", new()
    {
        Name = "pc-nw",
        Location = "global",
        Type = "STANDARD",
        Description = "PC network description.",
    });

    var subnet_pc = new Gcp.VMwareEngine.PrivateCloud("subnet-pc", new()
    {
        Location = "us-west1-a",
        Name = "sample-pc",
        Description = "Sample test PC.",
        NetworkConfig = new Gcp.VMwareEngine.Inputs.PrivateCloudNetworkConfigArgs
        {
            ManagementCidr = "192.168.50.0/24",
            VmwareEngineNetwork = subnet_nw.Id,
        },
        ManagementCluster = new Gcp.VMwareEngine.Inputs.PrivateCloudManagementClusterArgs
        {
            ClusterId = "sample-mgmt-cluster",
            NodeTypeConfigs = new[]
            {
                new Gcp.VMwareEngine.Inputs.PrivateCloudManagementClusterNodeTypeConfigArgs
                {
                    NodeTypeId = "standard-72",
                    NodeCount = 3,
                },
            },
        },
    });

    var vmw_engine_subnet = new Gcp.VMwareEngine.Subnet("vmw-engine-subnet", new()
    {
        Name = "service-1",
        Parent = subnet_pc.Id,
        IpCidrRange = "192.168.100.0/26",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-gcp/sdk/v8/go/gcp/vmwareengine"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := vmwareengine.NewNetwork(ctx, "subnet-nw", &vmwareengine.NetworkArgs{
			Name:        pulumi.String("pc-nw"),
			Location:    pulumi.String("global"),
			Type:        pulumi.String("STANDARD"),
			Description: pulumi.String("PC network description."),
		})
		if err != nil {
			return err
		}
		_, err = vmwareengine.NewPrivateCloud(ctx, "subnet-pc", &vmwareengine.PrivateCloudArgs{
			Location:    pulumi.String("us-west1-a"),
			Name:        pulumi.String("sample-pc"),
			Description: pulumi.String("Sample test PC."),
			NetworkConfig: &vmwareengine.PrivateCloudNetworkConfigArgs{
				ManagementCidr:      pulumi.String("192.168.50.0/24"),
				VmwareEngineNetwork: subnet_nw.ID(),
			},
			ManagementCluster: &vmwareengine.PrivateCloudManagementClusterArgs{
				ClusterId: pulumi.String("sample-mgmt-cluster"),
				NodeTypeConfigs: vmwareengine.PrivateCloudManagementClusterNodeTypeConfigArray{
					&vmwareengine.PrivateCloudManagementClusterNodeTypeConfigArgs{
						NodeTypeId: pulumi.String("standard-72"),
						NodeCount:  pulumi.Int(3),
					},
				},
			},
		})
		if err != nil {
			return err
		}
		_, err = vmwareengine.NewSubnet(ctx, "vmw-engine-subnet", &vmwareengine.SubnetArgs{
			Name:        pulumi.String("service-1"),
			Parent:      subnet_pc.ID(),
			IpCidrRange: pulumi.String("192.168.100.0/26"),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.gcp.vmwareengine.Network;
import com.pulumi.gcp.vmwareengine.NetworkArgs;
import com.pulumi.gcp.vmwareengine.PrivateCloud;
import com.pulumi.gcp.vmwareengine.PrivateCloudArgs;
import com.pulumi.gcp.vmwareengine.inputs.PrivateCloudNetworkConfigArgs;
import com.pulumi.gcp.vmwareengine.inputs.PrivateCloudManagementClusterArgs;
import com.pulumi.gcp.vmwareengine.Subnet;
import com.pulumi.gcp.vmwareengine.SubnetArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var subnet_nw = new Network("subnet-nw", NetworkArgs.builder()
            .name("pc-nw")
            .location("global")
            .type("STANDARD")
            .description("PC network description.")
            .build());

        var subnet_pc = new PrivateCloud("subnet-pc", PrivateCloudArgs.builder()
            .location("us-west1-a")
            .name("sample-pc")
            .description("Sample test PC.")
            .networkConfig(PrivateCloudNetworkConfigArgs.builder()
                .managementCidr("192.168.50.0/24")
                .vmwareEngineNetwork(subnet_nw.id())
                .build())
            .managementCluster(PrivateCloudManagementClusterArgs.builder()
                .clusterId("sample-mgmt-cluster")
                .nodeTypeConfigs(PrivateCloudManagementClusterNodeTypeConfigArgs.builder()
                    .nodeTypeId("standard-72")
                    .nodeCount(3)
                    .build())
                .build())
            .build());

        var vmw_engine_subnet = new Subnet("vmw-engine-subnet", SubnetArgs.builder()
            .name("service-1")
            .parent(subnet_pc.id())
            .ipCidrRange("192.168.100.0/26")
            .build());

    }
}
```
```yaml
resources:
  subnet-nw:
    type: gcp:vmwareengine:Network
    properties:
      name: pc-nw
      location: global
      type: STANDARD
      description: PC network description.
  subnet-pc:
    type: gcp:vmwareengine:PrivateCloud
    properties:
      location: us-west1-a
      name: sample-pc
      description: Sample test PC.
      networkConfig:
        managementCidr: 192.168.50.0/24
        vmwareEngineNetwork: ${["subnet-nw"].id}
      managementCluster:
        clusterId: sample-mgmt-cluster
        nodeTypeConfigs:
          - nodeTypeId: standard-72
            nodeCount: 3
  vmw-engine-subnet:
    type: gcp:vmwareengine:Subnet
    properties:
      name: service-1
      parent: ${["subnet-pc"].id}
      ipCidrRange: 192.168.100.0/26
```
<!--End PulumiCodeChooser -->

## Import

Subnet can be imported using any of these accepted formats:

* `{{parent}}/subnets/{{name}}`

When using the `pulumi import` command, Subnet can be imported using one of the formats above. For example:

```sh
$ pulumi import gcp:vmwareengine/subnet:Subnet default {{parent}}/subnets/{{name}}
```

F
ipCidrRange" 3The IP address range of the subnet in CIDR format.
î
nameB" ÖThe ID of the subnet. For userDefined subnets, this name should be in the format of "service-n",
where n ranges from 1 to 5.


- - -
ï
parent" ÜThe resource name of the private cloud to create a new subnet in.
Resource names are schemeless URIs that follow the conventions in https://cloud.google.com/apis/design/resource_names.
For example: projects/my-project/locations/us-west1-a/privateClouds/my-cloud
"·

createTime" ŒCreation time of this resource.
A timestamp in RFC3339 UTC "Zulu" format, with nanosecond resolution and
up to nine fractional digits. Examples: "2014-10-02T15:01:23Z" and "2014-10-02T15:01:23.045123456Z".
"∑
dhcpAddressRangesl*j:h
f
vmwareengineSubnetDhcpAddressRange>gcp:vmwareengine/SubnetDhcpAddressRange:SubnetDhcpAddressRange4DHCP address ranges.
Structure is documented below.
"a
	gatewayId" PThe canonical identifier of the logical router that this subnet is attached to.
"m
	gatewayIp" \The IP address of the gateway of this subnet. Must fall within the IP prefix defined above.
"F
ipCidrRange" 3The IP address range of the subnet in CIDR format.
"í
name" ÖThe ID of the subnet. For userDefined subnets, this name should be in the format of "service-n",
where n ranges from 1 to 5.


- - -
"ï
parent" ÜThe resource name of the private cloud to create a new subnet in.
Resource names are schemeless URIs that follow the conventions in https://cloud.google.com/apis/design/resource_names.
For example: projects/my-project/locations/us-west1-a/privateClouds/my-cloud
"ﬂ
standardConfig
 »Whether the NSX-T configuration in the backend follows the standard configuration supported by Google Cloud.
If false, the subnet cannot be modified through Google Cloud, only through NSX-T directly.
""
state" State of the subnet.
"$
type" The type of the subnet.
"@
uid" 5System-generated unique identifier for the resource.
"Â

updateTime" “Last updated time of this resource.
A timestamp in RFC3339 UTC "Zulu" format, with nanosecond resolution and up to nine
fractional digits. Examples: "2014-10-02T15:01:23Z" and "2014-10-02T15:01:23.045123456Z".
"E
vlanId 7VLAN ID of the VLAN on which the subnet is configured.
*‡^
9
	vpcaccess	Connector!gcp:vpcaccess/connector:Connectorà=Serverless VPC Access connector resource.


To get more information about Connector, see:

* [API documentation](https://cloud.google.com/vpc/docs/reference/vpcaccess/rest/v1/projects.locations.connectors)
* How-to Guides
    * [Configuring Serverless VPC Access](https://cloud.google.com/vpc/docs/configure-serverless-vpc-access)

## Example Usage

### Vpc Access Connector


<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as gcp from "@pulumi/gcp";

const connector = new gcp.vpcaccess.Connector("connector", {
    name: "vpc-con",
    ipCidrRange: "10.8.0.0/28",
    network: "default",
    minInstances: 2,
    maxInstances: 3,
});
```
```python
import pulumi
import pulumi_gcp as gcp

connector = gcp.vpcaccess.Connector("connector",
    name="vpc-con",
    ip_cidr_range="10.8.0.0/28",
    network="default",
    min_instances=2,
    max_instances=3)
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Gcp = Pulumi.Gcp;

return await Deployment.RunAsync(() => 
{
    var connector = new Gcp.VpcAccess.Connector("connector", new()
    {
        Name = "vpc-con",
        IpCidrRange = "10.8.0.0/28",
        Network = "default",
        MinInstances = 2,
        MaxInstances = 3,
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-gcp/sdk/v8/go/gcp/vpcaccess"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := vpcaccess.NewConnector(ctx, "connector", &vpcaccess.ConnectorArgs{
			Name:         pulumi.String("vpc-con"),
			IpCidrRange:  pulumi.String("10.8.0.0/28"),
			Network:      pulumi.String("default"),
			MinInstances: pulumi.Int(2),
			MaxInstances: pulumi.Int(3),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.gcp.vpcaccess.Connector;
import com.pulumi.gcp.vpcaccess.ConnectorArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var connector = new Connector("connector", ConnectorArgs.builder()
            .name("vpc-con")
            .ipCidrRange("10.8.0.0/28")
            .network("default")
            .minInstances(2)
            .maxInstances(3)
            .build());

    }
}
```
```yaml
resources:
  connector:
    type: gcp:vpcaccess:Connector
    properties:
      name: vpc-con
      ipCidrRange: 10.8.0.0/28
      network: default
      minInstances: 2
      maxInstances: 3
```
<!--End PulumiCodeChooser -->
### Vpc Access Connector Shared Vpc


<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as gcp from "@pulumi/gcp";

const customTest = new gcp.compute.Subnetwork("custom_test", {
    name: "vpc-con",
    ipCidrRange: "10.2.0.0/28",
    region: "us-central1",
    network: "default",
});
const connector = new gcp.vpcaccess.Connector("connector", {
    name: "vpc-con",
    subnet: {
        name: customTest.name,
    },
    machineType: "e2-standard-4",
    minInstances: 2,
    maxInstances: 3,
});
```
```python
import pulumi
import pulumi_gcp as gcp

custom_test = gcp.compute.Subnetwork("custom_test",
    name="vpc-con",
    ip_cidr_range="10.2.0.0/28",
    region="us-central1",
    network="default")
connector = gcp.vpcaccess.Connector("connector",
    name="vpc-con",
    subnet={
        "name": custom_test.name,
    },
    machine_type="e2-standard-4",
    min_instances=2,
    max_instances=3)
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Gcp = Pulumi.Gcp;

return await Deployment.RunAsync(() => 
{
    var customTest = new Gcp.Compute.Subnetwork("custom_test", new()
    {
        Name = "vpc-con",
        IpCidrRange = "10.2.0.0/28",
        Region = "us-central1",
        Network = "default",
    });

    var connector = new Gcp.VpcAccess.Connector("connector", new()
    {
        Name = "vpc-con",
        Subnet = new Gcp.VpcAccess.Inputs.ConnectorSubnetArgs
        {
            Name = customTest.Name,
        },
        MachineType = "e2-standard-4",
        MinInstances = 2,
        MaxInstances = 3,
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-gcp/sdk/v8/go/gcp/compute"
	"github.com/pulumi/pulumi-gcp/sdk/v8/go/gcp/vpcaccess"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		customTest, err := compute.NewSubnetwork(ctx, "custom_test", &compute.SubnetworkArgs{
			Name:        pulumi.String("vpc-con"),
			IpCidrRange: pulumi.String("10.2.0.0/28"),
			Region:      pulumi.String("us-central1"),
			Network:     pulumi.String("default"),
		})
		if err != nil {
			return err
		}
		_, err = vpcaccess.NewConnector(ctx, "connector", &vpcaccess.ConnectorArgs{
			Name: pulumi.String("vpc-con"),
			Subnet: &vpcaccess.ConnectorSubnetArgs{
				Name: customTest.Name,
			},
			MachineType:  pulumi.String("e2-standard-4"),
			MinInstances: pulumi.Int(2),
			MaxInstances: pulumi.Int(3),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.gcp.compute.Subnetwork;
import com.pulumi.gcp.compute.SubnetworkArgs;
import com.pulumi.gcp.vpcaccess.Connector;
import com.pulumi.gcp.vpcaccess.ConnectorArgs;
import com.pulumi.gcp.vpcaccess.inputs.ConnectorSubnetArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var customTest = new Subnetwork("customTest", SubnetworkArgs.builder()
            .name("vpc-con")
            .ipCidrRange("10.2.0.0/28")
            .region("us-central1")
            .network("default")
            .build());

        var connector = new Connector("connector", ConnectorArgs.builder()
            .name("vpc-con")
            .subnet(ConnectorSubnetArgs.builder()
                .name(customTest.name())
                .build())
            .machineType("e2-standard-4")
            .minInstances(2)
            .maxInstances(3)
            .build());

    }
}
```
```yaml
resources:
  connector:
    type: gcp:vpcaccess:Connector
    properties:
      name: vpc-con
      subnet:
        name: ${customTest.name}
      machineType: e2-standard-4
      minInstances: 2
      maxInstances: 3
  customTest:
    type: gcp:compute:Subnetwork
    name: custom_test
    properties:
      name: vpc-con
      ipCidrRange: 10.2.0.0/28
      region: us-central1
      network: default
```
<!--End PulumiCodeChooser -->

## Import

Connector can be imported using any of these accepted formats:

* `projects/{{project}}/locations/{{region}}/connectors/{{name}}`

* `{{project}}/{{region}}/{{name}}`

* `{{region}}/{{name}}`

* `{{name}}`

When using the `pulumi import` command, Connector can be imported using one of the formats above. For example:

```sh
$ pulumi import gcp:vpcaccess/connector:Connector default projects/{{project}}/locations/{{region}}/connectors/{{name}}
```

```sh
$ pulumi import gcp:vpcaccess/connector:Connector default {{project}}/{{region}}/{{name}}
```

```sh
$ pulumi import gcp:vpcaccess/connector:Connector default {{region}}/{{name}}
```

```sh
$ pulumi import gcp:vpcaccess/connector:Connector default {{name}}
```

o
ipCidrRangeB" ZThe range of internal addresses that follows RFC 4632 notation. Example: `10.132.0.0/28`.
[
machineTypeB" FMachine type of VM Instance underlying connector. Default is e2-micro
∆
maxInstancesB ØMaximum value of instances in autoscaling group underlying the connector. Value must be between 3 and 10, inclusive. Must be
higher than the value specified by min_instances.
º
maxThroughputB §Maximum throughput of the connector in Mbps, must be greater than `min_throughput`. Default is 300. Refers to the expected throughput
when using an e2-micro machine type. Value must be a multiple of 100 from 300 through 1000. Must be higher than the value specified by
min_throughput. Only one of `max_throughput` and `max_instances` can be specified. The use of max_throughput is discouraged in favor of max_instances.
ƒ
minInstancesB ≠Minimum value of instances in autoscaling group underlying the connector. Value must be between 2 and 9, inclusive. Must be
lower than the value specified by max_instances.
õ
minThroughputB ÉMinimum throughput of the connector in Mbps. Default and min is 200. Refers to the expected throughput when using an e2-micro machine type.
Value must be a multiple of 100 from 200 through 900. Must be lower than the value specified by max_throughput.
Only one of `min_throughput` and `min_instances` can be specified. The use of min_throughput is discouraged in favor of min_instances.
D
nameB" 6The name of the resource (Max 25 characters).


- - -
[
networkB" JName or self_link of the VPC network. Required if `ip_cidr_range` is set.
{
projectB" jThe ID of the project in which the resource belongs.
If it is not provided, the provider project is used.
s
regionB" cRegion where the VPC Access connector resides. If it is not provided, the provider region is used.
ß
subnetQBO:M
K
	vpcaccessConnectorSubnet-gcp:vpcaccess/ConnectorSubnet:ConnectorSubnetJThe subnet in which to house the connector
Structure is documented below.
"A
connectedProjects*" &List of projects using the connector.
"o
ipCidrRangeB" ZThe range of internal addresses that follows RFC 4632 notation. Example: `10.132.0.0/28`.
"[
machineTypeB" FMachine type of VM Instance underlying connector. Default is e2-micro
"ƒ
maxInstances ØMaximum value of instances in autoscaling group underlying the connector. Value must be between 3 and 10, inclusive. Must be
higher than the value specified by min_instances.
"∫
maxThroughput §Maximum throughput of the connector in Mbps, must be greater than `min_throughput`. Default is 300. Refers to the expected throughput
when using an e2-micro machine type. Value must be a multiple of 100 from 300 through 1000. Must be higher than the value specified by
min_throughput. Only one of `max_throughput` and `max_instances` can be specified. The use of max_throughput is discouraged in favor of max_instances.
"¬
minInstances ≠Minimum value of instances in autoscaling group underlying the connector. Value must be between 2 and 9, inclusive. Must be
lower than the value specified by max_instances.
"ô
minThroughput ÉMinimum throughput of the connector in Mbps. Default and min is 200. Refers to the expected throughput when using an e2-micro machine type.
Value must be a multiple of 100 from 200 through 900. Must be lower than the value specified by max_throughput.
Only one of `min_throughput` and `min_instances` can be specified. The use of min_throughput is discouraged in favor of min_instances.
"B
name" 6The name of the resource (Max 25 characters).


- - -
"Y
network" JName or self_link of the VPC network. Required if `ip_cidr_range` is set.
"y
project" jThe ID of the project in which the resource belongs.
If it is not provided, the provider project is used.
"q
region" cRegion where the VPC Access connector resides. If it is not provided, the provider region is used.
"?
selfLink" /The fully qualified name of this VPC connector
"0
state" #State of the VPC access connector.
"ß
subnetQBO:M
K
	vpcaccessConnectorSubnet-gcp:vpcaccess/ConnectorSubnet:ConnectorSubnetJThe subnet in which to house the connector
Structure is documented below.
*Íµ
6
	workbenchInstancegcp:workbench/instance:InstanceçêA Workbench instance.



## Example Usage

### Workbench Instance Basic


<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as gcp from "@pulumi/gcp";

const instance = new gcp.workbench.Instance("instance", {
    name: "workbench-instance",
    location: "us-west1-a",
});
```
```python
import pulumi
import pulumi_gcp as gcp

instance = gcp.workbench.Instance("instance",
    name="workbench-instance",
    location="us-west1-a")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Gcp = Pulumi.Gcp;

return await Deployment.RunAsync(() => 
{
    var instance = new Gcp.Workbench.Instance("instance", new()
    {
        Name = "workbench-instance",
        Location = "us-west1-a",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-gcp/sdk/v8/go/gcp/workbench"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := workbench.NewInstance(ctx, "instance", &workbench.InstanceArgs{
			Name:     pulumi.String("workbench-instance"),
			Location: pulumi.String("us-west1-a"),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.gcp.workbench.Instance;
import com.pulumi.gcp.workbench.InstanceArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var instance = new Instance("instance", InstanceArgs.builder()
            .name("workbench-instance")
            .location("us-west1-a")
            .build());

    }
}
```
```yaml
resources:
  instance:
    type: gcp:workbench:Instance
    properties:
      name: workbench-instance
      location: us-west1-a
```
<!--End PulumiCodeChooser -->
### Workbench Instance Basic Container


<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as gcp from "@pulumi/gcp";

const instance = new gcp.workbench.Instance("instance", {
    name: "workbench-instance",
    location: "us-west1-a",
    gceSetup: {
        containerImage: {
            repository: "us-docker.pkg.dev/deeplearning-platform-release/gcr.io/base-cu113.py310",
            tag: "latest",
        },
    },
});
```
```python
import pulumi
import pulumi_gcp as gcp

instance = gcp.workbench.Instance("instance",
    name="workbench-instance",
    location="us-west1-a",
    gce_setup={
        "container_image": {
            "repository": "us-docker.pkg.dev/deeplearning-platform-release/gcr.io/base-cu113.py310",
            "tag": "latest",
        },
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Gcp = Pulumi.Gcp;

return await Deployment.RunAsync(() => 
{
    var instance = new Gcp.Workbench.Instance("instance", new()
    {
        Name = "workbench-instance",
        Location = "us-west1-a",
        GceSetup = new Gcp.Workbench.Inputs.InstanceGceSetupArgs
        {
            ContainerImage = new Gcp.Workbench.Inputs.InstanceGceSetupContainerImageArgs
            {
                Repository = "us-docker.pkg.dev/deeplearning-platform-release/gcr.io/base-cu113.py310",
                Tag = "latest",
            },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-gcp/sdk/v8/go/gcp/workbench"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := workbench.NewInstance(ctx, "instance", &workbench.InstanceArgs{
			Name:     pulumi.String("workbench-instance"),
			Location: pulumi.String("us-west1-a"),
			GceSetup: &workbench.InstanceGceSetupArgs{
				ContainerImage: &workbench.InstanceGceSetupContainerImageArgs{
					Repository: pulumi.String("us-docker.pkg.dev/deeplearning-platform-release/gcr.io/base-cu113.py310"),
					Tag:        pulumi.String("latest"),
				},
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.gcp.workbench.Instance;
import com.pulumi.gcp.workbench.InstanceArgs;
import com.pulumi.gcp.workbench.inputs.InstanceGceSetupArgs;
import com.pulumi.gcp.workbench.inputs.InstanceGceSetupContainerImageArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var instance = new Instance("instance", InstanceArgs.builder()
            .name("workbench-instance")
            .location("us-west1-a")
            .gceSetup(InstanceGceSetupArgs.builder()
                .containerImage(InstanceGceSetupContainerImageArgs.builder()
                    .repository("us-docker.pkg.dev/deeplearning-platform-release/gcr.io/base-cu113.py310")
                    .tag("latest")
                    .build())
                .build())
            .build());

    }
}
```
```yaml
resources:
  instance:
    type: gcp:workbench:Instance
    properties:
      name: workbench-instance
      location: us-west1-a
      gceSetup:
        containerImage:
          repository: us-docker.pkg.dev/deeplearning-platform-release/gcr.io/base-cu113.py310
          tag: latest
```
<!--End PulumiCodeChooser -->
### Workbench Instance Basic Gpu


<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as gcp from "@pulumi/gcp";

const instance = new gcp.workbench.Instance("instance", {
    name: "workbench-instance",
    location: "us-central1-a",
    gceSetup: {
        machineType: "n1-standard-1",
        acceleratorConfigs: [{
            type: "NVIDIA_TESLA_T4",
            coreCount: "1",
        }],
        vmImage: {
            project: "cloud-notebooks-managed",
            family: "workbench-instances",
        },
    },
});
```
```python
import pulumi
import pulumi_gcp as gcp

instance = gcp.workbench.Instance("instance",
    name="workbench-instance",
    location="us-central1-a",
    gce_setup={
        "machine_type": "n1-standard-1",
        "accelerator_configs": [{
            "type": "NVIDIA_TESLA_T4",
            "core_count": "1",
        }],
        "vm_image": {
            "project": "cloud-notebooks-managed",
            "family": "workbench-instances",
        },
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Gcp = Pulumi.Gcp;

return await Deployment.RunAsync(() => 
{
    var instance = new Gcp.Workbench.Instance("instance", new()
    {
        Name = "workbench-instance",
        Location = "us-central1-a",
        GceSetup = new Gcp.Workbench.Inputs.InstanceGceSetupArgs
        {
            MachineType = "n1-standard-1",
            AcceleratorConfigs = new[]
            {
                new Gcp.Workbench.Inputs.InstanceGceSetupAcceleratorConfigArgs
                {
                    Type = "NVIDIA_TESLA_T4",
                    CoreCount = "1",
                },
            },
            VmImage = new Gcp.Workbench.Inputs.InstanceGceSetupVmImageArgs
            {
                Project = "cloud-notebooks-managed",
                Family = "workbench-instances",
            },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-gcp/sdk/v8/go/gcp/workbench"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := workbench.NewInstance(ctx, "instance", &workbench.InstanceArgs{
			Name:     pulumi.String("workbench-instance"),
			Location: pulumi.String("us-central1-a"),
			GceSetup: &workbench.InstanceGceSetupArgs{
				MachineType: pulumi.String("n1-standard-1"),
				AcceleratorConfigs: workbench.InstanceGceSetupAcceleratorConfigArray{
					&workbench.InstanceGceSetupAcceleratorConfigArgs{
						Type:      pulumi.String("NVIDIA_TESLA_T4"),
						CoreCount: pulumi.String("1"),
					},
				},
				VmImage: &workbench.InstanceGceSetupVmImageArgs{
					Project: pulumi.String("cloud-notebooks-managed"),
					Family:  pulumi.String("workbench-instances"),
				},
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.gcp.workbench.Instance;
import com.pulumi.gcp.workbench.InstanceArgs;
import com.pulumi.gcp.workbench.inputs.InstanceGceSetupArgs;
import com.pulumi.gcp.workbench.inputs.InstanceGceSetupVmImageArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var instance = new Instance("instance", InstanceArgs.builder()
            .name("workbench-instance")
            .location("us-central1-a")
            .gceSetup(InstanceGceSetupArgs.builder()
                .machineType("n1-standard-1")
                .acceleratorConfigs(InstanceGceSetupAcceleratorConfigArgs.builder()
                    .type("NVIDIA_TESLA_T4")
                    .coreCount(1)
                    .build())
                .vmImage(InstanceGceSetupVmImageArgs.builder()
                    .project("cloud-notebooks-managed")
                    .family("workbench-instances")
                    .build())
                .build())
            .build());

    }
}
```
```yaml
resources:
  instance:
    type: gcp:workbench:Instance
    properties:
      name: workbench-instance
      location: us-central1-a
      gceSetup:
        machineType: n1-standard-1
        acceleratorConfigs:
          - type: NVIDIA_TESLA_T4
            coreCount: 1
        vmImage:
          project: cloud-notebooks-managed
          family: workbench-instances
```
<!--End PulumiCodeChooser -->
### Workbench Instance Labels Stopped


<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as gcp from "@pulumi/gcp";

const instance = new gcp.workbench.Instance("instance", {
    name: "workbench-instance",
    location: "us-central1-a",
    gceSetup: {
        machineType: "e2-standard-4",
        shieldedInstanceConfig: {
            enableSecureBoot: false,
            enableVtpm: false,
            enableIntegrityMonitoring: false,
        },
        serviceAccounts: [{
            email: "my@service-account.com",
        }],
        metadata: {
            terraform: "true",
        },
    },
    labels: {
        k: "val",
    },
    desiredState: "STOPPED",
});
```
```python
import pulumi
import pulumi_gcp as gcp

instance = gcp.workbench.Instance("instance",
    name="workbench-instance",
    location="us-central1-a",
    gce_setup={
        "machine_type": "e2-standard-4",
        "shielded_instance_config": {
            "enable_secure_boot": False,
            "enable_vtpm": False,
            "enable_integrity_monitoring": False,
        },
        "service_accounts": [{
            "email": "my@service-account.com",
        }],
        "metadata": {
            "terraform": "true",
        },
    },
    labels={
        "k": "val",
    },
    desired_state="STOPPED")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Gcp = Pulumi.Gcp;

return await Deployment.RunAsync(() => 
{
    var instance = new Gcp.Workbench.Instance("instance", new()
    {
        Name = "workbench-instance",
        Location = "us-central1-a",
        GceSetup = new Gcp.Workbench.Inputs.InstanceGceSetupArgs
        {
            MachineType = "e2-standard-4",
            ShieldedInstanceConfig = new Gcp.Workbench.Inputs.InstanceGceSetupShieldedInstanceConfigArgs
            {
                EnableSecureBoot = false,
                EnableVtpm = false,
                EnableIntegrityMonitoring = false,
            },
            ServiceAccounts = new[]
            {
                new Gcp.Workbench.Inputs.InstanceGceSetupServiceAccountArgs
                {
                    Email = "my@service-account.com",
                },
            },
            Metadata = 
            {
                { "terraform", "true" },
            },
        },
        Labels = 
        {
            { "k", "val" },
        },
        DesiredState = "STOPPED",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-gcp/sdk/v8/go/gcp/workbench"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := workbench.NewInstance(ctx, "instance", &workbench.InstanceArgs{
			Name:     pulumi.String("workbench-instance"),
			Location: pulumi.String("us-central1-a"),
			GceSetup: &workbench.InstanceGceSetupArgs{
				MachineType: pulumi.String("e2-standard-4"),
				ShieldedInstanceConfig: &workbench.InstanceGceSetupShieldedInstanceConfigArgs{
					EnableSecureBoot:          pulumi.Bool(false),
					EnableVtpm:                pulumi.Bool(false),
					EnableIntegrityMonitoring: pulumi.Bool(false),
				},
				ServiceAccounts: workbench.InstanceGceSetupServiceAccountArray{
					&workbench.InstanceGceSetupServiceAccountArgs{
						Email: pulumi.String("my@service-account.com"),
					},
				},
				Metadata: pulumi.StringMap{
					"terraform": pulumi.String("true"),
				},
			},
			Labels: pulumi.StringMap{
				"k": pulumi.String("val"),
			},
			DesiredState: pulumi.String("STOPPED"),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.gcp.workbench.Instance;
import com.pulumi.gcp.workbench.InstanceArgs;
import com.pulumi.gcp.workbench.inputs.InstanceGceSetupArgs;
import com.pulumi.gcp.workbench.inputs.InstanceGceSetupShieldedInstanceConfigArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var instance = new Instance("instance", InstanceArgs.builder()
            .name("workbench-instance")
            .location("us-central1-a")
            .gceSetup(InstanceGceSetupArgs.builder()
                .machineType("e2-standard-4")
                .shieldedInstanceConfig(InstanceGceSetupShieldedInstanceConfigArgs.builder()
                    .enableSecureBoot(false)
                    .enableVtpm(false)
                    .enableIntegrityMonitoring(false)
                    .build())
                .serviceAccounts(InstanceGceSetupServiceAccountArgs.builder()
                    .email("my@service-account.com")
                    .build())
                .metadata(Map.of("terraform", "true"))
                .build())
            .labels(Map.of("k", "val"))
            .desiredState("STOPPED")
            .build());

    }
}
```
```yaml
resources:
  instance:
    type: gcp:workbench:Instance
    properties:
      name: workbench-instance
      location: us-central1-a
      gceSetup:
        machineType: e2-standard-4
        shieldedInstanceConfig:
          enableSecureBoot: false
          enableVtpm: false
          enableIntegrityMonitoring: false
        serviceAccounts:
          - email: my@service-account.com
        metadata:
          terraform: 'true'
      labels:
        k: val
      desiredState: STOPPED
```
<!--End PulumiCodeChooser -->
### Workbench Instance Full


<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as gcp from "@pulumi/gcp";

const myNetwork = new gcp.compute.Network("my_network", {
    name: "wbi-test-default",
    autoCreateSubnetworks: false,
});
const mySubnetwork = new gcp.compute.Subnetwork("my_subnetwork", {
    name: "wbi-test-default",
    network: myNetwork.id,
    region: "us-central1",
    ipCidrRange: "10.0.1.0/24",
});
const static = new gcp.compute.Address("static", {name: "wbi-test-default"});
const actAsPermission = new gcp.serviceaccount.IAMBinding("act_as_permission", {
    serviceAccountId: "projects/my-project-name/serviceAccounts/my@service-account.com",
    role: "roles/iam.serviceAccountUser",
    members: ["user:example@example.com"],
});
const instance = new gcp.workbench.Instance("instance", {
    name: "workbench-instance",
    location: "us-central1-a",
    gceSetup: {
        machineType: "n1-standard-4",
        acceleratorConfigs: [{
            type: "NVIDIA_TESLA_T4",
            coreCount: "1",
        }],
        shieldedInstanceConfig: {
            enableSecureBoot: true,
            enableVtpm: true,
            enableIntegrityMonitoring: true,
        },
        disablePublicIp: false,
        serviceAccounts: [{
            email: "my@service-account.com",
        }],
        bootDisk: {
            diskSizeGb: "310",
            diskType: "PD_SSD",
            diskEncryption: "CMEK",
            kmsKey: "my-crypto-key",
        },
        dataDisks: {
            diskSizeGb: "330",
            diskType: "PD_SSD",
            diskEncryption: "CMEK",
            kmsKey: "my-crypto-key",
        },
        networkInterfaces: [{
            network: myNetwork.id,
            subnet: mySubnetwork.id,
            nicType: "GVNIC",
            accessConfigs: [{
                externalIp: static.address,
            }],
        }],
        metadata: {
            terraform: "true",
        },
        enableIpForwarding: true,
        tags: [
            "abc",
            "def",
        ],
    },
    disableProxyAccess: true,
    instanceOwners: ["example@example.com"],
    labels: {
        k: "val",
    },
    desiredState: "ACTIVE",
});
```
```python
import pulumi
import pulumi_gcp as gcp

my_network = gcp.compute.Network("my_network",
    name="wbi-test-default",
    auto_create_subnetworks=False)
my_subnetwork = gcp.compute.Subnetwork("my_subnetwork",
    name="wbi-test-default",
    network=my_network.id,
    region="us-central1",
    ip_cidr_range="10.0.1.0/24")
static = gcp.compute.Address("static", name="wbi-test-default")
act_as_permission = gcp.serviceaccount.IAMBinding("act_as_permission",
    service_account_id="projects/my-project-name/serviceAccounts/my@service-account.com",
    role="roles/iam.serviceAccountUser",
    members=["user:example@example.com"])
instance = gcp.workbench.Instance("instance",
    name="workbench-instance",
    location="us-central1-a",
    gce_setup={
        "machine_type": "n1-standard-4",
        "accelerator_configs": [{
            "type": "NVIDIA_TESLA_T4",
            "core_count": "1",
        }],
        "shielded_instance_config": {
            "enable_secure_boot": True,
            "enable_vtpm": True,
            "enable_integrity_monitoring": True,
        },
        "disable_public_ip": False,
        "service_accounts": [{
            "email": "my@service-account.com",
        }],
        "boot_disk": {
            "disk_size_gb": "310",
            "disk_type": "PD_SSD",
            "disk_encryption": "CMEK",
            "kms_key": "my-crypto-key",
        },
        "data_disks": {
            "disk_size_gb": "330",
            "disk_type": "PD_SSD",
            "disk_encryption": "CMEK",
            "kms_key": "my-crypto-key",
        },
        "network_interfaces": [{
            "network": my_network.id,
            "subnet": my_subnetwork.id,
            "nic_type": "GVNIC",
            "access_configs": [{
                "external_ip": static.address,
            }],
        }],
        "metadata": {
            "terraform": "true",
        },
        "enable_ip_forwarding": True,
        "tags": [
            "abc",
            "def",
        ],
    },
    disable_proxy_access=True,
    instance_owners=["example@example.com"],
    labels={
        "k": "val",
    },
    desired_state="ACTIVE")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Gcp = Pulumi.Gcp;

return await Deployment.RunAsync(() => 
{
    var myNetwork = new Gcp.Compute.Network("my_network", new()
    {
        Name = "wbi-test-default",
        AutoCreateSubnetworks = false,
    });

    var mySubnetwork = new Gcp.Compute.Subnetwork("my_subnetwork", new()
    {
        Name = "wbi-test-default",
        Network = myNetwork.Id,
        Region = "us-central1",
        IpCidrRange = "10.0.1.0/24",
    });

    var @static = new Gcp.Compute.Address("static", new()
    {
        Name = "wbi-test-default",
    });

    var actAsPermission = new Gcp.ServiceAccount.IAMBinding("act_as_permission", new()
    {
        ServiceAccountId = "projects/my-project-name/serviceAccounts/my@service-account.com",
        Role = "roles/iam.serviceAccountUser",
        Members = new[]
        {
            "user:example@example.com",
        },
    });

    var instance = new Gcp.Workbench.Instance("instance", new()
    {
        Name = "workbench-instance",
        Location = "us-central1-a",
        GceSetup = new Gcp.Workbench.Inputs.InstanceGceSetupArgs
        {
            MachineType = "n1-standard-4",
            AcceleratorConfigs = new[]
            {
                new Gcp.Workbench.Inputs.InstanceGceSetupAcceleratorConfigArgs
                {
                    Type = "NVIDIA_TESLA_T4",
                    CoreCount = "1",
                },
            },
            ShieldedInstanceConfig = new Gcp.Workbench.Inputs.InstanceGceSetupShieldedInstanceConfigArgs
            {
                EnableSecureBoot = true,
                EnableVtpm = true,
                EnableIntegrityMonitoring = true,
            },
            DisablePublicIp = false,
            ServiceAccounts = new[]
            {
                new Gcp.Workbench.Inputs.InstanceGceSetupServiceAccountArgs
                {
                    Email = "my@service-account.com",
                },
            },
            BootDisk = new Gcp.Workbench.Inputs.InstanceGceSetupBootDiskArgs
            {
                DiskSizeGb = "310",
                DiskType = "PD_SSD",
                DiskEncryption = "CMEK",
                KmsKey = "my-crypto-key",
            },
            DataDisks = new Gcp.Workbench.Inputs.InstanceGceSetupDataDisksArgs
            {
                DiskSizeGb = "330",
                DiskType = "PD_SSD",
                DiskEncryption = "CMEK",
                KmsKey = "my-crypto-key",
            },
            NetworkInterfaces = new[]
            {
                new Gcp.Workbench.Inputs.InstanceGceSetupNetworkInterfaceArgs
                {
                    Network = myNetwork.Id,
                    Subnet = mySubnetwork.Id,
                    NicType = "GVNIC",
                    AccessConfigs = new[]
                    {
                        new Gcp.Workbench.Inputs.InstanceGceSetupNetworkInterfaceAccessConfigArgs
                        {
                            ExternalIp = @static.IPAddress,
                        },
                    },
                },
            },
            Metadata = 
            {
                { "terraform", "true" },
            },
            EnableIpForwarding = true,
            Tags = new[]
            {
                "abc",
                "def",
            },
        },
        DisableProxyAccess = true,
        InstanceOwners = new[]
        {
            "example@example.com",
        },
        Labels = 
        {
            { "k", "val" },
        },
        DesiredState = "ACTIVE",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-gcp/sdk/v8/go/gcp/compute"
	"github.com/pulumi/pulumi-gcp/sdk/v8/go/gcp/serviceaccount"
	"github.com/pulumi/pulumi-gcp/sdk/v8/go/gcp/workbench"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		myNetwork, err := compute.NewNetwork(ctx, "my_network", &compute.NetworkArgs{
			Name:                  pulumi.String("wbi-test-default"),
			AutoCreateSubnetworks: pulumi.Bool(false),
		})
		if err != nil {
			return err
		}
		mySubnetwork, err := compute.NewSubnetwork(ctx, "my_subnetwork", &compute.SubnetworkArgs{
			Name:        pulumi.String("wbi-test-default"),
			Network:     myNetwork.ID(),
			Region:      pulumi.String("us-central1"),
			IpCidrRange: pulumi.String("10.0.1.0/24"),
		})
		if err != nil {
			return err
		}
		static, err := compute.NewAddress(ctx, "static", &compute.AddressArgs{
			Name: pulumi.String("wbi-test-default"),
		})
		if err != nil {
			return err
		}
		_, err = serviceaccount.NewIAMBinding(ctx, "act_as_permission", &serviceaccount.IAMBindingArgs{
			ServiceAccountId: pulumi.String("projects/my-project-name/serviceAccounts/my@service-account.com"),
			Role:             pulumi.String("roles/iam.serviceAccountUser"),
			Members: pulumi.StringArray{
				pulumi.String("user:example@example.com"),
			},
		})
		if err != nil {
			return err
		}
		_, err = workbench.NewInstance(ctx, "instance", &workbench.InstanceArgs{
			Name:     pulumi.String("workbench-instance"),
			Location: pulumi.String("us-central1-a"),
			GceSetup: &workbench.InstanceGceSetupArgs{
				MachineType: pulumi.String("n1-standard-4"),
				AcceleratorConfigs: workbench.InstanceGceSetupAcceleratorConfigArray{
					&workbench.InstanceGceSetupAcceleratorConfigArgs{
						Type:      pulumi.String("NVIDIA_TESLA_T4"),
						CoreCount: pulumi.String("1"),
					},
				},
				ShieldedInstanceConfig: &workbench.InstanceGceSetupShieldedInstanceConfigArgs{
					EnableSecureBoot:          pulumi.Bool(true),
					EnableVtpm:                pulumi.Bool(true),
					EnableIntegrityMonitoring: pulumi.Bool(true),
				},
				DisablePublicIp: pulumi.Bool(false),
				ServiceAccounts: workbench.InstanceGceSetupServiceAccountArray{
					&workbench.InstanceGceSetupServiceAccountArgs{
						Email: pulumi.String("my@service-account.com"),
					},
				},
				BootDisk: &workbench.InstanceGceSetupBootDiskArgs{
					DiskSizeGb:     pulumi.String("310"),
					DiskType:       pulumi.String("PD_SSD"),
					DiskEncryption: pulumi.String("CMEK"),
					KmsKey:         pulumi.String("my-crypto-key"),
				},
				DataDisks: &workbench.InstanceGceSetupDataDisksArgs{
					DiskSizeGb:     pulumi.String("330"),
					DiskType:       pulumi.String("PD_SSD"),
					DiskEncryption: pulumi.String("CMEK"),
					KmsKey:         pulumi.String("my-crypto-key"),
				},
				NetworkInterfaces: workbench.InstanceGceSetupNetworkInterfaceArray{
					&workbench.InstanceGceSetupNetworkInterfaceArgs{
						Network: myNetwork.ID(),
						Subnet:  mySubnetwork.ID(),
						NicType: pulumi.String("GVNIC"),
						AccessConfigs: workbench.InstanceGceSetupNetworkInterfaceAccessConfigArray{
							&workbench.InstanceGceSetupNetworkInterfaceAccessConfigArgs{
								ExternalIp: static.Address,
							},
						},
					},
				},
				Metadata: pulumi.StringMap{
					"terraform": pulumi.String("true"),
				},
				EnableIpForwarding: pulumi.Bool(true),
				Tags: pulumi.StringArray{
					pulumi.String("abc"),
					pulumi.String("def"),
				},
			},
			DisableProxyAccess: pulumi.Bool(true),
			InstanceOwners: pulumi.StringArray{
				pulumi.String("example@example.com"),
			},
			Labels: pulumi.StringMap{
				"k": pulumi.String("val"),
			},
			DesiredState: pulumi.String("ACTIVE"),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.gcp.compute.Network;
import com.pulumi.gcp.compute.NetworkArgs;
import com.pulumi.gcp.compute.Subnetwork;
import com.pulumi.gcp.compute.SubnetworkArgs;
import com.pulumi.gcp.compute.Address;
import com.pulumi.gcp.compute.AddressArgs;
import com.pulumi.gcp.serviceaccount.IAMBinding;
import com.pulumi.gcp.serviceaccount.IAMBindingArgs;
import com.pulumi.gcp.workbench.Instance;
import com.pulumi.gcp.workbench.InstanceArgs;
import com.pulumi.gcp.workbench.inputs.InstanceGceSetupArgs;
import com.pulumi.gcp.workbench.inputs.InstanceGceSetupShieldedInstanceConfigArgs;
import com.pulumi.gcp.workbench.inputs.InstanceGceSetupBootDiskArgs;
import com.pulumi.gcp.workbench.inputs.InstanceGceSetupDataDisksArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var myNetwork = new Network("myNetwork", NetworkArgs.builder()
            .name("wbi-test-default")
            .autoCreateSubnetworks(false)
            .build());

        var mySubnetwork = new Subnetwork("mySubnetwork", SubnetworkArgs.builder()
            .name("wbi-test-default")
            .network(myNetwork.id())
            .region("us-central1")
            .ipCidrRange("10.0.1.0/24")
            .build());

        var static_ = new Address("static", AddressArgs.builder()
            .name("wbi-test-default")
            .build());

        var actAsPermission = new IAMBinding("actAsPermission", IAMBindingArgs.builder()
            .serviceAccountId("projects/my-project-name/serviceAccounts/my@service-account.com")
            .role("roles/iam.serviceAccountUser")
            .members("user:example@example.com")
            .build());

        var instance = new Instance("instance", InstanceArgs.builder()
            .name("workbench-instance")
            .location("us-central1-a")
            .gceSetup(InstanceGceSetupArgs.builder()
                .machineType("n1-standard-4")
                .acceleratorConfigs(InstanceGceSetupAcceleratorConfigArgs.builder()
                    .type("NVIDIA_TESLA_T4")
                    .coreCount(1)
                    .build())
                .shieldedInstanceConfig(InstanceGceSetupShieldedInstanceConfigArgs.builder()
                    .enableSecureBoot(true)
                    .enableVtpm(true)
                    .enableIntegrityMonitoring(true)
                    .build())
                .disablePublicIp(false)
                .serviceAccounts(InstanceGceSetupServiceAccountArgs.builder()
                    .email("my@service-account.com")
                    .build())
                .bootDisk(InstanceGceSetupBootDiskArgs.builder()
                    .diskSizeGb(310)
                    .diskType("PD_SSD")
                    .diskEncryption("CMEK")
                    .kmsKey("my-crypto-key")
                    .build())
                .dataDisks(InstanceGceSetupDataDisksArgs.builder()
                    .diskSizeGb(330)
                    .diskType("PD_SSD")
                    .diskEncryption("CMEK")
                    .kmsKey("my-crypto-key")
                    .build())
                .networkInterfaces(InstanceGceSetupNetworkInterfaceArgs.builder()
                    .network(myNetwork.id())
                    .subnet(mySubnetwork.id())
                    .nicType("GVNIC")
                    .accessConfigs(InstanceGceSetupNetworkInterfaceAccessConfigArgs.builder()
                        .externalIp(static_.address())
                        .build())
                    .build())
                .metadata(Map.of("terraform", "true"))
                .enableIpForwarding(true)
                .tags(                
                    "abc",
                    "def")
                .build())
            .disableProxyAccess("true")
            .instanceOwners("example@example.com")
            .labels(Map.of("k", "val"))
            .desiredState("ACTIVE")
            .build());

    }
}
```
```yaml
resources:
  myNetwork:
    type: gcp:compute:Network
    name: my_network
    properties:
      name: wbi-test-default
      autoCreateSubnetworks: false
  mySubnetwork:
    type: gcp:compute:Subnetwork
    name: my_subnetwork
    properties:
      name: wbi-test-default
      network: ${myNetwork.id}
      region: us-central1
      ipCidrRange: 10.0.1.0/24
  static:
    type: gcp:compute:Address
    properties:
      name: wbi-test-default
  actAsPermission:
    type: gcp:serviceaccount:IAMBinding
    name: act_as_permission
    properties:
      serviceAccountId: projects/my-project-name/serviceAccounts/my@service-account.com
      role: roles/iam.serviceAccountUser
      members:
        - user:example@example.com
  instance:
    type: gcp:workbench:Instance
    properties:
      name: workbench-instance
      location: us-central1-a
      gceSetup:
        machineType: n1-standard-4
        acceleratorConfigs:
          - type: NVIDIA_TESLA_T4
            coreCount: 1
        shieldedInstanceConfig:
          enableSecureBoot: true
          enableVtpm: true
          enableIntegrityMonitoring: true
        disablePublicIp: false
        serviceAccounts:
          - email: my@service-account.com
        bootDisk:
          diskSizeGb: 310
          diskType: PD_SSD
          diskEncryption: CMEK
          kmsKey: my-crypto-key
        dataDisks:
          diskSizeGb: 330
          diskType: PD_SSD
          diskEncryption: CMEK
          kmsKey: my-crypto-key
        networkInterfaces:
          - network: ${myNetwork.id}
            subnet: ${mySubnetwork.id}
            nicType: GVNIC
            accessConfigs:
              - externalIp: ${static.address}
        metadata:
          terraform: 'true'
        enableIpForwarding: true
        tags:
          - abc
          - def
      disableProxyAccess: 'true'
      instanceOwners:
        - example@example.com
      labels:
        k: val
      desiredState: ACTIVE
```
<!--End PulumiCodeChooser -->

## Import

Instance can be imported using any of these accepted formats:

* `projects/{{project}}/locations/{{location}}/instances/{{name}}`

* `{{project}}/{{location}}/{{name}}`

* `{{location}}/{{name}}`

When using the `pulumi import` command, Instance can be imported using one of the formats above. For example:

```sh
$ pulumi import gcp:workbench/instance:Instance default projects/{{project}}/locations/{{location}}/instances/{{name}}
```

```sh
$ pulumi import gcp:workbench/instance:Instance default {{project}}/{{location}}/{{name}}
```

```sh
$ pulumi import gcp:workbench/instance:Instance default {{location}}/{{name}}
```

ï
desiredStateB" Desired state of the Workbench Instance. Set this field to `ACTIVE` to start the Instance, and `STOPPED` to stop the Instance.
h
disableProxyAccessB
 LOptional. If true, the workbench instance will not register with the proxy.
’
gceSetupTBR:P
N
	workbenchInstanceGceSetup/gcp:workbench/InstanceGceSetup:InstanceGceSetupsThe definition of how to configure a VM instance outside of Resources and Identity.
Structure is documented below.
G

instanceIdB" 3Required. User-defined unique ID of this instance.
≠
instanceOwnersB*" í'Optional. Input only. The owner of this instance after creation. Format:
`alias@example.com` Currently supports one owner only. If not specified, all of
the service account users of your VM instance''s service account can use the instance.
If specified, sets the access mode to `Single user`. For more details, see
https://cloud.google.com/vertex-ai/docs/workbench/instances/manage-access-jupyterlab'
¿
labelsB2" ≠Optional. Labels to apply to this instance. These can be later modified
by the UpdateInstance method.

**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
Please refer to the field `effective_labels` for all of the labels present on the resource.
M
location" =Part of `parent`. See documentation of `projectsId`.


- - -
Ä
nameB" rThe name of this workbench instance. Format: `projects/{project_id}/locations/{location}/instances/{instance_id}`
{
projectB" jThe ID of the project in which the resource belongs.
If it is not provided, the provider project is used.
"î

createTime" ÅAn RFC3339 timestamp in UTC time. This in the format of yyyy-MM-ddTHH:mm:ss.SSSZ.
The milliseconds portion (".SSS") is optional.
"_
creator" POutput only. Email address of entity that sent original CreateInstance request.
"ï
desiredStateB" Desired state of the Workbench Instance. Set this field to `ACTIVE` to start the Instance, and `STOPPED` to stop the Instance.
"h
disableProxyAccessB
 LOptional. If true, the workbench instance will not register with the proxy.
"¶
effectiveLabels2" åAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
"”
gceSetupR:P
N
	workbenchInstanceGceSetup/gcp:workbench/InstanceGceSetup:InstanceGceSetupsThe definition of how to configure a VM instance outside of Resources and Identity.
Structure is documented below.
"›
healthInfosZ*X:V
T
	workbenchInstanceHealthInfo3gcp:workbench/InstanceHealthInfo:InstanceHealthInfoÒ'Output only. Additional information about instance health. Example:
healthInfo": { "docker_proxy_agent_status": "1", "docker_status": "1", "jupyterlab_api_status":
"-1", "jupyterlab_status": "-1", "updated": "2020-10-18 09:40:03.573409" }'
"7
healthState" $Output only. Instance health_state.
"G

instanceIdB" 3Required. User-defined unique ID of this instance.
"≠
instanceOwnersB*" í'Optional. Input only. The owner of this instance after creation. Format:
`alias@example.com` Currently supports one owner only. If not specified, all of
the service account users of your VM instance''s service account can use the instance.
If specified, sets the access mode to `Single user`. For more details, see
https://cloud.google.com/vertex-ai/docs/workbench/instances/manage-access-jupyterlab'
"¿
labelsB2" ≠Optional. Labels to apply to this instance. These can be later modified
by the UpdateInstance method.

**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
Please refer to the field `effective_labels` for all of the labels present on the resource.
"M
location" =Part of `parent`. See documentation of `projectsId`.


- - -
"~
name" rThe name of this workbench instance. Format: `projects/{project_id}/locations/{location}/instances/{instance_id}`
"y
project" jThe ID of the project in which the resource belongs.
If it is not provided, the provider project is used.
"]
proxyUri" MOutput only. The proxy endpoint that is used to access the Jupyter notebook.
"É
pulumiLabels2" mThe combination of labels configured directly on the resource
and default labels configured on the provider.
"U
state" H(Output)
Output only. The state of this instance upgrade history entry.
"î

updateTime" ÅAn RFC3339 timestamp in UTC time. This in the format of yyyy-MM-ddTHH:mm:ss.SSSZ.
The milliseconds portion (".SSS") is optional.
"Œ
upgradeHistoriesf*d:b
`
	workbenchInstanceUpgradeHistory;gcp:workbench/InstanceUpgradeHistory:InstanceUpgradeHistoryROutput only. The upgrade history of this instance.
Structure is documented below.
*Ç
T
	workbenchInstanceIamBinding3gcp:workbench/instanceIamBinding:InstanceIamBindingÇ
	conditionuBs:q
o
	workbenchInstanceIamBindingConditionEgcp:workbench/InstanceIamBindingCondition:InstanceIamBindingCondition
locationB" 
members*" 
nameB" 
projectB" 

role" "Ç
	conditionuBs:q
o
	workbenchInstanceIamBindingConditionEgcp:workbench/InstanceIamBindingCondition:InstanceIamBindingCondition"

etag" "
location" "
members*" "

name" "
project" "

role" *Ò
Q
	workbenchInstanceIamMember1gcp:workbench/instanceIamMember:InstanceIamMember
	conditionrBp:n
l
	workbenchInstanceIamMemberConditionCgcp:workbench/InstanceIamMemberCondition:InstanceIamMemberCondition
locationB" 
member" 
nameB" 
projectB" 

role" "
	conditionrBp:n
l
	workbenchInstanceIamMemberConditionCgcp:workbench/InstanceIamMemberCondition:InstanceIamMemberCondition"

etag" "
location" "
member" "

name" "
project" "

role" *ﬂ
Q
	workbenchInstanceIamPolicy1gcp:workbench/instanceIamPolicy:InstanceIamPolicy
locationB" 
nameB" 

policyData" 
projectB" "

etag" "
location" "

name" "

policyData" "
project" *£~
6
	workflowsWorkflowgcp:workflows/workflow:WorkflowêTWorkflow program to be executed by Workflows.


To get more information about Workflow, see:

* [API documentation](https://cloud.google.com/workflows/docs/reference/rest/v1/projects.locations.workflows)
* How-to Guides
    * [Managing Workflows](https://cloud.google.com/workflows/docs/creating-updating-workflow)

## Example Usage

### Workflow Basic


<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as gcp from "@pulumi/gcp";

const testAccount = new gcp.serviceaccount.Account("test_account", {
    accountId: "my-account",
    displayName: "Test Service Account",
});
const example = new gcp.workflows.Workflow("example", {
    name: "workflow",
    region: "us-central1",
    description: "Magic",
    serviceAccount: testAccount.id,
    callLogLevel: "LOG_ERRORS_ONLY",
    labels: {
        env: "test",
    },
    userEnvVars: {
        url: "https://timeapi.io/api/Time/current/zone?timeZone=Europe/Amsterdam",
    },
    deletionProtection: false,
    sourceContents: `# This is a sample workflow. You can replace it with your source code.
#
# This workflow does the following:
# - reads current time and date information from an external API and stores
#   the response in currentTime variable
# - retrieves a list of Wikipedia articles related to the day of the week
#   from currentTime
# - returns the list of articles as an output of the workflow
#
# Note: In Terraform you need to escape the  or it will cause errors.

- getCurrentTime:
    call: http.get
    args:
        url: \${sys.get_env("url")}
    result: currentTime
- readWikipedia:
    call: http.get
    args:
        url: https://en.wikipedia.org/w/api.php
        query:
            action: opensearch
            search: \${currentTime.body.dayOfWeek}
    result: wikiResult
- returnOutput:
    return: \${wikiResult.body[1]}
`,
});
```
```python
import pulumi
import pulumi_gcp as gcp

test_account = gcp.serviceaccount.Account("test_account",
    account_id="my-account",
    display_name="Test Service Account")
example = gcp.workflows.Workflow("example",
    name="workflow",
    region="us-central1",
    description="Magic",
    service_account=test_account.id,
    call_log_level="LOG_ERRORS_ONLY",
    labels={
        "env": "test",
    },
    user_env_vars={
        "url": "https://timeapi.io/api/Time/current/zone?timeZone=Europe/Amsterdam",
    },
    deletion_protection=False,
    source_contents="""# This is a sample workflow. You can replace it with your source code.
#
# This workflow does the following:
# - reads current time and date information from an external API and stores
#   the response in currentTime variable
# - retrieves a list of Wikipedia articles related to the day of the week
#   from currentTime
# - returns the list of articles as an output of the workflow
#
# Note: In Terraform you need to escape the $$ or it will cause errors.

- getCurrentTime:
    call: http.get
    args:
        url: ${sys.get_env("url")}
    result: currentTime
- readWikipedia:
    call: http.get
    args:
        url: https://en.wikipedia.org/w/api.php
        query:
            action: opensearch
            search: ${currentTime.body.dayOfWeek}
    result: wikiResult
- returnOutput:
    return: ${wikiResult.body[1]}
""")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Gcp = Pulumi.Gcp;

return await Deployment.RunAsync(() => 
{
    var testAccount = new Gcp.ServiceAccount.Account("test_account", new()
    {
        AccountId = "my-account",
        DisplayName = "Test Service Account",
    });

    var example = new Gcp.Workflows.Workflow("example", new()
    {
        Name = "workflow",
        Region = "us-central1",
        Description = "Magic",
        ServiceAccount = testAccount.Id,
        CallLogLevel = "LOG_ERRORS_ONLY",
        Labels = 
        {
            { "env", "test" },
        },
        UserEnvVars = 
        {
            { "url", "https://timeapi.io/api/Time/current/zone?timeZone=Europe/Amsterdam" },
        },
        DeletionProtection = false,
        SourceContents = @"# This is a sample workflow. You can replace it with your source code.
#
# This workflow does the following:
# - reads current time and date information from an external API and stores
#   the response in currentTime variable
# - retrieves a list of Wikipedia articles related to the day of the week
#   from currentTime
# - returns the list of articles as an output of the workflow
#
# Note: In Terraform you need to escape the $$ or it will cause errors.

- getCurrentTime:
    call: http.get
    args:
        url: ${sys.get_env(""url"")}
    result: currentTime
- readWikipedia:
    call: http.get
    args:
        url: https://en.wikipedia.org/w/api.php
        query:
            action: opensearch
            search: ${currentTime.body.dayOfWeek}
    result: wikiResult
- returnOutput:
    return: ${wikiResult.body[1]}
",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-gcp/sdk/v8/go/gcp/serviceaccount"
	"github.com/pulumi/pulumi-gcp/sdk/v8/go/gcp/workflows"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		testAccount, err := serviceaccount.NewAccount(ctx, "test_account", &serviceaccount.AccountArgs{
			AccountId:   pulumi.String("my-account"),
			DisplayName: pulumi.String("Test Service Account"),
		})
		if err != nil {
			return err
		}
		_, err = workflows.NewWorkflow(ctx, "example", &workflows.WorkflowArgs{
			Name:           pulumi.String("workflow"),
			Region:         pulumi.String("us-central1"),
			Description:    pulumi.String("Magic"),
			ServiceAccount: testAccount.ID(),
			CallLogLevel:   pulumi.String("LOG_ERRORS_ONLY"),
			Labels: pulumi.StringMap{
				"env": pulumi.String("test"),
			},
			UserEnvVars: pulumi.StringMap{
				"url": pulumi.String("https://timeapi.io/api/Time/current/zone?timeZone=Europe/Amsterdam"),
			},
			DeletionProtection: pulumi.Bool(false),
			SourceContents: pulumi.String(`# This is a sample workflow. You can replace it with your source code.
#
# This workflow does the following:
# - reads current time and date information from an external API and stores
#   the response in currentTime variable
# - retrieves a list of Wikipedia articles related to the day of the week
#   from currentTime
# - returns the list of articles as an output of the workflow
#
# Note: In Terraform you need to escape the $$ or it will cause errors.

- getCurrentTime:
    call: http.get
    args:
        url: ${sys.get_env("url")}
    result: currentTime
- readWikipedia:
    call: http.get
    args:
        url: https://en.wikipedia.org/w/api.php
        query:
            action: opensearch
            search: ${currentTime.body.dayOfWeek}
    result: wikiResult
- returnOutput:
    return: ${wikiResult.body[1]}
`),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.gcp.serviceaccount.Account;
import com.pulumi.gcp.serviceaccount.AccountArgs;
import com.pulumi.gcp.workflows.Workflow;
import com.pulumi.gcp.workflows.WorkflowArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var testAccount = new Account("testAccount", AccountArgs.builder()
            .accountId("my-account")
            .displayName("Test Service Account")
            .build());

        var example = new Workflow("example", WorkflowArgs.builder()
            .name("workflow")
            .region("us-central1")
            .description("Magic")
            .serviceAccount(testAccount.id())
            .callLogLevel("LOG_ERRORS_ONLY")
            .labels(Map.of("env", "test"))
            .userEnvVars(Map.of("url", "https://timeapi.io/api/Time/current/zone?timeZone=Europe/Amsterdam"))
            .deletionProtection(false)
            .sourceContents("""
# This is a sample workflow. You can replace it with your source code.
#
# This workflow does the following:
# - reads current time and date information from an external API and stores
#   the response in currentTime variable
# - retrieves a list of Wikipedia articles related to the day of the week
#   from currentTime
# - returns the list of articles as an output of the workflow
#
# Note: In Terraform you need to escape the $$ or it will cause errors.

- getCurrentTime:
    call: http.get
    args:
        url: ${sys.get_env("url")}
    result: currentTime
- readWikipedia:
    call: http.get
    args:
        url: https://en.wikipedia.org/w/api.php
        query:
            action: opensearch
            search: ${currentTime.body.dayOfWeek}
    result: wikiResult
- returnOutput:
    return: ${wikiResult.body[1]}
            """)
            .build());

    }
}
```
```yaml
resources:
  testAccount:
    type: gcp:serviceaccount:Account
    name: test_account
    properties:
      accountId: my-account
      displayName: Test Service Account
  example:
    type: gcp:workflows:Workflow
    properties:
      name: workflow
      region: us-central1
      description: Magic
      serviceAccount: ${testAccount.id}
      callLogLevel: LOG_ERRORS_ONLY
      labels:
        env: test
      userEnvVars:
        url: https://timeapi.io/api/Time/current/zone?timeZone=Europe/Amsterdam
      deletionProtection: false
      sourceContents: |
        # This is a sample workflow. You can replace it with your source code.
        #
        # This workflow does the following:
        # - reads current time and date information from an external API and stores
        #   the response in currentTime variable
        # - retrieves a list of Wikipedia articles related to the day of the week
        #   from currentTime
        # - returns the list of articles as an output of the workflow
        #
        # Note: In Terraform you need to escape the $$ or it will cause errors.

        - getCurrentTime:
            call: http.get
            args:
                url: $${sys.get_env("url")}
            result: currentTime
        - readWikipedia:
            call: http.get
            args:
                url: https://en.wikipedia.org/w/api.php
                query:
                    action: opensearch
                    search: $${currentTime.body.dayOfWeek}
            result: wikiResult
        - returnOutput:
            return: $${wikiResult.body[1]}
```
<!--End PulumiCodeChooser -->

## Import

This resource does not support import.

—
callLogLevelB" ∫Describes the level of platform logging to apply to calls and call responses during
executions of this workflow. If both the workflow and the execution specify a logging level,
the execution level takes precedence.
Possible values are: `CALL_LOG_LEVEL_UNSPECIFIED`, `LOG_ALL_CALLS`, `LOG_ERRORS_ONLY`, `LOG_NONE`.
´
cryptoKeyNameB" ìThe KMS key used to encrypt workflow and execution data.
Format: projects/{project}/locations/{location}/keyRings/{keyRing}/cryptoKeys/{cryptoKey}

deletionProtectionB
 u
descriptionB" `Description of the workflow provided by the user. Must be at most 1000 unicode characters long.
ï
labelsB2" ÇA set of key/value label pairs to assign to this Workflow.

**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
Please refer to the field `effective_labels` for all of the labels present on the resource.
$
nameB" Name of the Workflow.
ô

namePrefixB" ÑCreates a unique name beginning with the
specified prefix. If this and name are unspecified, a random value is chosen for the name.
{
projectB" jThe ID of the project in which the resource belongs.
If it is not provided, the provider project is used.
,
regionB" The region of the workflow.
Û
serviceAccountB" ⁄Name of the service account associated with the latest workflow version. This service
account represents the identity of the workflow and determines what permissions the workflow has.
Format: projects/{project}/serviceAccounts/{account} or {account}.
Using - as a wildcard for the {project} or not providing one at all will infer the project from the account.
The {account} value can be the email address or the unique_id of the service account.
If not provided, workflow will use the project's default service account.
Modifying this field for an existing workflow results in a new workflow revision.
O
sourceContentsB" 7Workflow code to be executed. The size limit is 128KB.
˚
userEnvVarsB2" „User-defined environment variables associated with this workflow revision. This map has a maximum length of 20. Each string can take up to 4KiB. Keys cannot be empty strings and cannot start with ‚ÄúGOOGLE‚Äù or ‚ÄúWORKFLOWS".
"—
callLogLevelB" ∫Describes the level of platform logging to apply to calls and call responses during
executions of this workflow. If both the workflow and the execution specify a logging level,
the execution level takes precedence.
Possible values are: `CALL_LOG_LEVEL_UNSPECIFIED`, `LOG_ALL_CALLS`, `LOG_ERRORS_ONLY`, `LOG_NONE`.
"ù

createTime" äThe timestamp of when the workflow was created in RFC3339 UTC "Zulu" format, with nanosecond resolution and up to nine fractional digits.
"´
cryptoKeyNameB" ìThe KMS key used to encrypt workflow and execution data.
Format: projects/{project}/locations/{location}/keyRings/{keyRing}/cryptoKeys/{cryptoKey}
"
deletionProtectionB
 "s
description" `Description of the workflow provided by the user. Must be at most 1000 unicode characters long.
"¶
effectiveLabels2" åAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
"ï
labelsB2" ÇA set of key/value label pairs to assign to this Workflow.

**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
Please refer to the field `effective_labels` for all of the labels present on the resource.
""
name" Name of the Workflow.
"ó

namePrefix" ÑCreates a unique name beginning with the
specified prefix. If this and name are unspecified, a random value is chosen for the name.
"y
project" jThe ID of the project in which the resource belongs.
If it is not provided, the provider project is used.
"É
pulumiLabels2" mThe combination of labels configured directly on the resource
and default labels configured on the provider.
",
regionB" The region of the workflow.
"}

revisionId" kThe revision of the workflow. A new one is generated if the service account or source contents is changed.
"Ò
serviceAccount" ⁄Name of the service account associated with the latest workflow version. This service
account represents the identity of the workflow and determines what permissions the workflow has.
Format: projects/{project}/serviceAccounts/{account} or {account}.
Using - as a wildcard for the {project} or not providing one at all will infer the project from the account.
The {account} value can be the email address or the unique_id of the service account.
If not provided, workflow will use the project's default service account.
Modifying this field for an existing workflow results in a new workflow revision.
"O
sourceContentsB" 7Workflow code to be executed. The size limit is 128KB.
"/
state" "State of the workflow deployment.
"¢

updateTime" èThe timestamp of when the workflow was last updated in RFC3339 UTC "Zulu" format, with nanosecond resolution and up to nine fractional digits.
"˚
userEnvVarsB2" „User-defined environment variables associated with this workflow revision. This map has a maximum length of 20. Each string can take up to 4KiB. Keys cannot be empty strings and cannot start with ‚ÄúGOOGLE‚Äù or ‚ÄúWORKFLOWS".
*ÇÖ
E
workstationsWorkstation(gcp:workstations/workstation:Workstation√j## Example Usage

### Workstation Basic


<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as gcp from "@pulumi/gcp";

const _default = new gcp.compute.Network("default", {
    name: "workstation-cluster",
    autoCreateSubnetworks: false,
});
const defaultSubnetwork = new gcp.compute.Subnetwork("default", {
    name: "workstation-cluster",
    ipCidrRange: "10.0.0.0/24",
    region: "us-central1",
    network: _default.name,
});
const defaultWorkstationCluster = new gcp.workstations.WorkstationCluster("default", {
    workstationClusterId: "workstation-cluster",
    network: _default.id,
    subnetwork: defaultSubnetwork.id,
    location: "us-central1",
    labels: {
        label: "key",
    },
    annotations: {
        "label-one": "value-one",
    },
});
const defaultWorkstationConfig = new gcp.workstations.WorkstationConfig("default", {
    workstationConfigId: "workstation-config",
    workstationClusterId: defaultWorkstationCluster.workstationClusterId,
    location: "us-central1",
    host: {
        gceInstance: {
            machineType: "e2-standard-4",
            bootDiskSizeGb: 35,
            disablePublicIpAddresses: true,
        },
    },
});
const defaultWorkstation = new gcp.workstations.Workstation("default", {
    workstationId: "work-station",
    workstationConfigId: defaultWorkstationConfig.workstationConfigId,
    workstationClusterId: defaultWorkstationCluster.workstationClusterId,
    location: "us-central1",
    labels: {
        label: "key",
    },
    env: {
        name: "foo",
    },
    annotations: {
        "label-one": "value-one",
    },
});
```
```python
import pulumi
import pulumi_gcp as gcp

default = gcp.compute.Network("default",
    name="workstation-cluster",
    auto_create_subnetworks=False)
default_subnetwork = gcp.compute.Subnetwork("default",
    name="workstation-cluster",
    ip_cidr_range="10.0.0.0/24",
    region="us-central1",
    network=default.name)
default_workstation_cluster = gcp.workstations.WorkstationCluster("default",
    workstation_cluster_id="workstation-cluster",
    network=default.id,
    subnetwork=default_subnetwork.id,
    location="us-central1",
    labels={
        "label": "key",
    },
    annotations={
        "label-one": "value-one",
    })
default_workstation_config = gcp.workstations.WorkstationConfig("default",
    workstation_config_id="workstation-config",
    workstation_cluster_id=default_workstation_cluster.workstation_cluster_id,
    location="us-central1",
    host={
        "gce_instance": {
            "machine_type": "e2-standard-4",
            "boot_disk_size_gb": 35,
            "disable_public_ip_addresses": True,
        },
    })
default_workstation = gcp.workstations.Workstation("default",
    workstation_id="work-station",
    workstation_config_id=default_workstation_config.workstation_config_id,
    workstation_cluster_id=default_workstation_cluster.workstation_cluster_id,
    location="us-central1",
    labels={
        "label": "key",
    },
    env={
        "name": "foo",
    },
    annotations={
        "label-one": "value-one",
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Gcp = Pulumi.Gcp;

return await Deployment.RunAsync(() => 
{
    var @default = new Gcp.Compute.Network("default", new()
    {
        Name = "workstation-cluster",
        AutoCreateSubnetworks = false,
    });

    var defaultSubnetwork = new Gcp.Compute.Subnetwork("default", new()
    {
        Name = "workstation-cluster",
        IpCidrRange = "10.0.0.0/24",
        Region = "us-central1",
        Network = @default.Name,
    });

    var defaultWorkstationCluster = new Gcp.Workstations.WorkstationCluster("default", new()
    {
        WorkstationClusterId = "workstation-cluster",
        Network = @default.Id,
        Subnetwork = defaultSubnetwork.Id,
        Location = "us-central1",
        Labels = 
        {
            { "label", "key" },
        },
        Annotations = 
        {
            { "label-one", "value-one" },
        },
    });

    var defaultWorkstationConfig = new Gcp.Workstations.WorkstationConfig("default", new()
    {
        WorkstationConfigId = "workstation-config",
        WorkstationClusterId = defaultWorkstationCluster.WorkstationClusterId,
        Location = "us-central1",
        Host = new Gcp.Workstations.Inputs.WorkstationConfigHostArgs
        {
            GceInstance = new Gcp.Workstations.Inputs.WorkstationConfigHostGceInstanceArgs
            {
                MachineType = "e2-standard-4",
                BootDiskSizeGb = 35,
                DisablePublicIpAddresses = true,
            },
        },
    });

    var defaultWorkstation = new Gcp.Workstations.Workstation("default", new()
    {
        WorkstationId = "work-station",
        WorkstationConfigId = defaultWorkstationConfig.WorkstationConfigId,
        WorkstationClusterId = defaultWorkstationCluster.WorkstationClusterId,
        Location = "us-central1",
        Labels = 
        {
            { "label", "key" },
        },
        Env = 
        {
            { "name", "foo" },
        },
        Annotations = 
        {
            { "label-one", "value-one" },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-gcp/sdk/v8/go/gcp/compute"
	"github.com/pulumi/pulumi-gcp/sdk/v8/go/gcp/workstations"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := compute.NewNetwork(ctx, "default", &compute.NetworkArgs{
			Name:                  pulumi.String("workstation-cluster"),
			AutoCreateSubnetworks: pulumi.Bool(false),
		})
		if err != nil {
			return err
		}
		defaultSubnetwork, err := compute.NewSubnetwork(ctx, "default", &compute.SubnetworkArgs{
			Name:        pulumi.String("workstation-cluster"),
			IpCidrRange: pulumi.String("10.0.0.0/24"),
			Region:      pulumi.String("us-central1"),
			Network:     _default.Name,
		})
		if err != nil {
			return err
		}
		defaultWorkstationCluster, err := workstations.NewWorkstationCluster(ctx, "default", &workstations.WorkstationClusterArgs{
			WorkstationClusterId: pulumi.String("workstation-cluster"),
			Network:              _default.ID(),
			Subnetwork:           defaultSubnetwork.ID(),
			Location:             pulumi.String("us-central1"),
			Labels: pulumi.StringMap{
				"label": pulumi.String("key"),
			},
			Annotations: pulumi.StringMap{
				"label-one": pulumi.String("value-one"),
			},
		})
		if err != nil {
			return err
		}
		defaultWorkstationConfig, err := workstations.NewWorkstationConfig(ctx, "default", &workstations.WorkstationConfigArgs{
			WorkstationConfigId:  pulumi.String("workstation-config"),
			WorkstationClusterId: defaultWorkstationCluster.WorkstationClusterId,
			Location:             pulumi.String("us-central1"),
			Host: &workstations.WorkstationConfigHostArgs{
				GceInstance: &workstations.WorkstationConfigHostGceInstanceArgs{
					MachineType:              pulumi.String("e2-standard-4"),
					BootDiskSizeGb:           pulumi.Int(35),
					DisablePublicIpAddresses: pulumi.Bool(true),
				},
			},
		})
		if err != nil {
			return err
		}
		_, err = workstations.NewWorkstation(ctx, "default", &workstations.WorkstationArgs{
			WorkstationId:        pulumi.String("work-station"),
			WorkstationConfigId:  defaultWorkstationConfig.WorkstationConfigId,
			WorkstationClusterId: defaultWorkstationCluster.WorkstationClusterId,
			Location:             pulumi.String("us-central1"),
			Labels: pulumi.StringMap{
				"label": pulumi.String("key"),
			},
			Env: pulumi.StringMap{
				"name": pulumi.String("foo"),
			},
			Annotations: pulumi.StringMap{
				"label-one": pulumi.String("value-one"),
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.gcp.compute.Network;
import com.pulumi.gcp.compute.NetworkArgs;
import com.pulumi.gcp.compute.Subnetwork;
import com.pulumi.gcp.compute.SubnetworkArgs;
import com.pulumi.gcp.workstations.WorkstationCluster;
import com.pulumi.gcp.workstations.WorkstationClusterArgs;
import com.pulumi.gcp.workstations.WorkstationConfig;
import com.pulumi.gcp.workstations.WorkstationConfigArgs;
import com.pulumi.gcp.workstations.inputs.WorkstationConfigHostArgs;
import com.pulumi.gcp.workstations.inputs.WorkstationConfigHostGceInstanceArgs;
import com.pulumi.gcp.workstations.Workstation;
import com.pulumi.gcp.workstations.WorkstationArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var default_ = new Network("default", NetworkArgs.builder()
            .name("workstation-cluster")
            .autoCreateSubnetworks(false)
            .build());

        var defaultSubnetwork = new Subnetwork("defaultSubnetwork", SubnetworkArgs.builder()
            .name("workstation-cluster")
            .ipCidrRange("10.0.0.0/24")
            .region("us-central1")
            .network(default_.name())
            .build());

        var defaultWorkstationCluster = new WorkstationCluster("defaultWorkstationCluster", WorkstationClusterArgs.builder()
            .workstationClusterId("workstation-cluster")
            .network(default_.id())
            .subnetwork(defaultSubnetwork.id())
            .location("us-central1")
            .labels(Map.of("label", "key"))
            .annotations(Map.of("label-one", "value-one"))
            .build());

        var defaultWorkstationConfig = new WorkstationConfig("defaultWorkstationConfig", WorkstationConfigArgs.builder()
            .workstationConfigId("workstation-config")
            .workstationClusterId(defaultWorkstationCluster.workstationClusterId())
            .location("us-central1")
            .host(WorkstationConfigHostArgs.builder()
                .gceInstance(WorkstationConfigHostGceInstanceArgs.builder()
                    .machineType("e2-standard-4")
                    .bootDiskSizeGb(35)
                    .disablePublicIpAddresses(true)
                    .build())
                .build())
            .build());

        var defaultWorkstation = new Workstation("defaultWorkstation", WorkstationArgs.builder()
            .workstationId("work-station")
            .workstationConfigId(defaultWorkstationConfig.workstationConfigId())
            .workstationClusterId(defaultWorkstationCluster.workstationClusterId())
            .location("us-central1")
            .labels(Map.of("label", "key"))
            .env(Map.of("name", "foo"))
            .annotations(Map.of("label-one", "value-one"))
            .build());

    }
}
```
```yaml
resources:
  default:
    type: gcp:compute:Network
    properties:
      name: workstation-cluster
      autoCreateSubnetworks: false
  defaultSubnetwork:
    type: gcp:compute:Subnetwork
    name: default
    properties:
      name: workstation-cluster
      ipCidrRange: 10.0.0.0/24
      region: us-central1
      network: ${default.name}
  defaultWorkstationCluster:
    type: gcp:workstations:WorkstationCluster
    name: default
    properties:
      workstationClusterId: workstation-cluster
      network: ${default.id}
      subnetwork: ${defaultSubnetwork.id}
      location: us-central1
      labels:
        label: key
      annotations:
        label-one: value-one
  defaultWorkstationConfig:
    type: gcp:workstations:WorkstationConfig
    name: default
    properties:
      workstationConfigId: workstation-config
      workstationClusterId: ${defaultWorkstationCluster.workstationClusterId}
      location: us-central1
      host:
        gceInstance:
          machineType: e2-standard-4
          bootDiskSizeGb: 35
          disablePublicIpAddresses: true
  defaultWorkstation:
    type: gcp:workstations:Workstation
    name: default
    properties:
      workstationId: work-station
      workstationConfigId: ${defaultWorkstationConfig.workstationConfigId}
      workstationClusterId: ${defaultWorkstationCluster.workstationClusterId}
      location: us-central1
      labels:
        label: key
      env:
        name: foo
      annotations:
        label-one: value-one
```
<!--End PulumiCodeChooser -->

## Import

Workstation can be imported using any of these accepted formats:

* `projects/{{project}}/locations/{{location}}/workstationClusters/{{workstation_cluster_id}}/workstationConfigs/{{workstation_config_id}}/workstations/{{workstation_id}}`

* `{{project}}/{{location}}/{{workstation_cluster_id}}/{{workstation_config_id}}/{{workstation_id}}`

* `{{location}}/{{workstation_cluster_id}}/{{workstation_config_id}}/{{workstation_id}}`

When using the `pulumi import` command, Workstation can be imported using one of the formats above. For example:

```sh
$ pulumi import gcp:workstations/workstation:Workstation default projects/{{project}}/locations/{{location}}/workstationClusters/{{workstation_cluster_id}}/workstationConfigs/{{workstation_config_id}}/workstations/{{workstation_id}}
```

```sh
$ pulumi import gcp:workstations/workstation:Workstation default {{project}}/{{location}}/{{workstation_cluster_id}}/{{workstation_config_id}}/{{workstation_id}}
```

```sh
$ pulumi import gcp:workstations/workstation:Workstation default {{location}}/{{workstation_cluster_id}}/{{workstation_config_id}}/{{workstation_id}}
```

©
annotationsB2" ëClient-specified annotations. This is distinct from labels.
**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.
Please refer to the field `effective_annotations` for all of the annotations present on the resource.
<
displayNameB" 'Human-readable name for this resource.
j
envB2" ['Client-specified environment variables passed to the workstation container's entrypoint.'
€
labelsB2" »Client-specified labels that are applied to the resource and that are also propagated to the underlying Compute Engine resources.
**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
Please refer to the field `effective_labels` for all of the labels present on the resource.
T
location" DThe location where the workstation parent resources reside.


- - -
{
projectB" jThe ID of the project in which the resource belongs.
If it is not provided, the provider project is used.
F
workstationClusterId" *The ID of the parent workstation cluster.
L
workstationConfigId" 1The ID of the parent workstation cluster config.
4
workstationId" ID to use for the workstation.
"©
annotationsB2" ëClient-specified annotations. This is distinct from labels.
**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.
Please refer to the field `effective_annotations` for all of the annotations present on the resource.
"7

createTime" %Time when this resource was created.
"<
displayNameB" 'Human-readable name for this resource.
"
effectiveAnnotations2" "¶
effectiveLabels2" åAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
"j
envB2" ['Client-specified environment variables passed to the workstation container's entrypoint.'
"©
host" úHost to which clients can send HTTPS traffic that will be received by the workstation.
Authorized traffic will be received to the workstation as HTTP on port 80.
To send traffic to a different port, clients may prefix the host with the destination port in the format "{port}-{host}".
"€
labelsB2" »Client-specified labels that are applied to the resource and that are also propagated to the underlying Compute Engine resources.
**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
Please refer to the field `effective_labels` for all of the labels present on the resource.
"T
location" DThe location where the workstation parent resources reside.


- - -
"(
name" Full name of this resource.
"y
project" jThe ID of the project in which the resource belongs.
If it is not provided, the provider project is used.
"É
pulumiLabels2" mThe combination of labels configured directly on the resource
and default labels configured on the provider.
"/
state" "Current state of the workstation.
"B
uid" 7A system-assigned unique identified for this resource.
"F
workstationClusterId" *The ID of the parent workstation cluster.
"L
workstationConfigId" 1The ID of the parent workstation cluster config.
"4
workstationId" ID to use for the workstation.
*•‡
Z
workstationsWorkstationCluster6gcp:workstations/workstationCluster:WorkstationClusterã∏## Example Usage

### Workstation Cluster Basic


<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as gcp from "@pulumi/gcp";

const defaultNetwork = new gcp.compute.Network("default", {
    name: "workstation-cluster",
    autoCreateSubnetworks: false,
});
const defaultSubnetwork = new gcp.compute.Subnetwork("default", {
    name: "workstation-cluster",
    ipCidrRange: "10.0.0.0/24",
    region: "us-central1",
    network: defaultNetwork.name,
});
const _default = new gcp.workstations.WorkstationCluster("default", {
    workstationClusterId: "workstation-cluster",
    network: defaultNetwork.id,
    subnetwork: defaultSubnetwork.id,
    location: "us-central1",
    labels: {
        label: "key",
    },
    annotations: {
        "label-one": "value-one",
    },
});
const project = gcp.organizations.getProject({});
```
```python
import pulumi
import pulumi_gcp as gcp

default_network = gcp.compute.Network("default",
    name="workstation-cluster",
    auto_create_subnetworks=False)
default_subnetwork = gcp.compute.Subnetwork("default",
    name="workstation-cluster",
    ip_cidr_range="10.0.0.0/24",
    region="us-central1",
    network=default_network.name)
default = gcp.workstations.WorkstationCluster("default",
    workstation_cluster_id="workstation-cluster",
    network=default_network.id,
    subnetwork=default_subnetwork.id,
    location="us-central1",
    labels={
        "label": "key",
    },
    annotations={
        "label-one": "value-one",
    })
project = gcp.organizations.get_project()
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Gcp = Pulumi.Gcp;

return await Deployment.RunAsync(() => 
{
    var defaultNetwork = new Gcp.Compute.Network("default", new()
    {
        Name = "workstation-cluster",
        AutoCreateSubnetworks = false,
    });

    var defaultSubnetwork = new Gcp.Compute.Subnetwork("default", new()
    {
        Name = "workstation-cluster",
        IpCidrRange = "10.0.0.0/24",
        Region = "us-central1",
        Network = defaultNetwork.Name,
    });

    var @default = new Gcp.Workstations.WorkstationCluster("default", new()
    {
        WorkstationClusterId = "workstation-cluster",
        Network = defaultNetwork.Id,
        Subnetwork = defaultSubnetwork.Id,
        Location = "us-central1",
        Labels = 
        {
            { "label", "key" },
        },
        Annotations = 
        {
            { "label-one", "value-one" },
        },
    });

    var project = Gcp.Organizations.GetProject.Invoke();

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-gcp/sdk/v8/go/gcp/compute"
	"github.com/pulumi/pulumi-gcp/sdk/v8/go/gcp/organizations"
	"github.com/pulumi/pulumi-gcp/sdk/v8/go/gcp/workstations"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		defaultNetwork, err := compute.NewNetwork(ctx, "default", &compute.NetworkArgs{
			Name:                  pulumi.String("workstation-cluster"),
			AutoCreateSubnetworks: pulumi.Bool(false),
		})
		if err != nil {
			return err
		}
		defaultSubnetwork, err := compute.NewSubnetwork(ctx, "default", &compute.SubnetworkArgs{
			Name:        pulumi.String("workstation-cluster"),
			IpCidrRange: pulumi.String("10.0.0.0/24"),
			Region:      pulumi.String("us-central1"),
			Network:     defaultNetwork.Name,
		})
		if err != nil {
			return err
		}
		_, err = workstations.NewWorkstationCluster(ctx, "default", &workstations.WorkstationClusterArgs{
			WorkstationClusterId: pulumi.String("workstation-cluster"),
			Network:              defaultNetwork.ID(),
			Subnetwork:           defaultSubnetwork.ID(),
			Location:             pulumi.String("us-central1"),
			Labels: pulumi.StringMap{
				"label": pulumi.String("key"),
			},
			Annotations: pulumi.StringMap{
				"label-one": pulumi.String("value-one"),
			},
		})
		if err != nil {
			return err
		}
		_, err = organizations.LookupProject(ctx, &organizations.LookupProjectArgs{}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.gcp.compute.Network;
import com.pulumi.gcp.compute.NetworkArgs;
import com.pulumi.gcp.compute.Subnetwork;
import com.pulumi.gcp.compute.SubnetworkArgs;
import com.pulumi.gcp.workstations.WorkstationCluster;
import com.pulumi.gcp.workstations.WorkstationClusterArgs;
import com.pulumi.gcp.organizations.OrganizationsFunctions;
import com.pulumi.gcp.organizations.inputs.GetProjectArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var defaultNetwork = new Network("defaultNetwork", NetworkArgs.builder()
            .name("workstation-cluster")
            .autoCreateSubnetworks(false)
            .build());

        var defaultSubnetwork = new Subnetwork("defaultSubnetwork", SubnetworkArgs.builder()
            .name("workstation-cluster")
            .ipCidrRange("10.0.0.0/24")
            .region("us-central1")
            .network(defaultNetwork.name())
            .build());

        var default_ = new WorkstationCluster("default", WorkstationClusterArgs.builder()
            .workstationClusterId("workstation-cluster")
            .network(defaultNetwork.id())
            .subnetwork(defaultSubnetwork.id())
            .location("us-central1")
            .labels(Map.of("label", "key"))
            .annotations(Map.of("label-one", "value-one"))
            .build());

        final var project = OrganizationsFunctions.getProject();

    }
}
```
```yaml
resources:
  default:
    type: gcp:workstations:WorkstationCluster
    properties:
      workstationClusterId: workstation-cluster
      network: ${defaultNetwork.id}
      subnetwork: ${defaultSubnetwork.id}
      location: us-central1
      labels:
        label: key
      annotations:
        label-one: value-one
  defaultNetwork:
    type: gcp:compute:Network
    name: default
    properties:
      name: workstation-cluster
      autoCreateSubnetworks: false
  defaultSubnetwork:
    type: gcp:compute:Subnetwork
    name: default
    properties:
      name: workstation-cluster
      ipCidrRange: 10.0.0.0/24
      region: us-central1
      network: ${defaultNetwork.name}
variables:
  project:
    fn::invoke:
      function: gcp:organizations:getProject
      arguments: {}
```
<!--End PulumiCodeChooser -->
### Workstation Cluster Private


<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as gcp from "@pulumi/gcp";

const defaultNetwork = new gcp.compute.Network("default", {
    name: "workstation-cluster-private",
    autoCreateSubnetworks: false,
});
const defaultSubnetwork = new gcp.compute.Subnetwork("default", {
    name: "workstation-cluster-private",
    ipCidrRange: "10.0.0.0/24",
    region: "us-central1",
    network: defaultNetwork.name,
});
const _default = new gcp.workstations.WorkstationCluster("default", {
    workstationClusterId: "workstation-cluster-private",
    network: defaultNetwork.id,
    subnetwork: defaultSubnetwork.id,
    location: "us-central1",
    privateClusterConfig: {
        enablePrivateEndpoint: true,
    },
    labels: {
        label: "key",
    },
    annotations: {
        "label-one": "value-one",
    },
});
const project = gcp.organizations.getProject({});
```
```python
import pulumi
import pulumi_gcp as gcp

default_network = gcp.compute.Network("default",
    name="workstation-cluster-private",
    auto_create_subnetworks=False)
default_subnetwork = gcp.compute.Subnetwork("default",
    name="workstation-cluster-private",
    ip_cidr_range="10.0.0.0/24",
    region="us-central1",
    network=default_network.name)
default = gcp.workstations.WorkstationCluster("default",
    workstation_cluster_id="workstation-cluster-private",
    network=default_network.id,
    subnetwork=default_subnetwork.id,
    location="us-central1",
    private_cluster_config={
        "enable_private_endpoint": True,
    },
    labels={
        "label": "key",
    },
    annotations={
        "label-one": "value-one",
    })
project = gcp.organizations.get_project()
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Gcp = Pulumi.Gcp;

return await Deployment.RunAsync(() => 
{
    var defaultNetwork = new Gcp.Compute.Network("default", new()
    {
        Name = "workstation-cluster-private",
        AutoCreateSubnetworks = false,
    });

    var defaultSubnetwork = new Gcp.Compute.Subnetwork("default", new()
    {
        Name = "workstation-cluster-private",
        IpCidrRange = "10.0.0.0/24",
        Region = "us-central1",
        Network = defaultNetwork.Name,
    });

    var @default = new Gcp.Workstations.WorkstationCluster("default", new()
    {
        WorkstationClusterId = "workstation-cluster-private",
        Network = defaultNetwork.Id,
        Subnetwork = defaultSubnetwork.Id,
        Location = "us-central1",
        PrivateClusterConfig = new Gcp.Workstations.Inputs.WorkstationClusterPrivateClusterConfigArgs
        {
            EnablePrivateEndpoint = true,
        },
        Labels = 
        {
            { "label", "key" },
        },
        Annotations = 
        {
            { "label-one", "value-one" },
        },
    });

    var project = Gcp.Organizations.GetProject.Invoke();

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-gcp/sdk/v8/go/gcp/compute"
	"github.com/pulumi/pulumi-gcp/sdk/v8/go/gcp/organizations"
	"github.com/pulumi/pulumi-gcp/sdk/v8/go/gcp/workstations"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		defaultNetwork, err := compute.NewNetwork(ctx, "default", &compute.NetworkArgs{
			Name:                  pulumi.String("workstation-cluster-private"),
			AutoCreateSubnetworks: pulumi.Bool(false),
		})
		if err != nil {
			return err
		}
		defaultSubnetwork, err := compute.NewSubnetwork(ctx, "default", &compute.SubnetworkArgs{
			Name:        pulumi.String("workstation-cluster-private"),
			IpCidrRange: pulumi.String("10.0.0.0/24"),
			Region:      pulumi.String("us-central1"),
			Network:     defaultNetwork.Name,
		})
		if err != nil {
			return err
		}
		_, err = workstations.NewWorkstationCluster(ctx, "default", &workstations.WorkstationClusterArgs{
			WorkstationClusterId: pulumi.String("workstation-cluster-private"),
			Network:              defaultNetwork.ID(),
			Subnetwork:           defaultSubnetwork.ID(),
			Location:             pulumi.String("us-central1"),
			PrivateClusterConfig: &workstations.WorkstationClusterPrivateClusterConfigArgs{
				EnablePrivateEndpoint: pulumi.Bool(true),
			},
			Labels: pulumi.StringMap{
				"label": pulumi.String("key"),
			},
			Annotations: pulumi.StringMap{
				"label-one": pulumi.String("value-one"),
			},
		})
		if err != nil {
			return err
		}
		_, err = organizations.LookupProject(ctx, &organizations.LookupProjectArgs{}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.gcp.compute.Network;
import com.pulumi.gcp.compute.NetworkArgs;
import com.pulumi.gcp.compute.Subnetwork;
import com.pulumi.gcp.compute.SubnetworkArgs;
import com.pulumi.gcp.workstations.WorkstationCluster;
import com.pulumi.gcp.workstations.WorkstationClusterArgs;
import com.pulumi.gcp.workstations.inputs.WorkstationClusterPrivateClusterConfigArgs;
import com.pulumi.gcp.organizations.OrganizationsFunctions;
import com.pulumi.gcp.organizations.inputs.GetProjectArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var defaultNetwork = new Network("defaultNetwork", NetworkArgs.builder()
            .name("workstation-cluster-private")
            .autoCreateSubnetworks(false)
            .build());

        var defaultSubnetwork = new Subnetwork("defaultSubnetwork", SubnetworkArgs.builder()
            .name("workstation-cluster-private")
            .ipCidrRange("10.0.0.0/24")
            .region("us-central1")
            .network(defaultNetwork.name())
            .build());

        var default_ = new WorkstationCluster("default", WorkstationClusterArgs.builder()
            .workstationClusterId("workstation-cluster-private")
            .network(defaultNetwork.id())
            .subnetwork(defaultSubnetwork.id())
            .location("us-central1")
            .privateClusterConfig(WorkstationClusterPrivateClusterConfigArgs.builder()
                .enablePrivateEndpoint(true)
                .build())
            .labels(Map.of("label", "key"))
            .annotations(Map.of("label-one", "value-one"))
            .build());

        final var project = OrganizationsFunctions.getProject();

    }
}
```
```yaml
resources:
  default:
    type: gcp:workstations:WorkstationCluster
    properties:
      workstationClusterId: workstation-cluster-private
      network: ${defaultNetwork.id}
      subnetwork: ${defaultSubnetwork.id}
      location: us-central1
      privateClusterConfig:
        enablePrivateEndpoint: true
      labels:
        label: key
      annotations:
        label-one: value-one
  defaultNetwork:
    type: gcp:compute:Network
    name: default
    properties:
      name: workstation-cluster-private
      autoCreateSubnetworks: false
  defaultSubnetwork:
    type: gcp:compute:Subnetwork
    name: default
    properties:
      name: workstation-cluster-private
      ipCidrRange: 10.0.0.0/24
      region: us-central1
      network: ${defaultNetwork.name}
variables:
  project:
    fn::invoke:
      function: gcp:organizations:getProject
      arguments: {}
```
<!--End PulumiCodeChooser -->
### Workstation Cluster Custom Domain


<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as gcp from "@pulumi/gcp";

const defaultNetwork = new gcp.compute.Network("default", {
    name: "workstation-cluster-custom-domain",
    autoCreateSubnetworks: false,
});
const defaultSubnetwork = new gcp.compute.Subnetwork("default", {
    name: "workstation-cluster-custom-domain",
    ipCidrRange: "10.0.0.0/24",
    region: "us-central1",
    network: defaultNetwork.name,
});
const _default = new gcp.workstations.WorkstationCluster("default", {
    workstationClusterId: "workstation-cluster-custom-domain",
    network: defaultNetwork.id,
    subnetwork: defaultSubnetwork.id,
    location: "us-central1",
    privateClusterConfig: {
        enablePrivateEndpoint: true,
    },
    domainConfig: {
        domain: "workstations.example.com",
    },
    labels: {
        label: "key",
    },
    annotations: {
        "label-one": "value-one",
    },
});
const project = gcp.organizations.getProject({});
```
```python
import pulumi
import pulumi_gcp as gcp

default_network = gcp.compute.Network("default",
    name="workstation-cluster-custom-domain",
    auto_create_subnetworks=False)
default_subnetwork = gcp.compute.Subnetwork("default",
    name="workstation-cluster-custom-domain",
    ip_cidr_range="10.0.0.0/24",
    region="us-central1",
    network=default_network.name)
default = gcp.workstations.WorkstationCluster("default",
    workstation_cluster_id="workstation-cluster-custom-domain",
    network=default_network.id,
    subnetwork=default_subnetwork.id,
    location="us-central1",
    private_cluster_config={
        "enable_private_endpoint": True,
    },
    domain_config={
        "domain": "workstations.example.com",
    },
    labels={
        "label": "key",
    },
    annotations={
        "label-one": "value-one",
    })
project = gcp.organizations.get_project()
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Gcp = Pulumi.Gcp;

return await Deployment.RunAsync(() => 
{
    var defaultNetwork = new Gcp.Compute.Network("default", new()
    {
        Name = "workstation-cluster-custom-domain",
        AutoCreateSubnetworks = false,
    });

    var defaultSubnetwork = new Gcp.Compute.Subnetwork("default", new()
    {
        Name = "workstation-cluster-custom-domain",
        IpCidrRange = "10.0.0.0/24",
        Region = "us-central1",
        Network = defaultNetwork.Name,
    });

    var @default = new Gcp.Workstations.WorkstationCluster("default", new()
    {
        WorkstationClusterId = "workstation-cluster-custom-domain",
        Network = defaultNetwork.Id,
        Subnetwork = defaultSubnetwork.Id,
        Location = "us-central1",
        PrivateClusterConfig = new Gcp.Workstations.Inputs.WorkstationClusterPrivateClusterConfigArgs
        {
            EnablePrivateEndpoint = true,
        },
        DomainConfig = new Gcp.Workstations.Inputs.WorkstationClusterDomainConfigArgs
        {
            Domain = "workstations.example.com",
        },
        Labels = 
        {
            { "label", "key" },
        },
        Annotations = 
        {
            { "label-one", "value-one" },
        },
    });

    var project = Gcp.Organizations.GetProject.Invoke();

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-gcp/sdk/v8/go/gcp/compute"
	"github.com/pulumi/pulumi-gcp/sdk/v8/go/gcp/organizations"
	"github.com/pulumi/pulumi-gcp/sdk/v8/go/gcp/workstations"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		defaultNetwork, err := compute.NewNetwork(ctx, "default", &compute.NetworkArgs{
			Name:                  pulumi.String("workstation-cluster-custom-domain"),
			AutoCreateSubnetworks: pulumi.Bool(false),
		})
		if err != nil {
			return err
		}
		defaultSubnetwork, err := compute.NewSubnetwork(ctx, "default", &compute.SubnetworkArgs{
			Name:        pulumi.String("workstation-cluster-custom-domain"),
			IpCidrRange: pulumi.String("10.0.0.0/24"),
			Region:      pulumi.String("us-central1"),
			Network:     defaultNetwork.Name,
		})
		if err != nil {
			return err
		}
		_, err = workstations.NewWorkstationCluster(ctx, "default", &workstations.WorkstationClusterArgs{
			WorkstationClusterId: pulumi.String("workstation-cluster-custom-domain"),
			Network:              defaultNetwork.ID(),
			Subnetwork:           defaultSubnetwork.ID(),
			Location:             pulumi.String("us-central1"),
			PrivateClusterConfig: &workstations.WorkstationClusterPrivateClusterConfigArgs{
				EnablePrivateEndpoint: pulumi.Bool(true),
			},
			DomainConfig: &workstations.WorkstationClusterDomainConfigArgs{
				Domain: pulumi.String("workstations.example.com"),
			},
			Labels: pulumi.StringMap{
				"label": pulumi.String("key"),
			},
			Annotations: pulumi.StringMap{
				"label-one": pulumi.String("value-one"),
			},
		})
		if err != nil {
			return err
		}
		_, err = organizations.LookupProject(ctx, &organizations.LookupProjectArgs{}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.gcp.compute.Network;
import com.pulumi.gcp.compute.NetworkArgs;
import com.pulumi.gcp.compute.Subnetwork;
import com.pulumi.gcp.compute.SubnetworkArgs;
import com.pulumi.gcp.workstations.WorkstationCluster;
import com.pulumi.gcp.workstations.WorkstationClusterArgs;
import com.pulumi.gcp.workstations.inputs.WorkstationClusterPrivateClusterConfigArgs;
import com.pulumi.gcp.workstations.inputs.WorkstationClusterDomainConfigArgs;
import com.pulumi.gcp.organizations.OrganizationsFunctions;
import com.pulumi.gcp.organizations.inputs.GetProjectArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var defaultNetwork = new Network("defaultNetwork", NetworkArgs.builder()
            .name("workstation-cluster-custom-domain")
            .autoCreateSubnetworks(false)
            .build());

        var defaultSubnetwork = new Subnetwork("defaultSubnetwork", SubnetworkArgs.builder()
            .name("workstation-cluster-custom-domain")
            .ipCidrRange("10.0.0.0/24")
            .region("us-central1")
            .network(defaultNetwork.name())
            .build());

        var default_ = new WorkstationCluster("default", WorkstationClusterArgs.builder()
            .workstationClusterId("workstation-cluster-custom-domain")
            .network(defaultNetwork.id())
            .subnetwork(defaultSubnetwork.id())
            .location("us-central1")
            .privateClusterConfig(WorkstationClusterPrivateClusterConfigArgs.builder()
                .enablePrivateEndpoint(true)
                .build())
            .domainConfig(WorkstationClusterDomainConfigArgs.builder()
                .domain("workstations.example.com")
                .build())
            .labels(Map.of("label", "key"))
            .annotations(Map.of("label-one", "value-one"))
            .build());

        final var project = OrganizationsFunctions.getProject();

    }
}
```
```yaml
resources:
  default:
    type: gcp:workstations:WorkstationCluster
    properties:
      workstationClusterId: workstation-cluster-custom-domain
      network: ${defaultNetwork.id}
      subnetwork: ${defaultSubnetwork.id}
      location: us-central1
      privateClusterConfig:
        enablePrivateEndpoint: true
      domainConfig:
        domain: workstations.example.com
      labels:
        label: key
      annotations:
        label-one: value-one
  defaultNetwork:
    type: gcp:compute:Network
    name: default
    properties:
      name: workstation-cluster-custom-domain
      autoCreateSubnetworks: false
  defaultSubnetwork:
    type: gcp:compute:Subnetwork
    name: default
    properties:
      name: workstation-cluster-custom-domain
      ipCidrRange: 10.0.0.0/24
      region: us-central1
      network: ${defaultNetwork.name}
variables:
  project:
    fn::invoke:
      function: gcp:organizations:getProject
      arguments: {}
```
<!--End PulumiCodeChooser -->

## Import

WorkstationCluster can be imported using any of these accepted formats:

* `projects/{{project}}/locations/{{location}}/workstationClusters/{{workstation_cluster_id}}`

* `{{project}}/{{location}}/{{workstation_cluster_id}}`

* `{{location}}/{{workstation_cluster_id}}`

When using the `pulumi import` command, WorkstationCluster can be imported using one of the formats above. For example:

```sh
$ pulumi import gcp:workstations/workstationCluster:WorkstationCluster default projects/{{project}}/locations/{{location}}/workstationClusters/{{workstation_cluster_id}}
```

```sh
$ pulumi import gcp:workstations/workstationCluster:WorkstationCluster default {{project}}/{{location}}/{{workstation_cluster_id}}
```

```sh
$ pulumi import gcp:workstations/workstationCluster:WorkstationCluster default {{location}}/{{workstation_cluster_id}}
```

©
annotationsB2" ëClient-specified annotations. This is distinct from labels.
**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.
Please refer to the field `effective_annotations` for all of the annotations present on the resource.
<
displayNameB" 'Human-readable name for this resource.
„
domainConfigÜBÉ:Ä
~
workstationsWorkstationClusterDomainConfigNgcp:workstations/WorkstationClusterDomainConfig:WorkstationClusterDomainConfigJConfiguration options for a custom domain.
Structure is documented below.
€
labelsB2" »Client-specified labels that are applied to the resource and that are also propagated to the underlying Compute Engine resources.
**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
Please refer to the field `effective_labels` for all of the labels present on the resource.
L
locationB" :The location where the workstation cluster should reside.
≈
network" µThe relative resource name of the VPC network on which the instance can be accessed.
It is specified in the following form: "projects/{projectNumber}/global/networks/{network_id}".
¸
privateClusterConfigüBú:ô
ñ
workstations&WorkstationClusterPrivateClusterConfig^gcp:workstations/WorkstationClusterPrivateClusterConfig:WorkstationClusterPrivateClusterConfigBConfiguration for private cluster.
Structure is documented below.
{
projectB" jThe ID of the project in which the resource belongs.
If it is not provided, the provider project is used.
µ

subnetwork" ¢Name of the Compute Engine subnetwork in which instances associated with this cluster will be created.
Must be part of the subnetwork specified for this cluster.
K
workstationClusterId" /ID to use for the workstation cluster.


- - -
"©
annotationsB2" ëClient-specified annotations. This is distinct from labels.
**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.
Please refer to the field `effective_annotations` for all of the annotations present on the resource.
"„

conditions{*y:w
u
workstationsWorkstationClusterConditionHgcp:workstations/WorkstationClusterCondition:WorkstationClusterConditionXStatus conditions describing the current resource state.
Structure is documented below.
"á
controlPlaneIp" The private IP address of the control plane for this workstation cluster.
Workstation VMs need access to this IP address to work with the service, so make sure that your firewall rules allow egress from the workstation VMs to this address.
"7

createTime" %Time when this resource was created.
"≤
degraded
 °Whether this resource is in degraded mode, in which case it may require user action to restore full functionality.
Details can be found in the conditions field.
"<
displayNameB" 'Human-readable name for this resource.
"„
domainConfigÜBÉ:Ä
~
workstationsWorkstationClusterDomainConfigNgcp:workstations/WorkstationClusterDomainConfig:WorkstationClusterDomainConfigJConfiguration options for a custom domain.
Structure is documented below.
"
effectiveAnnotations2" "¶
effectiveLabels2" åAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
"ù
etag" êChecksum computed by the server.
May be sent on update and delete requests to ensure that the client has an up-to-date value before proceeding.
"€
labelsB2" »Client-specified labels that are applied to the resource and that are also propagated to the underlying Compute Engine resources.
**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
Please refer to the field `effective_labels` for all of the labels present on the resource.
"L
locationB" :The location where the workstation cluster should reside.
".
name" "The name of the cluster resource.
"≈
network" µThe relative resource name of the VPC network on which the instance can be accessed.
It is specified in the following form: "projects/{projectNumber}/global/networks/{network_id}".
"¸
privateClusterConfigüBú:ô
ñ
workstations&WorkstationClusterPrivateClusterConfig^gcp:workstations/WorkstationClusterPrivateClusterConfig:WorkstationClusterPrivateClusterConfigBConfiguration for private cluster.
Structure is documented below.
"y
project" jThe ID of the project in which the resource belongs.
If it is not provided, the provider project is used.
"É
pulumiLabels2" mThe combination of labels configured directly on the resource
and default labels configured on the provider.
"µ

subnetwork" ¢Name of the Compute Engine subnetwork in which instances associated with this cluster will be created.
Must be part of the subnetwork specified for this cluster.
"5
uid" *The system-generated UID of the resource.
"K
workstationClusterId" /ID to use for the workstation cluster.


- - -
*ÿŒ
W
workstationsWorkstationConfig4gcp:workstations/workstationConfig:WorkstationConfig≈Ï## Example Usage

### Workstation Config Basic


<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as gcp from "@pulumi/gcp";

const tagKey1 = new gcp.tags.TagKey("tag_key1", {
    parent: "organizations/123456789",
    shortName: "keyname",
});
const tagValue1 = new gcp.tags.TagValue("tag_value1", {
    parent: tagKey1.id,
    shortName: "valuename",
});
const _default = new gcp.compute.Network("default", {
    name: "workstation-cluster",
    autoCreateSubnetworks: false,
});
const defaultSubnetwork = new gcp.compute.Subnetwork("default", {
    name: "workstation-cluster",
    ipCidrRange: "10.0.0.0/24",
    region: "us-central1",
    network: _default.name,
});
const defaultWorkstationCluster = new gcp.workstations.WorkstationCluster("default", {
    workstationClusterId: "workstation-cluster",
    network: _default.id,
    subnetwork: defaultSubnetwork.id,
    location: "us-central1",
    labels: {
        label: "key",
    },
    annotations: {
        "label-one": "value-one",
    },
});
const defaultWorkstationConfig = new gcp.workstations.WorkstationConfig("default", {
    workstationConfigId: "workstation-config",
    workstationClusterId: defaultWorkstationCluster.workstationClusterId,
    location: "us-central1",
    idleTimeout: "600s",
    runningTimeout: "21600s",
    replicaZones: [
        "us-central1-a",
        "us-central1-b",
    ],
    annotations: {
        "label-one": "value-one",
    },
    labels: {
        label: "key",
    },
    maxUsableWorkstations: 1,
    host: {
        gceInstance: {
            machineType: "e2-standard-4",
            bootDiskSizeGb: 35,
            disablePublicIpAddresses: true,
            disableSsh: false,
            vmTags: pulumi.all([tagKey1.id, tagValue1.id]).apply(([tagKey1Id, tagValue1Id]) => {
                [tagKey1Id]: tagValue1Id,
            }),
        },
    },
});
```
```python
import pulumi
import pulumi_gcp as gcp

tag_key1 = gcp.tags.TagKey("tag_key1",
    parent="organizations/123456789",
    short_name="keyname")
tag_value1 = gcp.tags.TagValue("tag_value1",
    parent=tag_key1.id,
    short_name="valuename")
default = gcp.compute.Network("default",
    name="workstation-cluster",
    auto_create_subnetworks=False)
default_subnetwork = gcp.compute.Subnetwork("default",
    name="workstation-cluster",
    ip_cidr_range="10.0.0.0/24",
    region="us-central1",
    network=default.name)
default_workstation_cluster = gcp.workstations.WorkstationCluster("default",
    workstation_cluster_id="workstation-cluster",
    network=default.id,
    subnetwork=default_subnetwork.id,
    location="us-central1",
    labels={
        "label": "key",
    },
    annotations={
        "label-one": "value-one",
    })
default_workstation_config = gcp.workstations.WorkstationConfig("default",
    workstation_config_id="workstation-config",
    workstation_cluster_id=default_workstation_cluster.workstation_cluster_id,
    location="us-central1",
    idle_timeout="600s",
    running_timeout="21600s",
    replica_zones=[
        "us-central1-a",
        "us-central1-b",
    ],
    annotations={
        "label-one": "value-one",
    },
    labels={
        "label": "key",
    },
    max_usable_workstations=1,
    host={
        "gce_instance": {
            "machine_type": "e2-standard-4",
            "boot_disk_size_gb": 35,
            "disable_public_ip_addresses": True,
            "disable_ssh": False,
            "vm_tags": pulumi.Output.all(
                tagKey1Id=tag_key1.id,
                tagValue1Id=tag_value1.id
).apply(lambda resolved_outputs: {
                resolved_outputs['tagKey1Id']: resolved_outputs['tagValue1Id'],
            })
,
        },
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Gcp = Pulumi.Gcp;

return await Deployment.RunAsync(() => 
{
    var tagKey1 = new Gcp.Tags.TagKey("tag_key1", new()
    {
        Parent = "organizations/123456789",
        ShortName = "keyname",
    });

    var tagValue1 = new Gcp.Tags.TagValue("tag_value1", new()
    {
        Parent = tagKey1.Id,
        ShortName = "valuename",
    });

    var @default = new Gcp.Compute.Network("default", new()
    {
        Name = "workstation-cluster",
        AutoCreateSubnetworks = false,
    });

    var defaultSubnetwork = new Gcp.Compute.Subnetwork("default", new()
    {
        Name = "workstation-cluster",
        IpCidrRange = "10.0.0.0/24",
        Region = "us-central1",
        Network = @default.Name,
    });

    var defaultWorkstationCluster = new Gcp.Workstations.WorkstationCluster("default", new()
    {
        WorkstationClusterId = "workstation-cluster",
        Network = @default.Id,
        Subnetwork = defaultSubnetwork.Id,
        Location = "us-central1",
        Labels = 
        {
            { "label", "key" },
        },
        Annotations = 
        {
            { "label-one", "value-one" },
        },
    });

    var defaultWorkstationConfig = new Gcp.Workstations.WorkstationConfig("default", new()
    {
        WorkstationConfigId = "workstation-config",
        WorkstationClusterId = defaultWorkstationCluster.WorkstationClusterId,
        Location = "us-central1",
        IdleTimeout = "600s",
        RunningTimeout = "21600s",
        ReplicaZones = new[]
        {
            "us-central1-a",
            "us-central1-b",
        },
        Annotations = 
        {
            { "label-one", "value-one" },
        },
        Labels = 
        {
            { "label", "key" },
        },
        MaxUsableWorkstations = 1,
        Host = new Gcp.Workstations.Inputs.WorkstationConfigHostArgs
        {
            GceInstance = new Gcp.Workstations.Inputs.WorkstationConfigHostGceInstanceArgs
            {
                MachineType = "e2-standard-4",
                BootDiskSizeGb = 35,
                DisablePublicIpAddresses = true,
                DisableSsh = false,
                VmTags = Output.Tuple(tagKey1.Id, tagValue1.Id).Apply(values =>
                {
                    var tagKey1Id = values.Item1;
                    var tagValue1Id = values.Item2;
                    return 
                    {
                        { tagKey1Id, tagValue1Id },
                    };
                }),
            },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-gcp/sdk/v8/go/gcp/compute"
	"github.com/pulumi/pulumi-gcp/sdk/v8/go/gcp/tags"
	"github.com/pulumi/pulumi-gcp/sdk/v8/go/gcp/workstations"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)
func main() {
pulumi.Run(func(ctx *pulumi.Context) error {
tagKey1, err := tags.NewTagKey(ctx, "tag_key1", &tags.TagKeyArgs{
Parent: pulumi.String("organizations/123456789"),
ShortName: pulumi.String("keyname"),
})
if err != nil {
return err
}
tagValue1, err := tags.NewTagValue(ctx, "tag_value1", &tags.TagValueArgs{
Parent: tagKey1.ID(),
ShortName: pulumi.String("valuename"),
})
if err != nil {
return err
}
_, err = compute.NewNetwork(ctx, "default", &compute.NetworkArgs{
Name: pulumi.String("workstation-cluster"),
AutoCreateSubnetworks: pulumi.Bool(false),
})
if err != nil {
return err
}
defaultSubnetwork, err := compute.NewSubnetwork(ctx, "default", &compute.SubnetworkArgs{
Name: pulumi.String("workstation-cluster"),
IpCidrRange: pulumi.String("10.0.0.0/24"),
Region: pulumi.String("us-central1"),
Network: _default.Name,
})
if err != nil {
return err
}
defaultWorkstationCluster, err := workstations.NewWorkstationCluster(ctx, "default", &workstations.WorkstationClusterArgs{
WorkstationClusterId: pulumi.String("workstation-cluster"),
Network: _default.ID(),
Subnetwork: defaultSubnetwork.ID(),
Location: pulumi.String("us-central1"),
Labels: pulumi.StringMap{
"label": pulumi.String("key"),
},
Annotations: pulumi.StringMap{
"label-one": pulumi.String("value-one"),
},
})
if err != nil {
return err
}
_, err = workstations.NewWorkstationConfig(ctx, "default", &workstations.WorkstationConfigArgs{
WorkstationConfigId: pulumi.String("workstation-config"),
WorkstationClusterId: defaultWorkstationCluster.WorkstationClusterId,
Location: pulumi.String("us-central1"),
IdleTimeout: pulumi.String("600s"),
RunningTimeout: pulumi.String("21600s"),
ReplicaZones: pulumi.StringArray{
pulumi.String("us-central1-a"),
pulumi.String("us-central1-b"),
},
Annotations: pulumi.StringMap{
"label-one": pulumi.String("value-one"),
},
Labels: pulumi.StringMap{
"label": pulumi.String("key"),
},
MaxUsableWorkstations: pulumi.Int(1),
Host: &workstations.WorkstationConfigHostArgs{
GceInstance: &workstations.WorkstationConfigHostGceInstanceArgs{
MachineType: pulumi.String("e2-standard-4"),
BootDiskSizeGb: pulumi.Int(35),
DisablePublicIpAddresses: pulumi.Bool(true),
DisableSsh: pulumi.Bool(false),
VmTags: pulumi.All(tagKey1.ID(),tagValue1.ID()).ApplyT(func(_args []interface{}) (map[string]string, error) {
tagKey1Id := _args[0].(string)
tagValue1Id := _args[1].(string)
return map[string]string{
tagKey1Id: tagValue1Id,
}, nil
}).(pulumi.Map[string]stringOutput),
},
},
})
if err != nil {
return err
}
return nil
})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.gcp.tags.TagKey;
import com.pulumi.gcp.tags.TagKeyArgs;
import com.pulumi.gcp.tags.TagValue;
import com.pulumi.gcp.tags.TagValueArgs;
import com.pulumi.gcp.compute.Network;
import com.pulumi.gcp.compute.NetworkArgs;
import com.pulumi.gcp.compute.Subnetwork;
import com.pulumi.gcp.compute.SubnetworkArgs;
import com.pulumi.gcp.workstations.WorkstationCluster;
import com.pulumi.gcp.workstations.WorkstationClusterArgs;
import com.pulumi.gcp.workstations.WorkstationConfig;
import com.pulumi.gcp.workstations.WorkstationConfigArgs;
import com.pulumi.gcp.workstations.inputs.WorkstationConfigHostArgs;
import com.pulumi.gcp.workstations.inputs.WorkstationConfigHostGceInstanceArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var tagKey1 = new TagKey("tagKey1", TagKeyArgs.builder()
            .parent("organizations/123456789")
            .shortName("keyname")
            .build());

        var tagValue1 = new TagValue("tagValue1", TagValueArgs.builder()
            .parent(tagKey1.id())
            .shortName("valuename")
            .build());

        var default_ = new Network("default", NetworkArgs.builder()
            .name("workstation-cluster")
            .autoCreateSubnetworks(false)
            .build());

        var defaultSubnetwork = new Subnetwork("defaultSubnetwork", SubnetworkArgs.builder()
            .name("workstation-cluster")
            .ipCidrRange("10.0.0.0/24")
            .region("us-central1")
            .network(default_.name())
            .build());

        var defaultWorkstationCluster = new WorkstationCluster("defaultWorkstationCluster", WorkstationClusterArgs.builder()
            .workstationClusterId("workstation-cluster")
            .network(default_.id())
            .subnetwork(defaultSubnetwork.id())
            .location("us-central1")
            .labels(Map.of("label", "key"))
            .annotations(Map.of("label-one", "value-one"))
            .build());

        var defaultWorkstationConfig = new WorkstationConfig("defaultWorkstationConfig", WorkstationConfigArgs.builder()
            .workstationConfigId("workstation-config")
            .workstationClusterId(defaultWorkstationCluster.workstationClusterId())
            .location("us-central1")
            .idleTimeout("600s")
            .runningTimeout("21600s")
            .replicaZones(            
                "us-central1-a",
                "us-central1-b")
            .annotations(Map.of("label-one", "value-one"))
            .labels(Map.of("label", "key"))
            .maxUsableWorkstations(1)
            .host(WorkstationConfigHostArgs.builder()
                .gceInstance(WorkstationConfigHostGceInstanceArgs.builder()
                    .machineType("e2-standard-4")
                    .bootDiskSizeGb(35)
                    .disablePublicIpAddresses(true)
                    .disableSsh(false)
                    .vmTags(Output.tuple(tagKey1.id(), tagValue1.id()).applyValue(values -> {
                        var tagKey1Id = values.t1;
                        var tagValue1Id = values.t2;
                        return Map.of(tagKey1Id, tagValue1Id);
                    }))
                    .build())
                .build())
            .build());

    }
}
```
```yaml
resources:
  tagKey1:
    type: gcp:tags:TagKey
    name: tag_key1
    properties:
      parent: organizations/123456789
      shortName: keyname
  tagValue1:
    type: gcp:tags:TagValue
    name: tag_value1
    properties:
      parent: ${tagKey1.id}
      shortName: valuename
  default:
    type: gcp:compute:Network
    properties:
      name: workstation-cluster
      autoCreateSubnetworks: false
  defaultSubnetwork:
    type: gcp:compute:Subnetwork
    name: default
    properties:
      name: workstation-cluster
      ipCidrRange: 10.0.0.0/24
      region: us-central1
      network: ${default.name}
  defaultWorkstationCluster:
    type: gcp:workstations:WorkstationCluster
    name: default
    properties:
      workstationClusterId: workstation-cluster
      network: ${default.id}
      subnetwork: ${defaultSubnetwork.id}
      location: us-central1
      labels:
        label: key
      annotations:
        label-one: value-one
  defaultWorkstationConfig:
    type: gcp:workstations:WorkstationConfig
    name: default
    properties:
      workstationConfigId: workstation-config
      workstationClusterId: ${defaultWorkstationCluster.workstationClusterId}
      location: us-central1
      idleTimeout: 600s
      runningTimeout: 21600s
      replicaZones:
        - us-central1-a
        - us-central1-b
      annotations:
        label-one: value-one
      labels:
        label: key
      maxUsableWorkstations: 1
      host:
        gceInstance:
          machineType: e2-standard-4
          bootDiskSizeGb: 35
          disablePublicIpAddresses: true
          disableSsh: false
          vmTags:
            ${tagKey1.id}: ${tagValue1.id}
```
<!--End PulumiCodeChooser -->
### Workstation Config Container


<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as gcp from "@pulumi/gcp";

const _default = new gcp.compute.Network("default", {
    name: "workstation-cluster",
    autoCreateSubnetworks: false,
});
const defaultSubnetwork = new gcp.compute.Subnetwork("default", {
    name: "workstation-cluster",
    ipCidrRange: "10.0.0.0/24",
    region: "us-central1",
    network: _default.name,
});
const defaultWorkstationCluster = new gcp.workstations.WorkstationCluster("default", {
    workstationClusterId: "workstation-cluster",
    network: _default.id,
    subnetwork: defaultSubnetwork.id,
    location: "us-central1",
    labels: {
        label: "key",
    },
    annotations: {
        "label-one": "value-one",
    },
});
const defaultWorkstationConfig = new gcp.workstations.WorkstationConfig("default", {
    workstationConfigId: "workstation-config",
    workstationClusterId: defaultWorkstationCluster.workstationClusterId,
    location: "us-central1",
    host: {
        gceInstance: {
            machineType: "n1-standard-4",
            bootDiskSizeGb: 35,
            disablePublicIpAddresses: true,
            enableNestedVirtualization: true,
        },
    },
    container: {
        image: "intellij",
        env: {
            NAME: "FOO",
            BABE: "bar",
        },
    },
});
```
```python
import pulumi
import pulumi_gcp as gcp

default = gcp.compute.Network("default",
    name="workstation-cluster",
    auto_create_subnetworks=False)
default_subnetwork = gcp.compute.Subnetwork("default",
    name="workstation-cluster",
    ip_cidr_range="10.0.0.0/24",
    region="us-central1",
    network=default.name)
default_workstation_cluster = gcp.workstations.WorkstationCluster("default",
    workstation_cluster_id="workstation-cluster",
    network=default.id,
    subnetwork=default_subnetwork.id,
    location="us-central1",
    labels={
        "label": "key",
    },
    annotations={
        "label-one": "value-one",
    })
default_workstation_config = gcp.workstations.WorkstationConfig("default",
    workstation_config_id="workstation-config",
    workstation_cluster_id=default_workstation_cluster.workstation_cluster_id,
    location="us-central1",
    host={
        "gce_instance": {
            "machine_type": "n1-standard-4",
            "boot_disk_size_gb": 35,
            "disable_public_ip_addresses": True,
            "enable_nested_virtualization": True,
        },
    },
    container={
        "image": "intellij",
        "env": {
            "NAME": "FOO",
            "BABE": "bar",
        },
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Gcp = Pulumi.Gcp;

return await Deployment.RunAsync(() => 
{
    var @default = new Gcp.Compute.Network("default", new()
    {
        Name = "workstation-cluster",
        AutoCreateSubnetworks = false,
    });

    var defaultSubnetwork = new Gcp.Compute.Subnetwork("default", new()
    {
        Name = "workstation-cluster",
        IpCidrRange = "10.0.0.0/24",
        Region = "us-central1",
        Network = @default.Name,
    });

    var defaultWorkstationCluster = new Gcp.Workstations.WorkstationCluster("default", new()
    {
        WorkstationClusterId = "workstation-cluster",
        Network = @default.Id,
        Subnetwork = defaultSubnetwork.Id,
        Location = "us-central1",
        Labels = 
        {
            { "label", "key" },
        },
        Annotations = 
        {
            { "label-one", "value-one" },
        },
    });

    var defaultWorkstationConfig = new Gcp.Workstations.WorkstationConfig("default", new()
    {
        WorkstationConfigId = "workstation-config",
        WorkstationClusterId = defaultWorkstationCluster.WorkstationClusterId,
        Location = "us-central1",
        Host = new Gcp.Workstations.Inputs.WorkstationConfigHostArgs
        {
            GceInstance = new Gcp.Workstations.Inputs.WorkstationConfigHostGceInstanceArgs
            {
                MachineType = "n1-standard-4",
                BootDiskSizeGb = 35,
                DisablePublicIpAddresses = true,
                EnableNestedVirtualization = true,
            },
        },
        Container = new Gcp.Workstations.Inputs.WorkstationConfigContainerArgs
        {
            Image = "intellij",
            Env = 
            {
                { "NAME", "FOO" },
                { "BABE", "bar" },
            },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-gcp/sdk/v8/go/gcp/compute"
	"github.com/pulumi/pulumi-gcp/sdk/v8/go/gcp/workstations"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := compute.NewNetwork(ctx, "default", &compute.NetworkArgs{
			Name:                  pulumi.String("workstation-cluster"),
			AutoCreateSubnetworks: pulumi.Bool(false),
		})
		if err != nil {
			return err
		}
		defaultSubnetwork, err := compute.NewSubnetwork(ctx, "default", &compute.SubnetworkArgs{
			Name:        pulumi.String("workstation-cluster"),
			IpCidrRange: pulumi.String("10.0.0.0/24"),
			Region:      pulumi.String("us-central1"),
			Network:     _default.Name,
		})
		if err != nil {
			return err
		}
		defaultWorkstationCluster, err := workstations.NewWorkstationCluster(ctx, "default", &workstations.WorkstationClusterArgs{
			WorkstationClusterId: pulumi.String("workstation-cluster"),
			Network:              _default.ID(),
			Subnetwork:           defaultSubnetwork.ID(),
			Location:             pulumi.String("us-central1"),
			Labels: pulumi.StringMap{
				"label": pulumi.String("key"),
			},
			Annotations: pulumi.StringMap{
				"label-one": pulumi.String("value-one"),
			},
		})
		if err != nil {
			return err
		}
		_, err = workstations.NewWorkstationConfig(ctx, "default", &workstations.WorkstationConfigArgs{
			WorkstationConfigId:  pulumi.String("workstation-config"),
			WorkstationClusterId: defaultWorkstationCluster.WorkstationClusterId,
			Location:             pulumi.String("us-central1"),
			Host: &workstations.WorkstationConfigHostArgs{
				GceInstance: &workstations.WorkstationConfigHostGceInstanceArgs{
					MachineType:                pulumi.String("n1-standard-4"),
					BootDiskSizeGb:             pulumi.Int(35),
					DisablePublicIpAddresses:   pulumi.Bool(true),
					EnableNestedVirtualization: pulumi.Bool(true),
				},
			},
			Container: &workstations.WorkstationConfigContainerArgs{
				Image: pulumi.String("intellij"),
				Env: pulumi.StringMap{
					"NAME": pulumi.String("FOO"),
					"BABE": pulumi.String("bar"),
				},
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.gcp.compute.Network;
import com.pulumi.gcp.compute.NetworkArgs;
import com.pulumi.gcp.compute.Subnetwork;
import com.pulumi.gcp.compute.SubnetworkArgs;
import com.pulumi.gcp.workstations.WorkstationCluster;
import com.pulumi.gcp.workstations.WorkstationClusterArgs;
import com.pulumi.gcp.workstations.WorkstationConfig;
import com.pulumi.gcp.workstations.WorkstationConfigArgs;
import com.pulumi.gcp.workstations.inputs.WorkstationConfigHostArgs;
import com.pulumi.gcp.workstations.inputs.WorkstationConfigHostGceInstanceArgs;
import com.pulumi.gcp.workstations.inputs.WorkstationConfigContainerArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var default_ = new Network("default", NetworkArgs.builder()
            .name("workstation-cluster")
            .autoCreateSubnetworks(false)
            .build());

        var defaultSubnetwork = new Subnetwork("defaultSubnetwork", SubnetworkArgs.builder()
            .name("workstation-cluster")
            .ipCidrRange("10.0.0.0/24")
            .region("us-central1")
            .network(default_.name())
            .build());

        var defaultWorkstationCluster = new WorkstationCluster("defaultWorkstationCluster", WorkstationClusterArgs.builder()
            .workstationClusterId("workstation-cluster")
            .network(default_.id())
            .subnetwork(defaultSubnetwork.id())
            .location("us-central1")
            .labels(Map.of("label", "key"))
            .annotations(Map.of("label-one", "value-one"))
            .build());

        var defaultWorkstationConfig = new WorkstationConfig("defaultWorkstationConfig", WorkstationConfigArgs.builder()
            .workstationConfigId("workstation-config")
            .workstationClusterId(defaultWorkstationCluster.workstationClusterId())
            .location("us-central1")
            .host(WorkstationConfigHostArgs.builder()
                .gceInstance(WorkstationConfigHostGceInstanceArgs.builder()
                    .machineType("n1-standard-4")
                    .bootDiskSizeGb(35)
                    .disablePublicIpAddresses(true)
                    .enableNestedVirtualization(true)
                    .build())
                .build())
            .container(WorkstationConfigContainerArgs.builder()
                .image("intellij")
                .env(Map.ofEntries(
                    Map.entry("NAME", "FOO"),
                    Map.entry("BABE", "bar")
                ))
                .build())
            .build());

    }
}
```
```yaml
resources:
  default:
    type: gcp:compute:Network
    properties:
      name: workstation-cluster
      autoCreateSubnetworks: false
  defaultSubnetwork:
    type: gcp:compute:Subnetwork
    name: default
    properties:
      name: workstation-cluster
      ipCidrRange: 10.0.0.0/24
      region: us-central1
      network: ${default.name}
  defaultWorkstationCluster:
    type: gcp:workstations:WorkstationCluster
    name: default
    properties:
      workstationClusterId: workstation-cluster
      network: ${default.id}
      subnetwork: ${defaultSubnetwork.id}
      location: us-central1
      labels:
        label: key
      annotations:
        label-one: value-one
  defaultWorkstationConfig:
    type: gcp:workstations:WorkstationConfig
    name: default
    properties:
      workstationConfigId: workstation-config
      workstationClusterId: ${defaultWorkstationCluster.workstationClusterId}
      location: us-central1
      host:
        gceInstance:
          machineType: n1-standard-4
          bootDiskSizeGb: 35
          disablePublicIpAddresses: true
          enableNestedVirtualization: true
      container:
        image: intellij
        env:
          NAME: FOO
          BABE: bar
```
<!--End PulumiCodeChooser -->
### Workstation Config Persistent Directories


<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as gcp from "@pulumi/gcp";

const _default = new gcp.compute.Network("default", {
    name: "workstation-cluster",
    autoCreateSubnetworks: false,
});
const defaultSubnetwork = new gcp.compute.Subnetwork("default", {
    name: "workstation-cluster",
    ipCidrRange: "10.0.0.0/24",
    region: "us-central1",
    network: _default.name,
});
const defaultWorkstationCluster = new gcp.workstations.WorkstationCluster("default", {
    workstationClusterId: "workstation-cluster",
    network: _default.id,
    subnetwork: defaultSubnetwork.id,
    location: "us-central1",
    labels: {
        label: "key",
    },
    annotations: {
        "label-one": "value-one",
    },
});
const defaultWorkstationConfig = new gcp.workstations.WorkstationConfig("default", {
    workstationConfigId: "workstation-config",
    workstationClusterId: defaultWorkstationCluster.workstationClusterId,
    location: "us-central1",
    host: {
        gceInstance: {
            machineType: "e2-standard-4",
            bootDiskSizeGb: 35,
            disablePublicIpAddresses: true,
            shieldedInstanceConfig: {
                enableSecureBoot: true,
                enableVtpm: true,
            },
        },
    },
    persistentDirectories: [{
        mountPath: "/home",
        gcePd: {
            sizeGb: 200,
            fsType: "ext4",
            diskType: "pd-standard",
            reclaimPolicy: "DELETE",
        },
    }],
});
```
```python
import pulumi
import pulumi_gcp as gcp

default = gcp.compute.Network("default",
    name="workstation-cluster",
    auto_create_subnetworks=False)
default_subnetwork = gcp.compute.Subnetwork("default",
    name="workstation-cluster",
    ip_cidr_range="10.0.0.0/24",
    region="us-central1",
    network=default.name)
default_workstation_cluster = gcp.workstations.WorkstationCluster("default",
    workstation_cluster_id="workstation-cluster",
    network=default.id,
    subnetwork=default_subnetwork.id,
    location="us-central1",
    labels={
        "label": "key",
    },
    annotations={
        "label-one": "value-one",
    })
default_workstation_config = gcp.workstations.WorkstationConfig("default",
    workstation_config_id="workstation-config",
    workstation_cluster_id=default_workstation_cluster.workstation_cluster_id,
    location="us-central1",
    host={
        "gce_instance": {
            "machine_type": "e2-standard-4",
            "boot_disk_size_gb": 35,
            "disable_public_ip_addresses": True,
            "shielded_instance_config": {
                "enable_secure_boot": True,
                "enable_vtpm": True,
            },
        },
    },
    persistent_directories=[{
        "mount_path": "/home",
        "gce_pd": {
            "size_gb": 200,
            "fs_type": "ext4",
            "disk_type": "pd-standard",
            "reclaim_policy": "DELETE",
        },
    }])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Gcp = Pulumi.Gcp;

return await Deployment.RunAsync(() => 
{
    var @default = new Gcp.Compute.Network("default", new()
    {
        Name = "workstation-cluster",
        AutoCreateSubnetworks = false,
    });

    var defaultSubnetwork = new Gcp.Compute.Subnetwork("default", new()
    {
        Name = "workstation-cluster",
        IpCidrRange = "10.0.0.0/24",
        Region = "us-central1",
        Network = @default.Name,
    });

    var defaultWorkstationCluster = new Gcp.Workstations.WorkstationCluster("default", new()
    {
        WorkstationClusterId = "workstation-cluster",
        Network = @default.Id,
        Subnetwork = defaultSubnetwork.Id,
        Location = "us-central1",
        Labels = 
        {
            { "label", "key" },
        },
        Annotations = 
        {
            { "label-one", "value-one" },
        },
    });

    var defaultWorkstationConfig = new Gcp.Workstations.WorkstationConfig("default", new()
    {
        WorkstationConfigId = "workstation-config",
        WorkstationClusterId = defaultWorkstationCluster.WorkstationClusterId,
        Location = "us-central1",
        Host = new Gcp.Workstations.Inputs.WorkstationConfigHostArgs
        {
            GceInstance = new Gcp.Workstations.Inputs.WorkstationConfigHostGceInstanceArgs
            {
                MachineType = "e2-standard-4",
                BootDiskSizeGb = 35,
                DisablePublicIpAddresses = true,
                ShieldedInstanceConfig = new Gcp.Workstations.Inputs.WorkstationConfigHostGceInstanceShieldedInstanceConfigArgs
                {
                    EnableSecureBoot = true,
                    EnableVtpm = true,
                },
            },
        },
        PersistentDirectories = new[]
        {
            new Gcp.Workstations.Inputs.WorkstationConfigPersistentDirectoryArgs
            {
                MountPath = "/home",
                GcePd = new Gcp.Workstations.Inputs.WorkstationConfigPersistentDirectoryGcePdArgs
                {
                    SizeGb = 200,
                    FsType = "ext4",
                    DiskType = "pd-standard",
                    ReclaimPolicy = "DELETE",
                },
            },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-gcp/sdk/v8/go/gcp/compute"
	"github.com/pulumi/pulumi-gcp/sdk/v8/go/gcp/workstations"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := compute.NewNetwork(ctx, "default", &compute.NetworkArgs{
			Name:                  pulumi.String("workstation-cluster"),
			AutoCreateSubnetworks: pulumi.Bool(false),
		})
		if err != nil {
			return err
		}
		defaultSubnetwork, err := compute.NewSubnetwork(ctx, "default", &compute.SubnetworkArgs{
			Name:        pulumi.String("workstation-cluster"),
			IpCidrRange: pulumi.String("10.0.0.0/24"),
			Region:      pulumi.String("us-central1"),
			Network:     _default.Name,
		})
		if err != nil {
			return err
		}
		defaultWorkstationCluster, err := workstations.NewWorkstationCluster(ctx, "default", &workstations.WorkstationClusterArgs{
			WorkstationClusterId: pulumi.String("workstation-cluster"),
			Network:              _default.ID(),
			Subnetwork:           defaultSubnetwork.ID(),
			Location:             pulumi.String("us-central1"),
			Labels: pulumi.StringMap{
				"label": pulumi.String("key"),
			},
			Annotations: pulumi.StringMap{
				"label-one": pulumi.String("value-one"),
			},
		})
		if err != nil {
			return err
		}
		_, err = workstations.NewWorkstationConfig(ctx, "default", &workstations.WorkstationConfigArgs{
			WorkstationConfigId:  pulumi.String("workstation-config"),
			WorkstationClusterId: defaultWorkstationCluster.WorkstationClusterId,
			Location:             pulumi.String("us-central1"),
			Host: &workstations.WorkstationConfigHostArgs{
				GceInstance: &workstations.WorkstationConfigHostGceInstanceArgs{
					MachineType:              pulumi.String("e2-standard-4"),
					BootDiskSizeGb:           pulumi.Int(35),
					DisablePublicIpAddresses: pulumi.Bool(true),
					ShieldedInstanceConfig: &workstations.WorkstationConfigHostGceInstanceShieldedInstanceConfigArgs{
						EnableSecureBoot: pulumi.Bool(true),
						EnableVtpm:       pulumi.Bool(true),
					},
				},
			},
			PersistentDirectories: workstations.WorkstationConfigPersistentDirectoryArray{
				&workstations.WorkstationConfigPersistentDirectoryArgs{
					MountPath: pulumi.String("/home"),
					GcePd: &workstations.WorkstationConfigPersistentDirectoryGcePdArgs{
						SizeGb:        pulumi.Int(200),
						FsType:        pulumi.String("ext4"),
						DiskType:      pulumi.String("pd-standard"),
						ReclaimPolicy: pulumi.String("DELETE"),
					},
				},
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.gcp.compute.Network;
import com.pulumi.gcp.compute.NetworkArgs;
import com.pulumi.gcp.compute.Subnetwork;
import com.pulumi.gcp.compute.SubnetworkArgs;
import com.pulumi.gcp.workstations.WorkstationCluster;
import com.pulumi.gcp.workstations.WorkstationClusterArgs;
import com.pulumi.gcp.workstations.WorkstationConfig;
import com.pulumi.gcp.workstations.WorkstationConfigArgs;
import com.pulumi.gcp.workstations.inputs.WorkstationConfigHostArgs;
import com.pulumi.gcp.workstations.inputs.WorkstationConfigHostGceInstanceArgs;
import com.pulumi.gcp.workstations.inputs.WorkstationConfigHostGceInstanceShieldedInstanceConfigArgs;
import com.pulumi.gcp.workstations.inputs.WorkstationConfigPersistentDirectoryArgs;
import com.pulumi.gcp.workstations.inputs.WorkstationConfigPersistentDirectoryGcePdArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var default_ = new Network("default", NetworkArgs.builder()
            .name("workstation-cluster")
            .autoCreateSubnetworks(false)
            .build());

        var defaultSubnetwork = new Subnetwork("defaultSubnetwork", SubnetworkArgs.builder()
            .name("workstation-cluster")
            .ipCidrRange("10.0.0.0/24")
            .region("us-central1")
            .network(default_.name())
            .build());

        var defaultWorkstationCluster = new WorkstationCluster("defaultWorkstationCluster", WorkstationClusterArgs.builder()
            .workstationClusterId("workstation-cluster")
            .network(default_.id())
            .subnetwork(defaultSubnetwork.id())
            .location("us-central1")
            .labels(Map.of("label", "key"))
            .annotations(Map.of("label-one", "value-one"))
            .build());

        var defaultWorkstationConfig = new WorkstationConfig("defaultWorkstationConfig", WorkstationConfigArgs.builder()
            .workstationConfigId("workstation-config")
            .workstationClusterId(defaultWorkstationCluster.workstationClusterId())
            .location("us-central1")
            .host(WorkstationConfigHostArgs.builder()
                .gceInstance(WorkstationConfigHostGceInstanceArgs.builder()
                    .machineType("e2-standard-4")
                    .bootDiskSizeGb(35)
                    .disablePublicIpAddresses(true)
                    .shieldedInstanceConfig(WorkstationConfigHostGceInstanceShieldedInstanceConfigArgs.builder()
                        .enableSecureBoot(true)
                        .enableVtpm(true)
                        .build())
                    .build())
                .build())
            .persistentDirectories(WorkstationConfigPersistentDirectoryArgs.builder()
                .mountPath("/home")
                .gcePd(WorkstationConfigPersistentDirectoryGcePdArgs.builder()
                    .sizeGb(200)
                    .fsType("ext4")
                    .diskType("pd-standard")
                    .reclaimPolicy("DELETE")
                    .build())
                .build())
            .build());

    }
}
```
```yaml
resources:
  default:
    type: gcp:compute:Network
    properties:
      name: workstation-cluster
      autoCreateSubnetworks: false
  defaultSubnetwork:
    type: gcp:compute:Subnetwork
    name: default
    properties:
      name: workstation-cluster
      ipCidrRange: 10.0.0.0/24
      region: us-central1
      network: ${default.name}
  defaultWorkstationCluster:
    type: gcp:workstations:WorkstationCluster
    name: default
    properties:
      workstationClusterId: workstation-cluster
      network: ${default.id}
      subnetwork: ${defaultSubnetwork.id}
      location: us-central1
      labels:
        label: key
      annotations:
        label-one: value-one
  defaultWorkstationConfig:
    type: gcp:workstations:WorkstationConfig
    name: default
    properties:
      workstationConfigId: workstation-config
      workstationClusterId: ${defaultWorkstationCluster.workstationClusterId}
      location: us-central1
      host:
        gceInstance:
          machineType: e2-standard-4
          bootDiskSizeGb: 35
          disablePublicIpAddresses: true
          shieldedInstanceConfig:
            enableSecureBoot: true
            enableVtpm: true
      persistentDirectories:
        - mountPath: /home
          gcePd:
            sizeGb: 200
            fsType: ext4
            diskType: pd-standard
            reclaimPolicy: DELETE
```
<!--End PulumiCodeChooser -->
### Workstation Config Source Snapshot


<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as gcp from "@pulumi/gcp";

const _default = new gcp.compute.Network("default", {
    name: "workstation-cluster",
    autoCreateSubnetworks: false,
});
const defaultSubnetwork = new gcp.compute.Subnetwork("default", {
    name: "workstation-cluster",
    ipCidrRange: "10.0.0.0/24",
    region: "us-central1",
    network: _default.name,
});
const mySourceDisk = new gcp.compute.Disk("my_source_disk", {
    name: "workstation-config",
    size: 10,
    type: "pd-ssd",
    zone: "us-central1-a",
});
const mySourceSnapshot = new gcp.compute.Snapshot("my_source_snapshot", {
    name: "workstation-config",
    sourceDisk: mySourceDisk.name,
    zone: "us-central1-a",
});
const defaultWorkstationCluster = new gcp.workstations.WorkstationCluster("default", {
    workstationClusterId: "workstation-cluster",
    network: _default.id,
    subnetwork: defaultSubnetwork.id,
    location: "us-central1",
});
const defaultWorkstationConfig = new gcp.workstations.WorkstationConfig("default", {
    workstationConfigId: "workstation-config",
    workstationClusterId: defaultWorkstationCluster.workstationClusterId,
    location: defaultWorkstationCluster.location,
    persistentDirectories: [{
        mountPath: "/home",
        gcePd: {
            sourceSnapshot: mySourceSnapshot.id,
            reclaimPolicy: "DELETE",
        },
    }],
});
```
```python
import pulumi
import pulumi_gcp as gcp

default = gcp.compute.Network("default",
    name="workstation-cluster",
    auto_create_subnetworks=False)
default_subnetwork = gcp.compute.Subnetwork("default",
    name="workstation-cluster",
    ip_cidr_range="10.0.0.0/24",
    region="us-central1",
    network=default.name)
my_source_disk = gcp.compute.Disk("my_source_disk",
    name="workstation-config",
    size=10,
    type="pd-ssd",
    zone="us-central1-a")
my_source_snapshot = gcp.compute.Snapshot("my_source_snapshot",
    name="workstation-config",
    source_disk=my_source_disk.name,
    zone="us-central1-a")
default_workstation_cluster = gcp.workstations.WorkstationCluster("default",
    workstation_cluster_id="workstation-cluster",
    network=default.id,
    subnetwork=default_subnetwork.id,
    location="us-central1")
default_workstation_config = gcp.workstations.WorkstationConfig("default",
    workstation_config_id="workstation-config",
    workstation_cluster_id=default_workstation_cluster.workstation_cluster_id,
    location=default_workstation_cluster.location,
    persistent_directories=[{
        "mount_path": "/home",
        "gce_pd": {
            "source_snapshot": my_source_snapshot.id,
            "reclaim_policy": "DELETE",
        },
    }])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Gcp = Pulumi.Gcp;

return await Deployment.RunAsync(() => 
{
    var @default = new Gcp.Compute.Network("default", new()
    {
        Name = "workstation-cluster",
        AutoCreateSubnetworks = false,
    });

    var defaultSubnetwork = new Gcp.Compute.Subnetwork("default", new()
    {
        Name = "workstation-cluster",
        IpCidrRange = "10.0.0.0/24",
        Region = "us-central1",
        Network = @default.Name,
    });

    var mySourceDisk = new Gcp.Compute.Disk("my_source_disk", new()
    {
        Name = "workstation-config",
        Size = 10,
        Type = "pd-ssd",
        Zone = "us-central1-a",
    });

    var mySourceSnapshot = new Gcp.Compute.Snapshot("my_source_snapshot", new()
    {
        Name = "workstation-config",
        SourceDisk = mySourceDisk.Name,
        Zone = "us-central1-a",
    });

    var defaultWorkstationCluster = new Gcp.Workstations.WorkstationCluster("default", new()
    {
        WorkstationClusterId = "workstation-cluster",
        Network = @default.Id,
        Subnetwork = defaultSubnetwork.Id,
        Location = "us-central1",
    });

    var defaultWorkstationConfig = new Gcp.Workstations.WorkstationConfig("default", new()
    {
        WorkstationConfigId = "workstation-config",
        WorkstationClusterId = defaultWorkstationCluster.WorkstationClusterId,
        Location = defaultWorkstationCluster.Location,
        PersistentDirectories = new[]
        {
            new Gcp.Workstations.Inputs.WorkstationConfigPersistentDirectoryArgs
            {
                MountPath = "/home",
                GcePd = new Gcp.Workstations.Inputs.WorkstationConfigPersistentDirectoryGcePdArgs
                {
                    SourceSnapshot = mySourceSnapshot.Id,
                    ReclaimPolicy = "DELETE",
                },
            },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-gcp/sdk/v8/go/gcp/compute"
	"github.com/pulumi/pulumi-gcp/sdk/v8/go/gcp/workstations"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := compute.NewNetwork(ctx, "default", &compute.NetworkArgs{
			Name:                  pulumi.String("workstation-cluster"),
			AutoCreateSubnetworks: pulumi.Bool(false),
		})
		if err != nil {
			return err
		}
		defaultSubnetwork, err := compute.NewSubnetwork(ctx, "default", &compute.SubnetworkArgs{
			Name:        pulumi.String("workstation-cluster"),
			IpCidrRange: pulumi.String("10.0.0.0/24"),
			Region:      pulumi.String("us-central1"),
			Network:     _default.Name,
		})
		if err != nil {
			return err
		}
		mySourceDisk, err := compute.NewDisk(ctx, "my_source_disk", &compute.DiskArgs{
			Name: pulumi.String("workstation-config"),
			Size: pulumi.Int(10),
			Type: pulumi.String("pd-ssd"),
			Zone: pulumi.String("us-central1-a"),
		})
		if err != nil {
			return err
		}
		mySourceSnapshot, err := compute.NewSnapshot(ctx, "my_source_snapshot", &compute.SnapshotArgs{
			Name:       pulumi.String("workstation-config"),
			SourceDisk: mySourceDisk.Name,
			Zone:       pulumi.String("us-central1-a"),
		})
		if err != nil {
			return err
		}
		defaultWorkstationCluster, err := workstations.NewWorkstationCluster(ctx, "default", &workstations.WorkstationClusterArgs{
			WorkstationClusterId: pulumi.String("workstation-cluster"),
			Network:              _default.ID(),
			Subnetwork:           defaultSubnetwork.ID(),
			Location:             pulumi.String("us-central1"),
		})
		if err != nil {
			return err
		}
		_, err = workstations.NewWorkstationConfig(ctx, "default", &workstations.WorkstationConfigArgs{
			WorkstationConfigId:  pulumi.String("workstation-config"),
			WorkstationClusterId: defaultWorkstationCluster.WorkstationClusterId,
			Location:             defaultWorkstationCluster.Location,
			PersistentDirectories: workstations.WorkstationConfigPersistentDirectoryArray{
				&workstations.WorkstationConfigPersistentDirectoryArgs{
					MountPath: pulumi.String("/home"),
					GcePd: &workstations.WorkstationConfigPersistentDirectoryGcePdArgs{
						SourceSnapshot: mySourceSnapshot.ID(),
						ReclaimPolicy:  pulumi.String("DELETE"),
					},
				},
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.gcp.compute.Network;
import com.pulumi.gcp.compute.NetworkArgs;
import com.pulumi.gcp.compute.Subnetwork;
import com.pulumi.gcp.compute.SubnetworkArgs;
import com.pulumi.gcp.compute.Disk;
import com.pulumi.gcp.compute.DiskArgs;
import com.pulumi.gcp.compute.Snapshot;
import com.pulumi.gcp.compute.SnapshotArgs;
import com.pulumi.gcp.workstations.WorkstationCluster;
import com.pulumi.gcp.workstations.WorkstationClusterArgs;
import com.pulumi.gcp.workstations.WorkstationConfig;
import com.pulumi.gcp.workstations.WorkstationConfigArgs;
import com.pulumi.gcp.workstations.inputs.WorkstationConfigPersistentDirectoryArgs;
import com.pulumi.gcp.workstations.inputs.WorkstationConfigPersistentDirectoryGcePdArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var default_ = new Network("default", NetworkArgs.builder()
            .name("workstation-cluster")
            .autoCreateSubnetworks(false)
            .build());

        var defaultSubnetwork = new Subnetwork("defaultSubnetwork", SubnetworkArgs.builder()
            .name("workstation-cluster")
            .ipCidrRange("10.0.0.0/24")
            .region("us-central1")
            .network(default_.name())
            .build());

        var mySourceDisk = new Disk("mySourceDisk", DiskArgs.builder()
            .name("workstation-config")
            .size(10)
            .type("pd-ssd")
            .zone("us-central1-a")
            .build());

        var mySourceSnapshot = new Snapshot("mySourceSnapshot", SnapshotArgs.builder()
            .name("workstation-config")
            .sourceDisk(mySourceDisk.name())
            .zone("us-central1-a")
            .build());

        var defaultWorkstationCluster = new WorkstationCluster("defaultWorkstationCluster", WorkstationClusterArgs.builder()
            .workstationClusterId("workstation-cluster")
            .network(default_.id())
            .subnetwork(defaultSubnetwork.id())
            .location("us-central1")
            .build());

        var defaultWorkstationConfig = new WorkstationConfig("defaultWorkstationConfig", WorkstationConfigArgs.builder()
            .workstationConfigId("workstation-config")
            .workstationClusterId(defaultWorkstationCluster.workstationClusterId())
            .location(defaultWorkstationCluster.location())
            .persistentDirectories(WorkstationConfigPersistentDirectoryArgs.builder()
                .mountPath("/home")
                .gcePd(WorkstationConfigPersistentDirectoryGcePdArgs.builder()
                    .sourceSnapshot(mySourceSnapshot.id())
                    .reclaimPolicy("DELETE")
                    .build())
                .build())
            .build());

    }
}
```
```yaml
resources:
  default:
    type: gcp:compute:Network
    properties:
      name: workstation-cluster
      autoCreateSubnetworks: false
  defaultSubnetwork:
    type: gcp:compute:Subnetwork
    name: default
    properties:
      name: workstation-cluster
      ipCidrRange: 10.0.0.0/24
      region: us-central1
      network: ${default.name}
  mySourceDisk:
    type: gcp:compute:Disk
    name: my_source_disk
    properties:
      name: workstation-config
      size: 10
      type: pd-ssd
      zone: us-central1-a
  mySourceSnapshot:
    type: gcp:compute:Snapshot
    name: my_source_snapshot
    properties:
      name: workstation-config
      sourceDisk: ${mySourceDisk.name}
      zone: us-central1-a
  defaultWorkstationCluster:
    type: gcp:workstations:WorkstationCluster
    name: default
    properties:
      workstationClusterId: workstation-cluster
      network: ${default.id}
      subnetwork: ${defaultSubnetwork.id}
      location: us-central1
  defaultWorkstationConfig:
    type: gcp:workstations:WorkstationConfig
    name: default
    properties:
      workstationConfigId: workstation-config
      workstationClusterId: ${defaultWorkstationCluster.workstationClusterId}
      location: ${defaultWorkstationCluster.location}
      persistentDirectories:
        - mountPath: /home
          gcePd:
            sourceSnapshot: ${mySourceSnapshot.id}
            reclaimPolicy: DELETE
```
<!--End PulumiCodeChooser -->
### Workstation Config Shielded Instance Config


<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as gcp from "@pulumi/gcp";

const _default = new gcp.compute.Network("default", {
    name: "workstation-cluster",
    autoCreateSubnetworks: false,
});
const defaultSubnetwork = new gcp.compute.Subnetwork("default", {
    name: "workstation-cluster",
    ipCidrRange: "10.0.0.0/24",
    region: "us-central1",
    network: _default.name,
});
const defaultWorkstationCluster = new gcp.workstations.WorkstationCluster("default", {
    workstationClusterId: "workstation-cluster",
    network: _default.id,
    subnetwork: defaultSubnetwork.id,
    location: "us-central1",
    labels: {
        label: "key",
    },
    annotations: {
        "label-one": "value-one",
    },
});
const defaultWorkstationConfig = new gcp.workstations.WorkstationConfig("default", {
    workstationConfigId: "workstation-config",
    workstationClusterId: defaultWorkstationCluster.workstationClusterId,
    location: "us-central1",
    host: {
        gceInstance: {
            machineType: "e2-standard-4",
            bootDiskSizeGb: 35,
            disablePublicIpAddresses: true,
            shieldedInstanceConfig: {
                enableSecureBoot: true,
                enableVtpm: true,
            },
        },
    },
});
```
```python
import pulumi
import pulumi_gcp as gcp

default = gcp.compute.Network("default",
    name="workstation-cluster",
    auto_create_subnetworks=False)
default_subnetwork = gcp.compute.Subnetwork("default",
    name="workstation-cluster",
    ip_cidr_range="10.0.0.0/24",
    region="us-central1",
    network=default.name)
default_workstation_cluster = gcp.workstations.WorkstationCluster("default",
    workstation_cluster_id="workstation-cluster",
    network=default.id,
    subnetwork=default_subnetwork.id,
    location="us-central1",
    labels={
        "label": "key",
    },
    annotations={
        "label-one": "value-one",
    })
default_workstation_config = gcp.workstations.WorkstationConfig("default",
    workstation_config_id="workstation-config",
    workstation_cluster_id=default_workstation_cluster.workstation_cluster_id,
    location="us-central1",
    host={
        "gce_instance": {
            "machine_type": "e2-standard-4",
            "boot_disk_size_gb": 35,
            "disable_public_ip_addresses": True,
            "shielded_instance_config": {
                "enable_secure_boot": True,
                "enable_vtpm": True,
            },
        },
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Gcp = Pulumi.Gcp;

return await Deployment.RunAsync(() => 
{
    var @default = new Gcp.Compute.Network("default", new()
    {
        Name = "workstation-cluster",
        AutoCreateSubnetworks = false,
    });

    var defaultSubnetwork = new Gcp.Compute.Subnetwork("default", new()
    {
        Name = "workstation-cluster",
        IpCidrRange = "10.0.0.0/24",
        Region = "us-central1",
        Network = @default.Name,
    });

    var defaultWorkstationCluster = new Gcp.Workstations.WorkstationCluster("default", new()
    {
        WorkstationClusterId = "workstation-cluster",
        Network = @default.Id,
        Subnetwork = defaultSubnetwork.Id,
        Location = "us-central1",
        Labels = 
        {
            { "label", "key" },
        },
        Annotations = 
        {
            { "label-one", "value-one" },
        },
    });

    var defaultWorkstationConfig = new Gcp.Workstations.WorkstationConfig("default", new()
    {
        WorkstationConfigId = "workstation-config",
        WorkstationClusterId = defaultWorkstationCluster.WorkstationClusterId,
        Location = "us-central1",
        Host = new Gcp.Workstations.Inputs.WorkstationConfigHostArgs
        {
            GceInstance = new Gcp.Workstations.Inputs.WorkstationConfigHostGceInstanceArgs
            {
                MachineType = "e2-standard-4",
                BootDiskSizeGb = 35,
                DisablePublicIpAddresses = true,
                ShieldedInstanceConfig = new Gcp.Workstations.Inputs.WorkstationConfigHostGceInstanceShieldedInstanceConfigArgs
                {
                    EnableSecureBoot = true,
                    EnableVtpm = true,
                },
            },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-gcp/sdk/v8/go/gcp/compute"
	"github.com/pulumi/pulumi-gcp/sdk/v8/go/gcp/workstations"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := compute.NewNetwork(ctx, "default", &compute.NetworkArgs{
			Name:                  pulumi.String("workstation-cluster"),
			AutoCreateSubnetworks: pulumi.Bool(false),
		})
		if err != nil {
			return err
		}
		defaultSubnetwork, err := compute.NewSubnetwork(ctx, "default", &compute.SubnetworkArgs{
			Name:        pulumi.String("workstation-cluster"),
			IpCidrRange: pulumi.String("10.0.0.0/24"),
			Region:      pulumi.String("us-central1"),
			Network:     _default.Name,
		})
		if err != nil {
			return err
		}
		defaultWorkstationCluster, err := workstations.NewWorkstationCluster(ctx, "default", &workstations.WorkstationClusterArgs{
			WorkstationClusterId: pulumi.String("workstation-cluster"),
			Network:              _default.ID(),
			Subnetwork:           defaultSubnetwork.ID(),
			Location:             pulumi.String("us-central1"),
			Labels: pulumi.StringMap{
				"label": pulumi.String("key"),
			},
			Annotations: pulumi.StringMap{
				"label-one": pulumi.String("value-one"),
			},
		})
		if err != nil {
			return err
		}
		_, err = workstations.NewWorkstationConfig(ctx, "default", &workstations.WorkstationConfigArgs{
			WorkstationConfigId:  pulumi.String("workstation-config"),
			WorkstationClusterId: defaultWorkstationCluster.WorkstationClusterId,
			Location:             pulumi.String("us-central1"),
			Host: &workstations.WorkstationConfigHostArgs{
				GceInstance: &workstations.WorkstationConfigHostGceInstanceArgs{
					MachineType:              pulumi.String("e2-standard-4"),
					BootDiskSizeGb:           pulumi.Int(35),
					DisablePublicIpAddresses: pulumi.Bool(true),
					ShieldedInstanceConfig: &workstations.WorkstationConfigHostGceInstanceShieldedInstanceConfigArgs{
						EnableSecureBoot: pulumi.Bool(true),
						EnableVtpm:       pulumi.Bool(true),
					},
				},
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.gcp.compute.Network;
import com.pulumi.gcp.compute.NetworkArgs;
import com.pulumi.gcp.compute.Subnetwork;
import com.pulumi.gcp.compute.SubnetworkArgs;
import com.pulumi.gcp.workstations.WorkstationCluster;
import com.pulumi.gcp.workstations.WorkstationClusterArgs;
import com.pulumi.gcp.workstations.WorkstationConfig;
import com.pulumi.gcp.workstations.WorkstationConfigArgs;
import com.pulumi.gcp.workstations.inputs.WorkstationConfigHostArgs;
import com.pulumi.gcp.workstations.inputs.WorkstationConfigHostGceInstanceArgs;
import com.pulumi.gcp.workstations.inputs.WorkstationConfigHostGceInstanceShieldedInstanceConfigArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var default_ = new Network("default", NetworkArgs.builder()
            .name("workstation-cluster")
            .autoCreateSubnetworks(false)
            .build());

        var defaultSubnetwork = new Subnetwork("defaultSubnetwork", SubnetworkArgs.builder()
            .name("workstation-cluster")
            .ipCidrRange("10.0.0.0/24")
            .region("us-central1")
            .network(default_.name())
            .build());

        var defaultWorkstationCluster = new WorkstationCluster("defaultWorkstationCluster", WorkstationClusterArgs.builder()
            .workstationClusterId("workstation-cluster")
            .network(default_.id())
            .subnetwork(defaultSubnetwork.id())
            .location("us-central1")
            .labels(Map.of("label", "key"))
            .annotations(Map.of("label-one", "value-one"))
            .build());

        var defaultWorkstationConfig = new WorkstationConfig("defaultWorkstationConfig", WorkstationConfigArgs.builder()
            .workstationConfigId("workstation-config")
            .workstationClusterId(defaultWorkstationCluster.workstationClusterId())
            .location("us-central1")
            .host(WorkstationConfigHostArgs.builder()
                .gceInstance(WorkstationConfigHostGceInstanceArgs.builder()
                    .machineType("e2-standard-4")
                    .bootDiskSizeGb(35)
                    .disablePublicIpAddresses(true)
                    .shieldedInstanceConfig(WorkstationConfigHostGceInstanceShieldedInstanceConfigArgs.builder()
                        .enableSecureBoot(true)
                        .enableVtpm(true)
                        .build())
                    .build())
                .build())
            .build());

    }
}
```
```yaml
resources:
  default:
    type: gcp:compute:Network
    properties:
      name: workstation-cluster
      autoCreateSubnetworks: false
  defaultSubnetwork:
    type: gcp:compute:Subnetwork
    name: default
    properties:
      name: workstation-cluster
      ipCidrRange: 10.0.0.0/24
      region: us-central1
      network: ${default.name}
  defaultWorkstationCluster:
    type: gcp:workstations:WorkstationCluster
    name: default
    properties:
      workstationClusterId: workstation-cluster
      network: ${default.id}
      subnetwork: ${defaultSubnetwork.id}
      location: us-central1
      labels:
        label: key
      annotations:
        label-one: value-one
  defaultWorkstationConfig:
    type: gcp:workstations:WorkstationConfig
    name: default
    properties:
      workstationConfigId: workstation-config
      workstationClusterId: ${defaultWorkstationCluster.workstationClusterId}
      location: us-central1
      host:
        gceInstance:
          machineType: e2-standard-4
          bootDiskSizeGb: 35
          disablePublicIpAddresses: true
          shieldedInstanceConfig:
            enableSecureBoot: true
            enableVtpm: true
```
<!--End PulumiCodeChooser -->
### Workstation Config Accelerators


<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as gcp from "@pulumi/gcp";

const _default = new gcp.compute.Network("default", {
    name: "workstation-cluster",
    autoCreateSubnetworks: false,
});
const defaultSubnetwork = new gcp.compute.Subnetwork("default", {
    name: "workstation-cluster",
    ipCidrRange: "10.0.0.0/24",
    region: "us-central1",
    network: _default.name,
});
const defaultWorkstationCluster = new gcp.workstations.WorkstationCluster("default", {
    workstationClusterId: "workstation-cluster",
    network: _default.id,
    subnetwork: defaultSubnetwork.id,
    location: "us-central1",
    labels: {
        label: "key",
    },
    annotations: {
        "label-one": "value-one",
    },
});
const defaultWorkstationConfig = new gcp.workstations.WorkstationConfig("default", {
    workstationConfigId: "workstation-config",
    workstationClusterId: defaultWorkstationCluster.workstationClusterId,
    location: "us-central1",
    host: {
        gceInstance: {
            machineType: "n1-standard-2",
            bootDiskSizeGb: 35,
            disablePublicIpAddresses: true,
            accelerators: [{
                type: "nvidia-tesla-t4",
                count: 1,
            }],
        },
    },
});
```
```python
import pulumi
import pulumi_gcp as gcp

default = gcp.compute.Network("default",
    name="workstation-cluster",
    auto_create_subnetworks=False)
default_subnetwork = gcp.compute.Subnetwork("default",
    name="workstation-cluster",
    ip_cidr_range="10.0.0.0/24",
    region="us-central1",
    network=default.name)
default_workstation_cluster = gcp.workstations.WorkstationCluster("default",
    workstation_cluster_id="workstation-cluster",
    network=default.id,
    subnetwork=default_subnetwork.id,
    location="us-central1",
    labels={
        "label": "key",
    },
    annotations={
        "label-one": "value-one",
    })
default_workstation_config = gcp.workstations.WorkstationConfig("default",
    workstation_config_id="workstation-config",
    workstation_cluster_id=default_workstation_cluster.workstation_cluster_id,
    location="us-central1",
    host={
        "gce_instance": {
            "machine_type": "n1-standard-2",
            "boot_disk_size_gb": 35,
            "disable_public_ip_addresses": True,
            "accelerators": [{
                "type": "nvidia-tesla-t4",
                "count": 1,
            }],
        },
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Gcp = Pulumi.Gcp;

return await Deployment.RunAsync(() => 
{
    var @default = new Gcp.Compute.Network("default", new()
    {
        Name = "workstation-cluster",
        AutoCreateSubnetworks = false,
    });

    var defaultSubnetwork = new Gcp.Compute.Subnetwork("default", new()
    {
        Name = "workstation-cluster",
        IpCidrRange = "10.0.0.0/24",
        Region = "us-central1",
        Network = @default.Name,
    });

    var defaultWorkstationCluster = new Gcp.Workstations.WorkstationCluster("default", new()
    {
        WorkstationClusterId = "workstation-cluster",
        Network = @default.Id,
        Subnetwork = defaultSubnetwork.Id,
        Location = "us-central1",
        Labels = 
        {
            { "label", "key" },
        },
        Annotations = 
        {
            { "label-one", "value-one" },
        },
    });

    var defaultWorkstationConfig = new Gcp.Workstations.WorkstationConfig("default", new()
    {
        WorkstationConfigId = "workstation-config",
        WorkstationClusterId = defaultWorkstationCluster.WorkstationClusterId,
        Location = "us-central1",
        Host = new Gcp.Workstations.Inputs.WorkstationConfigHostArgs
        {
            GceInstance = new Gcp.Workstations.Inputs.WorkstationConfigHostGceInstanceArgs
            {
                MachineType = "n1-standard-2",
                BootDiskSizeGb = 35,
                DisablePublicIpAddresses = true,
                Accelerators = new[]
                {
                    new Gcp.Workstations.Inputs.WorkstationConfigHostGceInstanceAcceleratorArgs
                    {
                        Type = "nvidia-tesla-t4",
                        Count = 1,
                    },
                },
            },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-gcp/sdk/v8/go/gcp/compute"
	"github.com/pulumi/pulumi-gcp/sdk/v8/go/gcp/workstations"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := compute.NewNetwork(ctx, "default", &compute.NetworkArgs{
			Name:                  pulumi.String("workstation-cluster"),
			AutoCreateSubnetworks: pulumi.Bool(false),
		})
		if err != nil {
			return err
		}
		defaultSubnetwork, err := compute.NewSubnetwork(ctx, "default", &compute.SubnetworkArgs{
			Name:        pulumi.String("workstation-cluster"),
			IpCidrRange: pulumi.String("10.0.0.0/24"),
			Region:      pulumi.String("us-central1"),
			Network:     _default.Name,
		})
		if err != nil {
			return err
		}
		defaultWorkstationCluster, err := workstations.NewWorkstationCluster(ctx, "default", &workstations.WorkstationClusterArgs{
			WorkstationClusterId: pulumi.String("workstation-cluster"),
			Network:              _default.ID(),
			Subnetwork:           defaultSubnetwork.ID(),
			Location:             pulumi.String("us-central1"),
			Labels: pulumi.StringMap{
				"label": pulumi.String("key"),
			},
			Annotations: pulumi.StringMap{
				"label-one": pulumi.String("value-one"),
			},
		})
		if err != nil {
			return err
		}
		_, err = workstations.NewWorkstationConfig(ctx, "default", &workstations.WorkstationConfigArgs{
			WorkstationConfigId:  pulumi.String("workstation-config"),
			WorkstationClusterId: defaultWorkstationCluster.WorkstationClusterId,
			Location:             pulumi.String("us-central1"),
			Host: &workstations.WorkstationConfigHostArgs{
				GceInstance: &workstations.WorkstationConfigHostGceInstanceArgs{
					MachineType:              pulumi.String("n1-standard-2"),
					BootDiskSizeGb:           pulumi.Int(35),
					DisablePublicIpAddresses: pulumi.Bool(true),
					Accelerators: workstations.WorkstationConfigHostGceInstanceAcceleratorArray{
						&workstations.WorkstationConfigHostGceInstanceAcceleratorArgs{
							Type:  pulumi.String("nvidia-tesla-t4"),
							Count: pulumi.Int(1),
						},
					},
				},
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.gcp.compute.Network;
import com.pulumi.gcp.compute.NetworkArgs;
import com.pulumi.gcp.compute.Subnetwork;
import com.pulumi.gcp.compute.SubnetworkArgs;
import com.pulumi.gcp.workstations.WorkstationCluster;
import com.pulumi.gcp.workstations.WorkstationClusterArgs;
import com.pulumi.gcp.workstations.WorkstationConfig;
import com.pulumi.gcp.workstations.WorkstationConfigArgs;
import com.pulumi.gcp.workstations.inputs.WorkstationConfigHostArgs;
import com.pulumi.gcp.workstations.inputs.WorkstationConfigHostGceInstanceArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var default_ = new Network("default", NetworkArgs.builder()
            .name("workstation-cluster")
            .autoCreateSubnetworks(false)
            .build());

        var defaultSubnetwork = new Subnetwork("defaultSubnetwork", SubnetworkArgs.builder()
            .name("workstation-cluster")
            .ipCidrRange("10.0.0.0/24")
            .region("us-central1")
            .network(default_.name())
            .build());

        var defaultWorkstationCluster = new WorkstationCluster("defaultWorkstationCluster", WorkstationClusterArgs.builder()
            .workstationClusterId("workstation-cluster")
            .network(default_.id())
            .subnetwork(defaultSubnetwork.id())
            .location("us-central1")
            .labels(Map.of("label", "key"))
            .annotations(Map.of("label-one", "value-one"))
            .build());

        var defaultWorkstationConfig = new WorkstationConfig("defaultWorkstationConfig", WorkstationConfigArgs.builder()
            .workstationConfigId("workstation-config")
            .workstationClusterId(defaultWorkstationCluster.workstationClusterId())
            .location("us-central1")
            .host(WorkstationConfigHostArgs.builder()
                .gceInstance(WorkstationConfigHostGceInstanceArgs.builder()
                    .machineType("n1-standard-2")
                    .bootDiskSizeGb(35)
                    .disablePublicIpAddresses(true)
                    .accelerators(WorkstationConfigHostGceInstanceAcceleratorArgs.builder()
                        .type("nvidia-tesla-t4")
                        .count("1")
                        .build())
                    .build())
                .build())
            .build());

    }
}
```
```yaml
resources:
  default:
    type: gcp:compute:Network
    properties:
      name: workstation-cluster
      autoCreateSubnetworks: false
  defaultSubnetwork:
    type: gcp:compute:Subnetwork
    name: default
    properties:
      name: workstation-cluster
      ipCidrRange: 10.0.0.0/24
      region: us-central1
      network: ${default.name}
  defaultWorkstationCluster:
    type: gcp:workstations:WorkstationCluster
    name: default
    properties:
      workstationClusterId: workstation-cluster
      network: ${default.id}
      subnetwork: ${defaultSubnetwork.id}
      location: us-central1
      labels:
        label: key
      annotations:
        label-one: value-one
  defaultWorkstationConfig:
    type: gcp:workstations:WorkstationConfig
    name: default
    properties:
      workstationConfigId: workstation-config
      workstationClusterId: ${defaultWorkstationCluster.workstationClusterId}
      location: us-central1
      host:
        gceInstance:
          machineType: n1-standard-2
          bootDiskSizeGb: 35
          disablePublicIpAddresses: true
          accelerators:
            - type: nvidia-tesla-t4
              count: '1'
```
<!--End PulumiCodeChooser -->
### Workstation Config Boost


<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as gcp from "@pulumi/gcp";

const _default = new gcp.compute.Network("default", {
    name: "workstation-cluster",
    autoCreateSubnetworks: false,
});
const defaultSubnetwork = new gcp.compute.Subnetwork("default", {
    name: "workstation-cluster",
    ipCidrRange: "10.0.0.0/24",
    region: "us-central1",
    network: _default.name,
});
const defaultWorkstationCluster = new gcp.workstations.WorkstationCluster("default", {
    workstationClusterId: "workstation-cluster",
    network: _default.id,
    subnetwork: defaultSubnetwork.id,
    location: "us-central1",
    labels: {
        label: "key",
    },
    annotations: {
        "label-one": "value-one",
    },
});
const defaultWorkstationConfig = new gcp.workstations.WorkstationConfig("default", {
    workstationConfigId: "workstation-config",
    workstationClusterId: defaultWorkstationCluster.workstationClusterId,
    location: "us-central1",
    host: {
        gceInstance: {
            machineType: "e2-standard-4",
            bootDiskSizeGb: 35,
            disablePublicIpAddresses: true,
            boostConfigs: [
                {
                    id: "boost-1",
                    machineType: "n1-standard-2",
                    accelerators: [{
                        type: "nvidia-tesla-t4",
                        count: 1,
                    }],
                },
                {
                    id: "boost-2",
                    machineType: "n1-standard-2",
                    poolSize: 2,
                    bootDiskSizeGb: 30,
                    enableNestedVirtualization: true,
                },
            ],
        },
    },
});
```
```python
import pulumi
import pulumi_gcp as gcp

default = gcp.compute.Network("default",
    name="workstation-cluster",
    auto_create_subnetworks=False)
default_subnetwork = gcp.compute.Subnetwork("default",
    name="workstation-cluster",
    ip_cidr_range="10.0.0.0/24",
    region="us-central1",
    network=default.name)
default_workstation_cluster = gcp.workstations.WorkstationCluster("default",
    workstation_cluster_id="workstation-cluster",
    network=default.id,
    subnetwork=default_subnetwork.id,
    location="us-central1",
    labels={
        "label": "key",
    },
    annotations={
        "label-one": "value-one",
    })
default_workstation_config = gcp.workstations.WorkstationConfig("default",
    workstation_config_id="workstation-config",
    workstation_cluster_id=default_workstation_cluster.workstation_cluster_id,
    location="us-central1",
    host={
        "gce_instance": {
            "machine_type": "e2-standard-4",
            "boot_disk_size_gb": 35,
            "disable_public_ip_addresses": True,
            "boost_configs": [
                {
                    "id": "boost-1",
                    "machine_type": "n1-standard-2",
                    "accelerators": [{
                        "type": "nvidia-tesla-t4",
                        "count": 1,
                    }],
                },
                {
                    "id": "boost-2",
                    "machine_type": "n1-standard-2",
                    "pool_size": 2,
                    "boot_disk_size_gb": 30,
                    "enable_nested_virtualization": True,
                },
            ],
        },
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Gcp = Pulumi.Gcp;

return await Deployment.RunAsync(() => 
{
    var @default = new Gcp.Compute.Network("default", new()
    {
        Name = "workstation-cluster",
        AutoCreateSubnetworks = false,
    });

    var defaultSubnetwork = new Gcp.Compute.Subnetwork("default", new()
    {
        Name = "workstation-cluster",
        IpCidrRange = "10.0.0.0/24",
        Region = "us-central1",
        Network = @default.Name,
    });

    var defaultWorkstationCluster = new Gcp.Workstations.WorkstationCluster("default", new()
    {
        WorkstationClusterId = "workstation-cluster",
        Network = @default.Id,
        Subnetwork = defaultSubnetwork.Id,
        Location = "us-central1",
        Labels = 
        {
            { "label", "key" },
        },
        Annotations = 
        {
            { "label-one", "value-one" },
        },
    });

    var defaultWorkstationConfig = new Gcp.Workstations.WorkstationConfig("default", new()
    {
        WorkstationConfigId = "workstation-config",
        WorkstationClusterId = defaultWorkstationCluster.WorkstationClusterId,
        Location = "us-central1",
        Host = new Gcp.Workstations.Inputs.WorkstationConfigHostArgs
        {
            GceInstance = new Gcp.Workstations.Inputs.WorkstationConfigHostGceInstanceArgs
            {
                MachineType = "e2-standard-4",
                BootDiskSizeGb = 35,
                DisablePublicIpAddresses = true,
                BoostConfigs = new[]
                {
                    new Gcp.Workstations.Inputs.WorkstationConfigHostGceInstanceBoostConfigArgs
                    {
                        Id = "boost-1",
                        MachineType = "n1-standard-2",
                        Accelerators = new[]
                        {
                            new Gcp.Workstations.Inputs.WorkstationConfigHostGceInstanceBoostConfigAcceleratorArgs
                            {
                                Type = "nvidia-tesla-t4",
                                Count = 1,
                            },
                        },
                    },
                    new Gcp.Workstations.Inputs.WorkstationConfigHostGceInstanceBoostConfigArgs
                    {
                        Id = "boost-2",
                        MachineType = "n1-standard-2",
                        PoolSize = 2,
                        BootDiskSizeGb = 30,
                        EnableNestedVirtualization = true,
                    },
                },
            },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-gcp/sdk/v8/go/gcp/compute"
	"github.com/pulumi/pulumi-gcp/sdk/v8/go/gcp/workstations"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := compute.NewNetwork(ctx, "default", &compute.NetworkArgs{
			Name:                  pulumi.String("workstation-cluster"),
			AutoCreateSubnetworks: pulumi.Bool(false),
		})
		if err != nil {
			return err
		}
		defaultSubnetwork, err := compute.NewSubnetwork(ctx, "default", &compute.SubnetworkArgs{
			Name:        pulumi.String("workstation-cluster"),
			IpCidrRange: pulumi.String("10.0.0.0/24"),
			Region:      pulumi.String("us-central1"),
			Network:     _default.Name,
		})
		if err != nil {
			return err
		}
		defaultWorkstationCluster, err := workstations.NewWorkstationCluster(ctx, "default", &workstations.WorkstationClusterArgs{
			WorkstationClusterId: pulumi.String("workstation-cluster"),
			Network:              _default.ID(),
			Subnetwork:           defaultSubnetwork.ID(),
			Location:             pulumi.String("us-central1"),
			Labels: pulumi.StringMap{
				"label": pulumi.String("key"),
			},
			Annotations: pulumi.StringMap{
				"label-one": pulumi.String("value-one"),
			},
		})
		if err != nil {
			return err
		}
		_, err = workstations.NewWorkstationConfig(ctx, "default", &workstations.WorkstationConfigArgs{
			WorkstationConfigId:  pulumi.String("workstation-config"),
			WorkstationClusterId: defaultWorkstationCluster.WorkstationClusterId,
			Location:             pulumi.String("us-central1"),
			Host: &workstations.WorkstationConfigHostArgs{
				GceInstance: &workstations.WorkstationConfigHostGceInstanceArgs{
					MachineType:              pulumi.String("e2-standard-4"),
					BootDiskSizeGb:           pulumi.Int(35),
					DisablePublicIpAddresses: pulumi.Bool(true),
					BoostConfigs: workstations.WorkstationConfigHostGceInstanceBoostConfigArray{
						&workstations.WorkstationConfigHostGceInstanceBoostConfigArgs{
							Id:          pulumi.String("boost-1"),
							MachineType: pulumi.String("n1-standard-2"),
							Accelerators: workstations.WorkstationConfigHostGceInstanceBoostConfigAcceleratorArray{
								&workstations.WorkstationConfigHostGceInstanceBoostConfigAcceleratorArgs{
									Type:  pulumi.String("nvidia-tesla-t4"),
									Count: pulumi.Int(1),
								},
							},
						},
						&workstations.WorkstationConfigHostGceInstanceBoostConfigArgs{
							Id:                         pulumi.String("boost-2"),
							MachineType:                pulumi.String("n1-standard-2"),
							PoolSize:                   pulumi.Int(2),
							BootDiskSizeGb:             pulumi.Int(30),
							EnableNestedVirtualization: pulumi.Bool(true),
						},
					},
				},
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.gcp.compute.Network;
import com.pulumi.gcp.compute.NetworkArgs;
import com.pulumi.gcp.compute.Subnetwork;
import com.pulumi.gcp.compute.SubnetworkArgs;
import com.pulumi.gcp.workstations.WorkstationCluster;
import com.pulumi.gcp.workstations.WorkstationClusterArgs;
import com.pulumi.gcp.workstations.WorkstationConfig;
import com.pulumi.gcp.workstations.WorkstationConfigArgs;
import com.pulumi.gcp.workstations.inputs.WorkstationConfigHostArgs;
import com.pulumi.gcp.workstations.inputs.WorkstationConfigHostGceInstanceArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var default_ = new Network("default", NetworkArgs.builder()
            .name("workstation-cluster")
            .autoCreateSubnetworks(false)
            .build());

        var defaultSubnetwork = new Subnetwork("defaultSubnetwork", SubnetworkArgs.builder()
            .name("workstation-cluster")
            .ipCidrRange("10.0.0.0/24")
            .region("us-central1")
            .network(default_.name())
            .build());

        var defaultWorkstationCluster = new WorkstationCluster("defaultWorkstationCluster", WorkstationClusterArgs.builder()
            .workstationClusterId("workstation-cluster")
            .network(default_.id())
            .subnetwork(defaultSubnetwork.id())
            .location("us-central1")
            .labels(Map.of("label", "key"))
            .annotations(Map.of("label-one", "value-one"))
            .build());

        var defaultWorkstationConfig = new WorkstationConfig("defaultWorkstationConfig", WorkstationConfigArgs.builder()
            .workstationConfigId("workstation-config")
            .workstationClusterId(defaultWorkstationCluster.workstationClusterId())
            .location("us-central1")
            .host(WorkstationConfigHostArgs.builder()
                .gceInstance(WorkstationConfigHostGceInstanceArgs.builder()
                    .machineType("e2-standard-4")
                    .bootDiskSizeGb(35)
                    .disablePublicIpAddresses(true)
                    .boostConfigs(                    
                        WorkstationConfigHostGceInstanceBoostConfigArgs.builder()
                            .id("boost-1")
                            .machineType("n1-standard-2")
                            .accelerators(WorkstationConfigHostGceInstanceBoostConfigAcceleratorArgs.builder()
                                .type("nvidia-tesla-t4")
                                .count("1")
                                .build())
                            .build(),
                        WorkstationConfigHostGceInstanceBoostConfigArgs.builder()
                            .id("boost-2")
                            .machineType("n1-standard-2")
                            .poolSize(2)
                            .bootDiskSizeGb(30)
                            .enableNestedVirtualization(true)
                            .build())
                    .build())
                .build())
            .build());

    }
}
```
```yaml
resources:
  default:
    type: gcp:compute:Network
    properties:
      name: workstation-cluster
      autoCreateSubnetworks: false
  defaultSubnetwork:
    type: gcp:compute:Subnetwork
    name: default
    properties:
      name: workstation-cluster
      ipCidrRange: 10.0.0.0/24
      region: us-central1
      network: ${default.name}
  defaultWorkstationCluster:
    type: gcp:workstations:WorkstationCluster
    name: default
    properties:
      workstationClusterId: workstation-cluster
      network: ${default.id}
      subnetwork: ${defaultSubnetwork.id}
      location: us-central1
      labels:
        label: key
      annotations:
        label-one: value-one
  defaultWorkstationConfig:
    type: gcp:workstations:WorkstationConfig
    name: default
    properties:
      workstationConfigId: workstation-config
      workstationClusterId: ${defaultWorkstationCluster.workstationClusterId}
      location: us-central1
      host:
        gceInstance:
          machineType: e2-standard-4
          bootDiskSizeGb: 35
          disablePublicIpAddresses: true
          boostConfigs:
            - id: boost-1
              machineType: n1-standard-2
              accelerators:
                - type: nvidia-tesla-t4
                  count: '1'
            - id: boost-2
              machineType: n1-standard-2
              poolSize: 2
              bootDiskSizeGb: 30
              enableNestedVirtualization: true
```
<!--End PulumiCodeChooser -->
### Workstation Config Encryption Key


<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as gcp from "@pulumi/gcp";

const _default = new gcp.compute.Network("default", {
    name: "workstation-cluster",
    autoCreateSubnetworks: false,
});
const defaultSubnetwork = new gcp.compute.Subnetwork("default", {
    name: "workstation-cluster",
    ipCidrRange: "10.0.0.0/24",
    region: "us-central1",
    network: _default.name,
});
const defaultWorkstationCluster = new gcp.workstations.WorkstationCluster("default", {
    workstationClusterId: "workstation-cluster",
    network: _default.id,
    subnetwork: defaultSubnetwork.id,
    location: "us-central1",
    labels: {
        label: "key",
    },
    annotations: {
        "label-one": "value-one",
    },
});
const defaultKeyRing = new gcp.kms.KeyRing("default", {
    name: "workstation-cluster",
    location: "us-central1",
});
const defaultCryptoKey = new gcp.kms.CryptoKey("default", {
    name: "workstation-cluster",
    keyRing: defaultKeyRing.id,
});
const defaultAccount = new gcp.serviceaccount.Account("default", {
    accountId: "my-account",
    displayName: "Service Account",
});
const defaultWorkstationConfig = new gcp.workstations.WorkstationConfig("default", {
    workstationConfigId: "workstation-config",
    workstationClusterId: defaultWorkstationCluster.workstationClusterId,
    location: "us-central1",
    host: {
        gceInstance: {
            machineType: "e2-standard-4",
            bootDiskSizeGb: 35,
            disablePublicIpAddresses: true,
            shieldedInstanceConfig: {
                enableSecureBoot: true,
                enableVtpm: true,
            },
        },
    },
    encryptionKey: {
        kmsKey: defaultCryptoKey.id,
        kmsKeyServiceAccount: defaultAccount.email,
    },
});
```
```python
import pulumi
import pulumi_gcp as gcp

default = gcp.compute.Network("default",
    name="workstation-cluster",
    auto_create_subnetworks=False)
default_subnetwork = gcp.compute.Subnetwork("default",
    name="workstation-cluster",
    ip_cidr_range="10.0.0.0/24",
    region="us-central1",
    network=default.name)
default_workstation_cluster = gcp.workstations.WorkstationCluster("default",
    workstation_cluster_id="workstation-cluster",
    network=default.id,
    subnetwork=default_subnetwork.id,
    location="us-central1",
    labels={
        "label": "key",
    },
    annotations={
        "label-one": "value-one",
    })
default_key_ring = gcp.kms.KeyRing("default",
    name="workstation-cluster",
    location="us-central1")
default_crypto_key = gcp.kms.CryptoKey("default",
    name="workstation-cluster",
    key_ring=default_key_ring.id)
default_account = gcp.serviceaccount.Account("default",
    account_id="my-account",
    display_name="Service Account")
default_workstation_config = gcp.workstations.WorkstationConfig("default",
    workstation_config_id="workstation-config",
    workstation_cluster_id=default_workstation_cluster.workstation_cluster_id,
    location="us-central1",
    host={
        "gce_instance": {
            "machine_type": "e2-standard-4",
            "boot_disk_size_gb": 35,
            "disable_public_ip_addresses": True,
            "shielded_instance_config": {
                "enable_secure_boot": True,
                "enable_vtpm": True,
            },
        },
    },
    encryption_key={
        "kms_key": default_crypto_key.id,
        "kms_key_service_account": default_account.email,
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Gcp = Pulumi.Gcp;

return await Deployment.RunAsync(() => 
{
    var @default = new Gcp.Compute.Network("default", new()
    {
        Name = "workstation-cluster",
        AutoCreateSubnetworks = false,
    });

    var defaultSubnetwork = new Gcp.Compute.Subnetwork("default", new()
    {
        Name = "workstation-cluster",
        IpCidrRange = "10.0.0.0/24",
        Region = "us-central1",
        Network = @default.Name,
    });

    var defaultWorkstationCluster = new Gcp.Workstations.WorkstationCluster("default", new()
    {
        WorkstationClusterId = "workstation-cluster",
        Network = @default.Id,
        Subnetwork = defaultSubnetwork.Id,
        Location = "us-central1",
        Labels = 
        {
            { "label", "key" },
        },
        Annotations = 
        {
            { "label-one", "value-one" },
        },
    });

    var defaultKeyRing = new Gcp.Kms.KeyRing("default", new()
    {
        Name = "workstation-cluster",
        Location = "us-central1",
    });

    var defaultCryptoKey = new Gcp.Kms.CryptoKey("default", new()
    {
        Name = "workstation-cluster",
        KeyRing = defaultKeyRing.Id,
    });

    var defaultAccount = new Gcp.ServiceAccount.Account("default", new()
    {
        AccountId = "my-account",
        DisplayName = "Service Account",
    });

    var defaultWorkstationConfig = new Gcp.Workstations.WorkstationConfig("default", new()
    {
        WorkstationConfigId = "workstation-config",
        WorkstationClusterId = defaultWorkstationCluster.WorkstationClusterId,
        Location = "us-central1",
        Host = new Gcp.Workstations.Inputs.WorkstationConfigHostArgs
        {
            GceInstance = new Gcp.Workstations.Inputs.WorkstationConfigHostGceInstanceArgs
            {
                MachineType = "e2-standard-4",
                BootDiskSizeGb = 35,
                DisablePublicIpAddresses = true,
                ShieldedInstanceConfig = new Gcp.Workstations.Inputs.WorkstationConfigHostGceInstanceShieldedInstanceConfigArgs
                {
                    EnableSecureBoot = true,
                    EnableVtpm = true,
                },
            },
        },
        EncryptionKey = new Gcp.Workstations.Inputs.WorkstationConfigEncryptionKeyArgs
        {
            KmsKey = defaultCryptoKey.Id,
            KmsKeyServiceAccount = defaultAccount.Email,
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-gcp/sdk/v8/go/gcp/compute"
	"github.com/pulumi/pulumi-gcp/sdk/v8/go/gcp/kms"
	"github.com/pulumi/pulumi-gcp/sdk/v8/go/gcp/serviceaccount"
	"github.com/pulumi/pulumi-gcp/sdk/v8/go/gcp/workstations"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := compute.NewNetwork(ctx, "default", &compute.NetworkArgs{
			Name:                  pulumi.String("workstation-cluster"),
			AutoCreateSubnetworks: pulumi.Bool(false),
		})
		if err != nil {
			return err
		}
		defaultSubnetwork, err := compute.NewSubnetwork(ctx, "default", &compute.SubnetworkArgs{
			Name:        pulumi.String("workstation-cluster"),
			IpCidrRange: pulumi.String("10.0.0.0/24"),
			Region:      pulumi.String("us-central1"),
			Network:     _default.Name,
		})
		if err != nil {
			return err
		}
		defaultWorkstationCluster, err := workstations.NewWorkstationCluster(ctx, "default", &workstations.WorkstationClusterArgs{
			WorkstationClusterId: pulumi.String("workstation-cluster"),
			Network:              _default.ID(),
			Subnetwork:           defaultSubnetwork.ID(),
			Location:             pulumi.String("us-central1"),
			Labels: pulumi.StringMap{
				"label": pulumi.String("key"),
			},
			Annotations: pulumi.StringMap{
				"label-one": pulumi.String("value-one"),
			},
		})
		if err != nil {
			return err
		}
		defaultKeyRing, err := kms.NewKeyRing(ctx, "default", &kms.KeyRingArgs{
			Name:     pulumi.String("workstation-cluster"),
			Location: pulumi.String("us-central1"),
		})
		if err != nil {
			return err
		}
		defaultCryptoKey, err := kms.NewCryptoKey(ctx, "default", &kms.CryptoKeyArgs{
			Name:    pulumi.String("workstation-cluster"),
			KeyRing: defaultKeyRing.ID(),
		})
		if err != nil {
			return err
		}
		defaultAccount, err := serviceaccount.NewAccount(ctx, "default", &serviceaccount.AccountArgs{
			AccountId:   pulumi.String("my-account"),
			DisplayName: pulumi.String("Service Account"),
		})
		if err != nil {
			return err
		}
		_, err = workstations.NewWorkstationConfig(ctx, "default", &workstations.WorkstationConfigArgs{
			WorkstationConfigId:  pulumi.String("workstation-config"),
			WorkstationClusterId: defaultWorkstationCluster.WorkstationClusterId,
			Location:             pulumi.String("us-central1"),
			Host: &workstations.WorkstationConfigHostArgs{
				GceInstance: &workstations.WorkstationConfigHostGceInstanceArgs{
					MachineType:              pulumi.String("e2-standard-4"),
					BootDiskSizeGb:           pulumi.Int(35),
					DisablePublicIpAddresses: pulumi.Bool(true),
					ShieldedInstanceConfig: &workstations.WorkstationConfigHostGceInstanceShieldedInstanceConfigArgs{
						EnableSecureBoot: pulumi.Bool(true),
						EnableVtpm:       pulumi.Bool(true),
					},
				},
			},
			EncryptionKey: &workstations.WorkstationConfigEncryptionKeyArgs{
				KmsKey:               defaultCryptoKey.ID(),
				KmsKeyServiceAccount: defaultAccount.Email,
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.gcp.compute.Network;
import com.pulumi.gcp.compute.NetworkArgs;
import com.pulumi.gcp.compute.Subnetwork;
import com.pulumi.gcp.compute.SubnetworkArgs;
import com.pulumi.gcp.workstations.WorkstationCluster;
import com.pulumi.gcp.workstations.WorkstationClusterArgs;
import com.pulumi.gcp.kms.KeyRing;
import com.pulumi.gcp.kms.KeyRingArgs;
import com.pulumi.gcp.kms.CryptoKey;
import com.pulumi.gcp.kms.CryptoKeyArgs;
import com.pulumi.gcp.serviceaccount.Account;
import com.pulumi.gcp.serviceaccount.AccountArgs;
import com.pulumi.gcp.workstations.WorkstationConfig;
import com.pulumi.gcp.workstations.WorkstationConfigArgs;
import com.pulumi.gcp.workstations.inputs.WorkstationConfigHostArgs;
import com.pulumi.gcp.workstations.inputs.WorkstationConfigHostGceInstanceArgs;
import com.pulumi.gcp.workstations.inputs.WorkstationConfigHostGceInstanceShieldedInstanceConfigArgs;
import com.pulumi.gcp.workstations.inputs.WorkstationConfigEncryptionKeyArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var default_ = new Network("default", NetworkArgs.builder()
            .name("workstation-cluster")
            .autoCreateSubnetworks(false)
            .build());

        var defaultSubnetwork = new Subnetwork("defaultSubnetwork", SubnetworkArgs.builder()
            .name("workstation-cluster")
            .ipCidrRange("10.0.0.0/24")
            .region("us-central1")
            .network(default_.name())
            .build());

        var defaultWorkstationCluster = new WorkstationCluster("defaultWorkstationCluster", WorkstationClusterArgs.builder()
            .workstationClusterId("workstation-cluster")
            .network(default_.id())
            .subnetwork(defaultSubnetwork.id())
            .location("us-central1")
            .labels(Map.of("label", "key"))
            .annotations(Map.of("label-one", "value-one"))
            .build());

        var defaultKeyRing = new KeyRing("defaultKeyRing", KeyRingArgs.builder()
            .name("workstation-cluster")
            .location("us-central1")
            .build());

        var defaultCryptoKey = new CryptoKey("defaultCryptoKey", CryptoKeyArgs.builder()
            .name("workstation-cluster")
            .keyRing(defaultKeyRing.id())
            .build());

        var defaultAccount = new Account("defaultAccount", AccountArgs.builder()
            .accountId("my-account")
            .displayName("Service Account")
            .build());

        var defaultWorkstationConfig = new WorkstationConfig("defaultWorkstationConfig", WorkstationConfigArgs.builder()
            .workstationConfigId("workstation-config")
            .workstationClusterId(defaultWorkstationCluster.workstationClusterId())
            .location("us-central1")
            .host(WorkstationConfigHostArgs.builder()
                .gceInstance(WorkstationConfigHostGceInstanceArgs.builder()
                    .machineType("e2-standard-4")
                    .bootDiskSizeGb(35)
                    .disablePublicIpAddresses(true)
                    .shieldedInstanceConfig(WorkstationConfigHostGceInstanceShieldedInstanceConfigArgs.builder()
                        .enableSecureBoot(true)
                        .enableVtpm(true)
                        .build())
                    .build())
                .build())
            .encryptionKey(WorkstationConfigEncryptionKeyArgs.builder()
                .kmsKey(defaultCryptoKey.id())
                .kmsKeyServiceAccount(defaultAccount.email())
                .build())
            .build());

    }
}
```
```yaml
resources:
  default:
    type: gcp:compute:Network
    properties:
      name: workstation-cluster
      autoCreateSubnetworks: false
  defaultSubnetwork:
    type: gcp:compute:Subnetwork
    name: default
    properties:
      name: workstation-cluster
      ipCidrRange: 10.0.0.0/24
      region: us-central1
      network: ${default.name}
  defaultWorkstationCluster:
    type: gcp:workstations:WorkstationCluster
    name: default
    properties:
      workstationClusterId: workstation-cluster
      network: ${default.id}
      subnetwork: ${defaultSubnetwork.id}
      location: us-central1
      labels:
        label: key
      annotations:
        label-one: value-one
  defaultKeyRing:
    type: gcp:kms:KeyRing
    name: default
    properties:
      name: workstation-cluster
      location: us-central1
  defaultCryptoKey:
    type: gcp:kms:CryptoKey
    name: default
    properties:
      name: workstation-cluster
      keyRing: ${defaultKeyRing.id}
  defaultAccount:
    type: gcp:serviceaccount:Account
    name: default
    properties:
      accountId: my-account
      displayName: Service Account
  defaultWorkstationConfig:
    type: gcp:workstations:WorkstationConfig
    name: default
    properties:
      workstationConfigId: workstation-config
      workstationClusterId: ${defaultWorkstationCluster.workstationClusterId}
      location: us-central1
      host:
        gceInstance:
          machineType: e2-standard-4
          bootDiskSizeGb: 35
          disablePublicIpAddresses: true
          shieldedInstanceConfig:
            enableSecureBoot: true
            enableVtpm: true
      encryptionKey:
        kmsKey: ${defaultCryptoKey.id}
        kmsKeyServiceAccount: ${defaultAccount.email}
```
<!--End PulumiCodeChooser -->
### Workstation Config Allowed Ports


<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as gcp from "@pulumi/gcp";

const _default = new gcp.compute.Network("default", {
    name: "workstation-cluster",
    autoCreateSubnetworks: false,
});
const defaultSubnetwork = new gcp.compute.Subnetwork("default", {
    name: "workstation-cluster",
    ipCidrRange: "10.0.0.0/24",
    region: "us-central1",
    network: _default.name,
});
const defaultWorkstationCluster = new gcp.workstations.WorkstationCluster("default", {
    workstationClusterId: "workstation-cluster",
    network: _default.id,
    subnetwork: defaultSubnetwork.id,
    location: "us-central1",
    labels: {
        label: "key",
    },
    annotations: {
        "label-one": "value-one",
    },
});
const defaultWorkstationConfig = new gcp.workstations.WorkstationConfig("default", {
    workstationConfigId: "workstation-config",
    workstationClusterId: defaultWorkstationCluster.workstationClusterId,
    location: "us-central1",
    host: {
        gceInstance: {
            machineType: "e2-standard-4",
            bootDiskSizeGb: 35,
            disablePublicIpAddresses: true,
        },
    },
    allowedPorts: [
        {
            first: 80,
            last: 80,
        },
        {
            first: 22,
            last: 22,
        },
        {
            first: 1024,
            last: 65535,
        },
    ],
});
```
```python
import pulumi
import pulumi_gcp as gcp

default = gcp.compute.Network("default",
    name="workstation-cluster",
    auto_create_subnetworks=False)
default_subnetwork = gcp.compute.Subnetwork("default",
    name="workstation-cluster",
    ip_cidr_range="10.0.0.0/24",
    region="us-central1",
    network=default.name)
default_workstation_cluster = gcp.workstations.WorkstationCluster("default",
    workstation_cluster_id="workstation-cluster",
    network=default.id,
    subnetwork=default_subnetwork.id,
    location="us-central1",
    labels={
        "label": "key",
    },
    annotations={
        "label-one": "value-one",
    })
default_workstation_config = gcp.workstations.WorkstationConfig("default",
    workstation_config_id="workstation-config",
    workstation_cluster_id=default_workstation_cluster.workstation_cluster_id,
    location="us-central1",
    host={
        "gce_instance": {
            "machine_type": "e2-standard-4",
            "boot_disk_size_gb": 35,
            "disable_public_ip_addresses": True,
        },
    },
    allowed_ports=[
        {
            "first": 80,
            "last": 80,
        },
        {
            "first": 22,
            "last": 22,
        },
        {
            "first": 1024,
            "last": 65535,
        },
    ])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Gcp = Pulumi.Gcp;

return await Deployment.RunAsync(() => 
{
    var @default = new Gcp.Compute.Network("default", new()
    {
        Name = "workstation-cluster",
        AutoCreateSubnetworks = false,
    });

    var defaultSubnetwork = new Gcp.Compute.Subnetwork("default", new()
    {
        Name = "workstation-cluster",
        IpCidrRange = "10.0.0.0/24",
        Region = "us-central1",
        Network = @default.Name,
    });

    var defaultWorkstationCluster = new Gcp.Workstations.WorkstationCluster("default", new()
    {
        WorkstationClusterId = "workstation-cluster",
        Network = @default.Id,
        Subnetwork = defaultSubnetwork.Id,
        Location = "us-central1",
        Labels = 
        {
            { "label", "key" },
        },
        Annotations = 
        {
            { "label-one", "value-one" },
        },
    });

    var defaultWorkstationConfig = new Gcp.Workstations.WorkstationConfig("default", new()
    {
        WorkstationConfigId = "workstation-config",
        WorkstationClusterId = defaultWorkstationCluster.WorkstationClusterId,
        Location = "us-central1",
        Host = new Gcp.Workstations.Inputs.WorkstationConfigHostArgs
        {
            GceInstance = new Gcp.Workstations.Inputs.WorkstationConfigHostGceInstanceArgs
            {
                MachineType = "e2-standard-4",
                BootDiskSizeGb = 35,
                DisablePublicIpAddresses = true,
            },
        },
        AllowedPorts = new[]
        {
            new Gcp.Workstations.Inputs.WorkstationConfigAllowedPortArgs
            {
                First = 80,
                Last = 80,
            },
            new Gcp.Workstations.Inputs.WorkstationConfigAllowedPortArgs
            {
                First = 22,
                Last = 22,
            },
            new Gcp.Workstations.Inputs.WorkstationConfigAllowedPortArgs
            {
                First = 1024,
                Last = 65535,
            },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-gcp/sdk/v8/go/gcp/compute"
	"github.com/pulumi/pulumi-gcp/sdk/v8/go/gcp/workstations"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := compute.NewNetwork(ctx, "default", &compute.NetworkArgs{
			Name:                  pulumi.String("workstation-cluster"),
			AutoCreateSubnetworks: pulumi.Bool(false),
		})
		if err != nil {
			return err
		}
		defaultSubnetwork, err := compute.NewSubnetwork(ctx, "default", &compute.SubnetworkArgs{
			Name:        pulumi.String("workstation-cluster"),
			IpCidrRange: pulumi.String("10.0.0.0/24"),
			Region:      pulumi.String("us-central1"),
			Network:     _default.Name,
		})
		if err != nil {
			return err
		}
		defaultWorkstationCluster, err := workstations.NewWorkstationCluster(ctx, "default", &workstations.WorkstationClusterArgs{
			WorkstationClusterId: pulumi.String("workstation-cluster"),
			Network:              _default.ID(),
			Subnetwork:           defaultSubnetwork.ID(),
			Location:             pulumi.String("us-central1"),
			Labels: pulumi.StringMap{
				"label": pulumi.String("key"),
			},
			Annotations: pulumi.StringMap{
				"label-one": pulumi.String("value-one"),
			},
		})
		if err != nil {
			return err
		}
		_, err = workstations.NewWorkstationConfig(ctx, "default", &workstations.WorkstationConfigArgs{
			WorkstationConfigId:  pulumi.String("workstation-config"),
			WorkstationClusterId: defaultWorkstationCluster.WorkstationClusterId,
			Location:             pulumi.String("us-central1"),
			Host: &workstations.WorkstationConfigHostArgs{
				GceInstance: &workstations.WorkstationConfigHostGceInstanceArgs{
					MachineType:              pulumi.String("e2-standard-4"),
					BootDiskSizeGb:           pulumi.Int(35),
					DisablePublicIpAddresses: pulumi.Bool(true),
				},
			},
			AllowedPorts: workstations.WorkstationConfigAllowedPortArray{
				&workstations.WorkstationConfigAllowedPortArgs{
					First: pulumi.Int(80),
					Last:  pulumi.Int(80),
				},
				&workstations.WorkstationConfigAllowedPortArgs{
					First: pulumi.Int(22),
					Last:  pulumi.Int(22),
				},
				&workstations.WorkstationConfigAllowedPortArgs{
					First: pulumi.Int(1024),
					Last:  pulumi.Int(65535),
				},
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.gcp.compute.Network;
import com.pulumi.gcp.compute.NetworkArgs;
import com.pulumi.gcp.compute.Subnetwork;
import com.pulumi.gcp.compute.SubnetworkArgs;
import com.pulumi.gcp.workstations.WorkstationCluster;
import com.pulumi.gcp.workstations.WorkstationClusterArgs;
import com.pulumi.gcp.workstations.WorkstationConfig;
import com.pulumi.gcp.workstations.WorkstationConfigArgs;
import com.pulumi.gcp.workstations.inputs.WorkstationConfigHostArgs;
import com.pulumi.gcp.workstations.inputs.WorkstationConfigHostGceInstanceArgs;
import com.pulumi.gcp.workstations.inputs.WorkstationConfigAllowedPortArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var default_ = new Network("default", NetworkArgs.builder()
            .name("workstation-cluster")
            .autoCreateSubnetworks(false)
            .build());

        var defaultSubnetwork = new Subnetwork("defaultSubnetwork", SubnetworkArgs.builder()
            .name("workstation-cluster")
            .ipCidrRange("10.0.0.0/24")
            .region("us-central1")
            .network(default_.name())
            .build());

        var defaultWorkstationCluster = new WorkstationCluster("defaultWorkstationCluster", WorkstationClusterArgs.builder()
            .workstationClusterId("workstation-cluster")
            .network(default_.id())
            .subnetwork(defaultSubnetwork.id())
            .location("us-central1")
            .labels(Map.of("label", "key"))
            .annotations(Map.of("label-one", "value-one"))
            .build());

        var defaultWorkstationConfig = new WorkstationConfig("defaultWorkstationConfig", WorkstationConfigArgs.builder()
            .workstationConfigId("workstation-config")
            .workstationClusterId(defaultWorkstationCluster.workstationClusterId())
            .location("us-central1")
            .host(WorkstationConfigHostArgs.builder()
                .gceInstance(WorkstationConfigHostGceInstanceArgs.builder()
                    .machineType("e2-standard-4")
                    .bootDiskSizeGb(35)
                    .disablePublicIpAddresses(true)
                    .build())
                .build())
            .allowedPorts(            
                WorkstationConfigAllowedPortArgs.builder()
                    .first(80)
                    .last(80)
                    .build(),
                WorkstationConfigAllowedPortArgs.builder()
                    .first(22)
                    .last(22)
                    .build(),
                WorkstationConfigAllowedPortArgs.builder()
                    .first(1024)
                    .last(65535)
                    .build())
            .build());

    }
}
```
```yaml
resources:
  default:
    type: gcp:compute:Network
    properties:
      name: workstation-cluster
      autoCreateSubnetworks: false
  defaultSubnetwork:
    type: gcp:compute:Subnetwork
    name: default
    properties:
      name: workstation-cluster
      ipCidrRange: 10.0.0.0/24
      region: us-central1
      network: ${default.name}
  defaultWorkstationCluster:
    type: gcp:workstations:WorkstationCluster
    name: default
    properties:
      workstationClusterId: workstation-cluster
      network: ${default.id}
      subnetwork: ${defaultSubnetwork.id}
      location: us-central1
      labels:
        label: key
      annotations:
        label-one: value-one
  defaultWorkstationConfig:
    type: gcp:workstations:WorkstationConfig
    name: default
    properties:
      workstationConfigId: workstation-config
      workstationClusterId: ${defaultWorkstationCluster.workstationClusterId}
      location: us-central1
      host:
        gceInstance:
          machineType: e2-standard-4
          bootDiskSizeGb: 35
          disablePublicIpAddresses: true
      allowedPorts:
        - first: 80
          last: 80
        - first: 22
          last: 22
        - first: 1024
          last: 65535
```
<!--End PulumiCodeChooser -->

## Import

WorkstationConfig can be imported using any of these accepted formats:

* `projects/{{project}}/locations/{{location}}/workstationClusters/{{workstation_cluster_id}}/workstationConfigs/{{workstation_config_id}}`

* `{{project}}/{{location}}/{{workstation_cluster_id}}/{{workstation_config_id}}`

* `{{location}}/{{workstation_cluster_id}}/{{workstation_config_id}}`

When using the `pulumi import` command, WorkstationConfig can be imported using one of the formats above. For example:

```sh
$ pulumi import gcp:workstations/workstationConfig:WorkstationConfig default projects/{{project}}/locations/{{location}}/workstationClusters/{{workstation_cluster_id}}/workstationConfigs/{{workstation_config_id}}
```

```sh
$ pulumi import gcp:workstations/workstationConfig:WorkstationConfig default {{project}}/{{location}}/{{workstation_cluster_id}}/{{workstation_config_id}}
```

```sh
$ pulumi import gcp:workstations/workstationConfig:WorkstationConfig default {{location}}/{{workstation_cluster_id}}/{{workstation_config_id}}
```

©
allowedPortsÄB~*|:z
x
workstationsWorkstationConfigAllowedPortJgcp:workstations/WorkstationConfigAllowedPort:WorkstationConfigAllowedPortïA list of port ranges specifying single ports or ranges of ports that are externally accessible in the workstation. Allowed ports must be one of 22, 80, or within range 1024-65535. If not specified defaults to ports 22, 80, and ports 1024-65535.
Structure is documented below.
©
annotationsB2" ëClient-specified annotations. This is distinct from labels.
**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.
Please refer to the field `effective_annotations` for all of the annotations present on the resource.
í
	containerxBv:t
r
workstationsWorkstationConfigContainerFgcp:workstations/WorkstationConfigContainer:WorkstationConfigContaineräContainer that will be run for each workstation using this configuration when that workstation is started.
Structure is documented below.
Î
disableTcpConnectionsB
 ÀDisables support for plain TCP connections in the workstation. By default the service supports TCP connections via a websocket relay. Setting this option to true disables that relay, which prevents the usage of services that require plain tcp connections, such as ssh. When enabled, all communication must occur over https or wss.
<
displayNameB" 'Human-readable name for this resource.
ç
enableAuditAgentB
 ÚWhether to enable Linux `auditd` logging on the workstation. When enabled, a service account must also be specified that has `logging.buckets.write` permission on the project. Operating system audit logging is distinct from Cloud Audit Logs.
Ω
encryptionKeyÜBÉ:Ä
~
workstationsWorkstationConfigEncryptionKeyNgcp:workstations/WorkstationConfigEncryptionKey:WorkstationConfigEncryptionKey¢Encrypts resources of this workstation configuration using a customer-managed encryption key.
If specified, the boot disk of the Compute Engine instance and the persistent disk are encrypted using this encryption key. If this field is not set, the disks are encrypted using a generated key. Customer-managed encryption keys do not protect disk metadata.
If the customer-managed encryption key is rotated, when the workstation instance is stopped, the system attempts to recreate the persistent disk with the new version of the key. Be sure to keep older versions of the key until the persistent disk is recreated. Otherwise, data on the persistent disk will be lost.
If the encryption key is revoked, the workstation session will automatically be stopped within 7 hours.
Structure is documented below.
ö
ephemeralDirectoriesôBñ*ì:ê
ç
workstations#WorkstationConfigEphemeralDirectoryXgcp:workstations/WorkstationConfigEphemeralDirectory:WorkstationConfigEphemeralDirectoryfEphemeral directories which won't persist across workstation sessions.
Structure is documented below.
≤
hostiBg:e
c
workstationsWorkstationConfigHost<gcp:workstations/WorkstationConfigHost:WorkstationConfigHost?Runtime host for a workstation.
Structure is documented below.
√
idleTimeoutB" ≠How long to wait before automatically stopping an instance that hasn't recently received any user traffic. A value of 0 indicates that this instance should never time out from idleness. Defaults to 20 minutes.
A duration in seconds with up to nine fractional digits, ending with 's'. Example: "3.5s".
€
labelsB2" »Client-specified labels that are applied to the resource and that are also propagated to the underlying Compute Engine resources.
**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
Please refer to the field `effective_labels` for all of the labels present on the resource.
Y
location" IThe location where the workstation cluster config should reside.


- - -
Ê
maxUsableWorkstationsB ∆Maximum number of workstations under this configuration a user can have workstations.workstation.use permission on. Only enforced on CreateWorkstation API calls on the user issuing the API request.
ã
persistentDirectoriesúBô*ñ:ì
ê
workstations$WorkstationConfigPersistentDirectoryZgcp:workstations/WorkstationConfigPersistentDirectory:WorkstationConfigPersistentDirectorySDirectories to persist across workstation sessions.
Structure is documented below.
{
projectB" jThe ID of the project in which the resource belongs.
If it is not provided, the provider project is used.
ı
readinessChecksçBä*á:Ñ
Å
workstationsWorkstationConfigReadinessCheckPgcp:workstations/WorkstationConfigReadinessCheck:WorkstationConfigReadinessCheckRReadiness checks to be performed on a workstation.
Structure is documented below.
ı
replicaZonesB*" ‹Specifies the zones used to replicate the VM and disk resources within the region. If set, exactly two zones within the workstation cluster's region must be specified‚Äîfor example, `['us-central1-a', 'us-central1-f']`.
If this field is empty, two default zones within the region are used. Immutable after the workstation configuration is created.
ñ
runningTimeoutB" ˝How long to wait before automatically stopping a workstation after it was started. A value of 0 indicates that workstations using this configuration should never time out from running duration. Must be greater than 0 and less than 24 hours if `encryption_key` is set. Defaults to 12 hours.
A duration in seconds with up to nine fractional digits, ending with 's'. Example: "3.5s".
F
workstationClusterId" *The ID of the parent workstation cluster.
T
workstationConfigId" 9The ID to be assigned to the workstation cluster config.
"¶
allowedPorts~*|:z
x
workstationsWorkstationConfigAllowedPortJgcp:workstations/WorkstationConfigAllowedPort:WorkstationConfigAllowedPortïA list of port ranges specifying single ports or ranges of ports that are externally accessible in the workstation. Allowed ports must be one of 22, 80, or within range 1024-65535. If not specified defaults to ports 22, 80, and ports 1024-65535.
Structure is documented below.
"©
annotationsB2" ëClient-specified annotations. This is distinct from labels.
**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.
Please refer to the field `effective_annotations` for all of the annotations present on the resource.
"‡

conditionsx*v:t
r
workstationsWorkstationConfigConditionFgcp:workstations/WorkstationConfigCondition:WorkstationConfigConditionXStatus conditions describing the current resource state.
Structure is documented below.
"ê
	containerv:t
r
workstationsWorkstationConfigContainerFgcp:workstations/WorkstationConfigContainer:WorkstationConfigContaineräContainer that will be run for each workstation using this configuration when that workstation is started.
Structure is documented below.
"7

createTime" %Time when this resource was created.
"≤
degraded
 °Whether this resource is in degraded mode, in which case it may require user action to restore full functionality. Details can be found in the conditions field.
"Î
disableTcpConnectionsB
 ÀDisables support for plain TCP connections in the workstation. By default the service supports TCP connections via a websocket relay. Setting this option to true disables that relay, which prevents the usage of services that require plain tcp connections, such as ssh. When enabled, all communication must occur over https or wss.
"<
displayNameB" 'Human-readable name for this resource.
"
effectiveAnnotations2" "¶
effectiveLabels2" åAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
"ç
enableAuditAgentB
 ÚWhether to enable Linux `auditd` logging on the workstation. When enabled, a service account must also be specified that has `logging.buckets.write` permission on the project. Operating system audit logging is distinct from Cloud Audit Logs.
"Ω
encryptionKeyÜBÉ:Ä
~
workstationsWorkstationConfigEncryptionKeyNgcp:workstations/WorkstationConfigEncryptionKey:WorkstationConfigEncryptionKey¢Encrypts resources of this workstation configuration using a customer-managed encryption key.
If specified, the boot disk of the Compute Engine instance and the persistent disk are encrypted using this encryption key. If this field is not set, the disks are encrypted using a generated key. Customer-managed encryption keys do not protect disk metadata.
If the customer-managed encryption key is rotated, when the workstation instance is stopped, the system attempts to recreate the persistent disk with the new version of the key. Be sure to keep older versions of the key until the persistent disk is recreated. Otherwise, data on the persistent disk will be lost.
If the encryption key is revoked, the workstation session will automatically be stopped within 7 hours.
Structure is documented below.
"ó
ephemeralDirectoriesñ*ì:ê
ç
workstations#WorkstationConfigEphemeralDirectoryXgcp:workstations/WorkstationConfigEphemeralDirectory:WorkstationConfigEphemeralDirectoryfEphemeral directories which won't persist across workstation sessions.
Structure is documented below.
"ù
etag" êChecksum computed by the server.
May be sent on update and delete requests to ensure that the client has an up-to-date value before proceeding.
"∞
hostg:e
c
workstationsWorkstationConfigHost<gcp:workstations/WorkstationConfigHost:WorkstationConfigHost?Runtime host for a workstation.
Structure is documented below.
"√
idleTimeoutB" ≠How long to wait before automatically stopping an instance that hasn't recently received any user traffic. A value of 0 indicates that this instance should never time out from idleness. Defaults to 20 minutes.
A duration in seconds with up to nine fractional digits, ending with 's'. Example: "3.5s".
"€
labelsB2" »Client-specified labels that are applied to the resource and that are also propagated to the underlying Compute Engine resources.
**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
Please refer to the field `effective_labels` for all of the labels present on the resource.
"Y
location" IThe location where the workstation cluster config should reside.


- - -
"‰
maxUsableWorkstations ∆Maximum number of workstations under this configuration a user can have workstations.workstation.use permission on. Only enforced on CreateWorkstation API calls on the user issuing the API request.
"(
name" Full name of this resource.
"à
persistentDirectoriesô*ñ:ì
ê
workstations$WorkstationConfigPersistentDirectoryZgcp:workstations/WorkstationConfigPersistentDirectory:WorkstationConfigPersistentDirectorySDirectories to persist across workstation sessions.
Structure is documented below.
"y
project" jThe ID of the project in which the resource belongs.
If it is not provided, the provider project is used.
"É
pulumiLabels2" mThe combination of labels configured directly on the resource
and default labels configured on the provider.
"ı
readinessChecksçBä*á:Ñ
Å
workstationsWorkstationConfigReadinessCheckPgcp:workstations/WorkstationConfigReadinessCheck:WorkstationConfigReadinessCheckRReadiness checks to be performed on a workstation.
Structure is documented below.
"Û
replicaZones*" ‹Specifies the zones used to replicate the VM and disk resources within the region. If set, exactly two zones within the workstation cluster's region must be specified‚Äîfor example, `['us-central1-a', 'us-central1-f']`.
If this field is empty, two default zones within the region are used. Immutable after the workstation configuration is created.
"ñ
runningTimeoutB" ˝How long to wait before automatically stopping a workstation after it was started. A value of 0 indicates that workstations using this configuration should never time out from running duration. Must be greater than 0 and less than 24 hours if `encryption_key` is set. Defaults to 12 hours.
A duration in seconds with up to nine fractional digits, ending with 's'. Example: "3.5s".
"5
uid" *The system-generated UID of the resource.
"F
workstationClusterId" *The ID of the parent workstation cluster.
"T
workstationConfigId" 9The ID to be assigned to the workstation cluster config.
*ﬂ4
u
workstationsWorkstationConfigIamBindingHgcp:workstations/workstationConfigIamBinding:WorkstationConfigIamBindingÚ

## Import

For all import syntaxes, the "resource in question" can take any of the following forms:

* projects/{{project}}/locations/{{location}}/workstationClusters/{{workstation_cluster_id}}/workstationConfigs/{{workstation_config_id}}

* {{project}}/{{location}}/{{workstation_cluster_id}}/{{workstation_config_id}}

* {{location}}/{{workstation_cluster_id}}/{{workstation_config_id}}

* {{workstation_config_id}}

Any variables not passed in the import command will be taken from the provider configuration.

Cloud Workstations workstationconfig IAM resources can be imported using the resource identifiers, role, and member.

IAM member imports use space-delimited identifiers: the resource in question, the role, and the member identity, e.g.

```sh
$ pulumi import gcp:workstations/workstationConfigIamBinding:WorkstationConfigIamBinding editor "projects/{{project}}/locations/{{location}}/workstationClusters/{{workstation_cluster_id}}/workstationConfigs/{{workstation_config_id}} roles/viewer user:jane@example.com"
```

IAM binding imports use space-delimited identifiers: the resource in question and the role, e.g.

```sh
$ pulumi import gcp:workstations/workstationConfigIamBinding:WorkstationConfigIamBinding editor "projects/{{project}}/locations/{{location}}/workstationClusters/{{workstation_cluster_id}}/workstationConfigs/{{workstation_config_id}} roles/viewer"
```

IAM policy imports use the identifier of the resource in question, e.g.

```sh
$ pulumi import gcp:workstations/workstationConfigIamBinding:WorkstationConfigIamBinding editor projects/{{project}}/locations/{{location}}/workstationClusters/{{workstation_cluster_id}}/workstationConfigs/{{workstation_config_id}}
```

-> **Custom Roles** If you're importing a IAM resource with a custom role, make sure to use the

 full name of the custom role, e.g. `[projects/my-project|organizations/my-org]/roles/my-custom-role`.

ß
	conditionôBñ:ì
ê
workstations$WorkstationConfigIamBindingConditionZgcp:workstations/WorkstationConfigIamBindingCondition:WorkstationConfigIamBindingConditionÊ
locationB" ”The location where the workstation cluster config should reside.
Used to find the parent resource to bind the IAM policy to. If not specified,
the value will be parsed from the identifier of the parent resource. If no location is provided in the parent identifier and no
location is specified, it is taken from the provider configuration.
÷	
members*" ƒ	Identities that will be granted the privilege in `role`.
Each entry can have one of the following values:
* **allUsers**: A special identifier that represents anyone who is on the internet; with or without a Google account.
* **allAuthenticatedUsers**: A special identifier that represents anyone who is authenticated with a Google account or a service account.
* **user:{emailid}**: An email address that represents a specific Google account. For example, alice@gmail.com or joe@example.com.
* **serviceAccount:{emailid}**: An email address that represents a service account. For example, my-other-app@appspot.gserviceaccount.com.
* **group:{emailid}**: An email address that represents a Google group. For example, admins@example.com.
* **domain:{domain}**: A G Suite domain (primary, instead of alias) name that represents all the users of that domain. For example, google.com or example.com.
* **projectOwner:projectid**: Owners of the given project. For example, "projectOwner:my-example-project"
* **projectEditor:projectid**: Editors of the given project. For example, "projectEditor:my-example-project"
* **projectViewer:projectid**: Viewers of the given project. For example, "projectViewer:my-example-project"
ì
projectB" ÅThe ID of the project in which the resource belongs.
If it is not provided, the project will be parsed from the identifier of the parent resource. If no project is provided in the parent identifier and no project is specified, the provider project is used.
Â
role" ÿThe role that should be applied. Only one
`gcp.workstations.WorkstationConfigIamBinding` can be used per role. Note that custom roles must be of the format
`[projects|organizations]/{parent-name}/roles/{role-name}`.

workstationClusterId" 
workstationConfigId" "ß
	conditionôBñ:ì
ê
workstations$WorkstationConfigIamBindingConditionZgcp:workstations/WorkstationConfigIamBindingCondition:WorkstationConfigIamBindingCondition"3
etag" '(Computed) The etag of the IAM policy.
"‰
location" ”The location where the workstation cluster config should reside.
Used to find the parent resource to bind the IAM policy to. If not specified,
the value will be parsed from the identifier of the parent resource. If no location is provided in the parent identifier and no
location is specified, it is taken from the provider configuration.
"÷	
members*" ƒ	Identities that will be granted the privilege in `role`.
Each entry can have one of the following values:
* **allUsers**: A special identifier that represents anyone who is on the internet; with or without a Google account.
* **allAuthenticatedUsers**: A special identifier that represents anyone who is authenticated with a Google account or a service account.
* **user:{emailid}**: An email address that represents a specific Google account. For example, alice@gmail.com or joe@example.com.
* **serviceAccount:{emailid}**: An email address that represents a service account. For example, my-other-app@appspot.gserviceaccount.com.
* **group:{emailid}**: An email address that represents a Google group. For example, admins@example.com.
* **domain:{domain}**: A G Suite domain (primary, instead of alias) name that represents all the users of that domain. For example, google.com or example.com.
* **projectOwner:projectid**: Owners of the given project. For example, "projectOwner:my-example-project"
* **projectEditor:projectid**: Editors of the given project. For example, "projectEditor:my-example-project"
* **projectViewer:projectid**: Viewers of the given project. For example, "projectViewer:my-example-project"
"ë
project" ÅThe ID of the project in which the resource belongs.
If it is not provided, the project will be parsed from the identifier of the parent resource. If no project is provided in the parent identifier and no project is specified, the provider project is used.
"Â
role" ÿThe role that should be applied. Only one
`gcp.workstations.WorkstationConfigIamBinding` can be used per role. Note that custom roles must be of the format
`[projects|organizations]/{parent-name}/roles/{role-name}`.
"
workstationClusterId" "
workstationConfigId" * 4
r
workstationsWorkstationConfigIamMemberFgcp:workstations/workstationConfigIamMember:WorkstationConfigIamMemberÏ

## Import

For all import syntaxes, the "resource in question" can take any of the following forms:

* projects/{{project}}/locations/{{location}}/workstationClusters/{{workstation_cluster_id}}/workstationConfigs/{{workstation_config_id}}

* {{project}}/{{location}}/{{workstation_cluster_id}}/{{workstation_config_id}}

* {{location}}/{{workstation_cluster_id}}/{{workstation_config_id}}

* {{workstation_config_id}}

Any variables not passed in the import command will be taken from the provider configuration.

Cloud Workstations workstationconfig IAM resources can be imported using the resource identifiers, role, and member.

IAM member imports use space-delimited identifiers: the resource in question, the role, and the member identity, e.g.

```sh
$ pulumi import gcp:workstations/workstationConfigIamMember:WorkstationConfigIamMember editor "projects/{{project}}/locations/{{location}}/workstationClusters/{{workstation_cluster_id}}/workstationConfigs/{{workstation_config_id}} roles/viewer user:jane@example.com"
```

IAM binding imports use space-delimited identifiers: the resource in question and the role, e.g.

```sh
$ pulumi import gcp:workstations/workstationConfigIamMember:WorkstationConfigIamMember editor "projects/{{project}}/locations/{{location}}/workstationClusters/{{workstation_cluster_id}}/workstationConfigs/{{workstation_config_id}} roles/viewer"
```

IAM policy imports use the identifier of the resource in question, e.g.

```sh
$ pulumi import gcp:workstations/workstationConfigIamMember:WorkstationConfigIamMember editor projects/{{project}}/locations/{{location}}/workstationClusters/{{workstation_cluster_id}}/workstationConfigs/{{workstation_config_id}}
```

-> **Custom Roles** If you're importing a IAM resource with a custom role, make sure to use the

 full name of the custom role, e.g. `[projects/my-project|organizations/my-org]/roles/my-custom-role`.

§
	conditionñBì:ê
ç
workstations#WorkstationConfigIamMemberConditionXgcp:workstations/WorkstationConfigIamMemberCondition:WorkstationConfigIamMemberConditionÊ
locationB" ”The location where the workstation cluster config should reside.
Used to find the parent resource to bind the IAM policy to. If not specified,
the value will be parsed from the identifier of the parent resource. If no location is provided in the parent identifier and no
location is specified, it is taken from the provider configuration.
”	
member" ƒ	Identities that will be granted the privilege in `role`.
Each entry can have one of the following values:
* **allUsers**: A special identifier that represents anyone who is on the internet; with or without a Google account.
* **allAuthenticatedUsers**: A special identifier that represents anyone who is authenticated with a Google account or a service account.
* **user:{emailid}**: An email address that represents a specific Google account. For example, alice@gmail.com or joe@example.com.
* **serviceAccount:{emailid}**: An email address that represents a service account. For example, my-other-app@appspot.gserviceaccount.com.
* **group:{emailid}**: An email address that represents a Google group. For example, admins@example.com.
* **domain:{domain}**: A G Suite domain (primary, instead of alias) name that represents all the users of that domain. For example, google.com or example.com.
* **projectOwner:projectid**: Owners of the given project. For example, "projectOwner:my-example-project"
* **projectEditor:projectid**: Editors of the given project. For example, "projectEditor:my-example-project"
* **projectViewer:projectid**: Viewers of the given project. For example, "projectViewer:my-example-project"
ì
projectB" ÅThe ID of the project in which the resource belongs.
If it is not provided, the project will be parsed from the identifier of the parent resource. If no project is provided in the parent identifier and no project is specified, the provider project is used.
Â
role" ÿThe role that should be applied. Only one
`gcp.workstations.WorkstationConfigIamBinding` can be used per role. Note that custom roles must be of the format
`[projects|organizations]/{parent-name}/roles/{role-name}`.

workstationClusterId" 
workstationConfigId" "§
	conditionñBì:ê
ç
workstations#WorkstationConfigIamMemberConditionXgcp:workstations/WorkstationConfigIamMemberCondition:WorkstationConfigIamMemberCondition"3
etag" '(Computed) The etag of the IAM policy.
"‰
location" ”The location where the workstation cluster config should reside.
Used to find the parent resource to bind the IAM policy to. If not specified,
the value will be parsed from the identifier of the parent resource. If no location is provided in the parent identifier and no
location is specified, it is taken from the provider configuration.
"”	
member" ƒ	Identities that will be granted the privilege in `role`.
Each entry can have one of the following values:
* **allUsers**: A special identifier that represents anyone who is on the internet; with or without a Google account.
* **allAuthenticatedUsers**: A special identifier that represents anyone who is authenticated with a Google account or a service account.
* **user:{emailid}**: An email address that represents a specific Google account. For example, alice@gmail.com or joe@example.com.
* **serviceAccount:{emailid}**: An email address that represents a service account. For example, my-other-app@appspot.gserviceaccount.com.
* **group:{emailid}**: An email address that represents a Google group. For example, admins@example.com.
* **domain:{domain}**: A G Suite domain (primary, instead of alias) name that represents all the users of that domain. For example, google.com or example.com.
* **projectOwner:projectid**: Owners of the given project. For example, "projectOwner:my-example-project"
* **projectEditor:projectid**: Editors of the given project. For example, "projectEditor:my-example-project"
* **projectViewer:projectid**: Viewers of the given project. For example, "projectViewer:my-example-project"
"ë
project" ÅThe ID of the project in which the resource belongs.
If it is not provided, the project will be parsed from the identifier of the parent resource. If no project is provided in the parent identifier and no project is specified, the provider project is used.
"Â
role" ÿThe role that should be applied. Only one
`gcp.workstations.WorkstationConfigIamBinding` can be used per role. Note that custom roles must be of the format
`[projects|organizations]/{parent-name}/roles/{role-name}`.
"
workstationClusterId" "
workstationConfigId" *¬
r
workstationsWorkstationConfigIamPolicyFgcp:workstations/workstationConfigIamPolicy:WorkstationConfigIamPolicyÏ

## Import

For all import syntaxes, the "resource in question" can take any of the following forms:

* projects/{{project}}/locations/{{location}}/workstationClusters/{{workstation_cluster_id}}/workstationConfigs/{{workstation_config_id}}

* {{project}}/{{location}}/{{workstation_cluster_id}}/{{workstation_config_id}}

* {{location}}/{{workstation_cluster_id}}/{{workstation_config_id}}

* {{workstation_config_id}}

Any variables not passed in the import command will be taken from the provider configuration.

Cloud Workstations workstationconfig IAM resources can be imported using the resource identifiers, role, and member.

IAM member imports use space-delimited identifiers: the resource in question, the role, and the member identity, e.g.

```sh
$ pulumi import gcp:workstations/workstationConfigIamPolicy:WorkstationConfigIamPolicy editor "projects/{{project}}/locations/{{location}}/workstationClusters/{{workstation_cluster_id}}/workstationConfigs/{{workstation_config_id}} roles/viewer user:jane@example.com"
```

IAM binding imports use space-delimited identifiers: the resource in question and the role, e.g.

```sh
$ pulumi import gcp:workstations/workstationConfigIamPolicy:WorkstationConfigIamPolicy editor "projects/{{project}}/locations/{{location}}/workstationClusters/{{workstation_cluster_id}}/workstationConfigs/{{workstation_config_id}} roles/viewer"
```

IAM policy imports use the identifier of the resource in question, e.g.

```sh
$ pulumi import gcp:workstations/workstationConfigIamPolicy:WorkstationConfigIamPolicy editor projects/{{project}}/locations/{{location}}/workstationClusters/{{workstation_cluster_id}}/workstationConfigs/{{workstation_config_id}}
```

-> **Custom Roles** If you're importing a IAM resource with a custom role, make sure to use the

 full name of the custom role, e.g. `[projects/my-project|organizations/my-org]/roles/my-custom-role`.

Ê
locationB" ”The location where the workstation cluster config should reside.
Used to find the parent resource to bind the IAM policy to. If not specified,
the value will be parsed from the identifier of the parent resource. If no location is provided in the parent identifier and no
location is specified, it is taken from the provider configuration.
_

policyData" MThe policy data generated by
a `gcp.organizations.getIAMPolicy` data source.
ì
projectB" ÅThe ID of the project in which the resource belongs.
If it is not provided, the project will be parsed from the identifier of the parent resource. If no project is provided in the parent identifier and no project is specified, the provider project is used.

workstationClusterId" 
workstationConfigId" "3
etag" '(Computed) The etag of the IAM policy.
"‰
location" ”The location where the workstation cluster config should reside.
Used to find the parent resource to bind the IAM policy to. If not specified,
the value will be parsed from the identifier of the parent resource. If no location is provided in the parent identifier and no
location is specified, it is taken from the provider configuration.
"_

policyData" MThe policy data generated by
a `gcp.organizations.getIAMPolicy` data source.
"ë
project" ÅThe ID of the project in which the resource belongs.
If it is not provided, the project will be parsed from the identifier of the parent resource. If no project is provided in the parent identifier and no project is specified, the provider project is used.
"
workstationClusterId" "
workstationConfigId" *∞5
c
workstationsWorkstationIamBinding<gcp:workstations/workstationIamBinding:WorkstationIamBindingÁ

## Import

For all import syntaxes, the "resource in question" can take any of the following forms:

* projects/{{project}}/locations/{{location}}/workstationClusters/{{workstation_cluster_id}}/workstationConfigs/{{workstation_config_id}}/workstations/{{workstation_id}}

* {{project}}/{{location}}/{{workstation_cluster_id}}/{{workstation_config_id}}/{{workstation_id}}

* {{location}}/{{workstation_cluster_id}}/{{workstation_config_id}}/{{workstation_id}}

* {{workstation_id}}

Any variables not passed in the import command will be taken from the provider configuration.

Cloud Workstations workstation IAM resources can be imported using the resource identifiers, role, and member.

IAM member imports use space-delimited identifiers: the resource in question, the role, and the member identity, e.g.

```sh
$ pulumi import gcp:workstations/workstationIamBinding:WorkstationIamBinding editor "projects/{{project}}/locations/{{location}}/workstationClusters/{{workstation_cluster_id}}/workstationConfigs/{{workstation_config_id}}/workstations/{{workstation_id}} roles/viewer user:jane@example.com"
```

IAM binding imports use space-delimited identifiers: the resource in question and the role, e.g.

```sh
$ pulumi import gcp:workstations/workstationIamBinding:WorkstationIamBinding editor "projects/{{project}}/locations/{{location}}/workstationClusters/{{workstation_cluster_id}}/workstationConfigs/{{workstation_config_id}}/workstations/{{workstation_id}} roles/viewer"
```

IAM policy imports use the identifier of the resource in question, e.g.

```sh
$ pulumi import gcp:workstations/workstationIamBinding:WorkstationIamBinding editor projects/{{project}}/locations/{{location}}/workstationClusters/{{workstation_cluster_id}}/workstationConfigs/{{workstation_config_id}}/workstations/{{workstation_id}}
```

-> **Custom Roles** If you're importing a IAM resource with a custom role, make sure to use the

 full name of the custom role, e.g. `[projects/my-project|organizations/my-org]/roles/my-custom-role`.

î
	conditionÜBÉ:Ä
~
workstationsWorkstationIamBindingConditionNgcp:workstations/WorkstationIamBindingCondition:WorkstationIamBindingCondition·
locationB" ŒThe location where the workstation parent resources reside.
Used to find the parent resource to bind the IAM policy to. If not specified,
the value will be parsed from the identifier of the parent resource. If no location is provided in the parent identifier and no
location is specified, it is taken from the provider configuration.
÷	
members*" ƒ	Identities that will be granted the privilege in `role`.
Each entry can have one of the following values:
* **allUsers**: A special identifier that represents anyone who is on the internet; with or without a Google account.
* **allAuthenticatedUsers**: A special identifier that represents anyone who is authenticated with a Google account or a service account.
* **user:{emailid}**: An email address that represents a specific Google account. For example, alice@gmail.com or joe@example.com.
* **serviceAccount:{emailid}**: An email address that represents a service account. For example, my-other-app@appspot.gserviceaccount.com.
* **group:{emailid}**: An email address that represents a Google group. For example, admins@example.com.
* **domain:{domain}**: A G Suite domain (primary, instead of alias) name that represents all the users of that domain. For example, google.com or example.com.
* **projectOwner:projectid**: Owners of the given project. For example, "projectOwner:my-example-project"
* **projectEditor:projectid**: Editors of the given project. For example, "projectEditor:my-example-project"
* **projectViewer:projectid**: Viewers of the given project. For example, "projectViewer:my-example-project"
ì
projectB" ÅThe ID of the project in which the resource belongs.
If it is not provided, the project will be parsed from the identifier of the parent resource. If no project is provided in the parent identifier and no project is specified, the provider project is used.
ﬂ
role" “The role that should be applied. Only one
`gcp.workstations.WorkstationIamBinding` can be used per role. Note that custom roles must be of the format
`[projects|organizations]/{parent-name}/roles/{role-name}`.

workstationClusterId" 
workstationConfigId" 
workstationId" "î
	conditionÜBÉ:Ä
~
workstationsWorkstationIamBindingConditionNgcp:workstations/WorkstationIamBindingCondition:WorkstationIamBindingCondition"3
etag" '(Computed) The etag of the IAM policy.
"ﬂ
location" ŒThe location where the workstation parent resources reside.
Used to find the parent resource to bind the IAM policy to. If not specified,
the value will be parsed from the identifier of the parent resource. If no location is provided in the parent identifier and no
location is specified, it is taken from the provider configuration.
"÷	
members*" ƒ	Identities that will be granted the privilege in `role`.
Each entry can have one of the following values:
* **allUsers**: A special identifier that represents anyone who is on the internet; with or without a Google account.
* **allAuthenticatedUsers**: A special identifier that represents anyone who is authenticated with a Google account or a service account.
* **user:{emailid}**: An email address that represents a specific Google account. For example, alice@gmail.com or joe@example.com.
* **serviceAccount:{emailid}**: An email address that represents a service account. For example, my-other-app@appspot.gserviceaccount.com.
* **group:{emailid}**: An email address that represents a Google group. For example, admins@example.com.
* **domain:{domain}**: A G Suite domain (primary, instead of alias) name that represents all the users of that domain. For example, google.com or example.com.
* **projectOwner:projectid**: Owners of the given project. For example, "projectOwner:my-example-project"
* **projectEditor:projectid**: Editors of the given project. For example, "projectEditor:my-example-project"
* **projectViewer:projectid**: Viewers of the given project. For example, "projectViewer:my-example-project"
"ë
project" ÅThe ID of the project in which the resource belongs.
If it is not provided, the project will be parsed from the identifier of the parent resource. If no project is provided in the parent identifier and no project is specified, the provider project is used.
"ﬂ
role" “The role that should be applied. Only one
`gcp.workstations.WorkstationIamBinding` can be used per role. Note that custom roles must be of the format
`[projects|organizations]/{parent-name}/roles/{role-name}`.
"
workstationClusterId" "
workstationConfigId" "
workstationId" *ó5
`
workstationsWorkstationIamMember:gcp:workstations/workstationIamMember:WorkstationIamMember·

## Import

For all import syntaxes, the "resource in question" can take any of the following forms:

* projects/{{project}}/locations/{{location}}/workstationClusters/{{workstation_cluster_id}}/workstationConfigs/{{workstation_config_id}}/workstations/{{workstation_id}}

* {{project}}/{{location}}/{{workstation_cluster_id}}/{{workstation_config_id}}/{{workstation_id}}

* {{location}}/{{workstation_cluster_id}}/{{workstation_config_id}}/{{workstation_id}}

* {{workstation_id}}

Any variables not passed in the import command will be taken from the provider configuration.

Cloud Workstations workstation IAM resources can be imported using the resource identifiers, role, and member.

IAM member imports use space-delimited identifiers: the resource in question, the role, and the member identity, e.g.

```sh
$ pulumi import gcp:workstations/workstationIamMember:WorkstationIamMember editor "projects/{{project}}/locations/{{location}}/workstationClusters/{{workstation_cluster_id}}/workstationConfigs/{{workstation_config_id}}/workstations/{{workstation_id}} roles/viewer user:jane@example.com"
```

IAM binding imports use space-delimited identifiers: the resource in question and the role, e.g.

```sh
$ pulumi import gcp:workstations/workstationIamMember:WorkstationIamMember editor "projects/{{project}}/locations/{{location}}/workstationClusters/{{workstation_cluster_id}}/workstationConfigs/{{workstation_config_id}}/workstations/{{workstation_id}} roles/viewer"
```

IAM policy imports use the identifier of the resource in question, e.g.

```sh
$ pulumi import gcp:workstations/workstationIamMember:WorkstationIamMember editor projects/{{project}}/locations/{{location}}/workstationClusters/{{workstation_cluster_id}}/workstationConfigs/{{workstation_config_id}}/workstations/{{workstation_id}}
```

-> **Custom Roles** If you're importing a IAM resource with a custom role, make sure to use the

 full name of the custom role, e.g. `[projects/my-project|organizations/my-org]/roles/my-custom-role`.

è
	conditionÅB:}
{
workstationsWorkstationIamMemberConditionLgcp:workstations/WorkstationIamMemberCondition:WorkstationIamMemberCondition·
locationB" ŒThe location where the workstation parent resources reside.
Used to find the parent resource to bind the IAM policy to. If not specified,
the value will be parsed from the identifier of the parent resource. If no location is provided in the parent identifier and no
location is specified, it is taken from the provider configuration.
”	
member" ƒ	Identities that will be granted the privilege in `role`.
Each entry can have one of the following values:
* **allUsers**: A special identifier that represents anyone who is on the internet; with or without a Google account.
* **allAuthenticatedUsers**: A special identifier that represents anyone who is authenticated with a Google account or a service account.
* **user:{emailid}**: An email address that represents a specific Google account. For example, alice@gmail.com or joe@example.com.
* **serviceAccount:{emailid}**: An email address that represents a service account. For example, my-other-app@appspot.gserviceaccount.com.
* **group:{emailid}**: An email address that represents a Google group. For example, admins@example.com.
* **domain:{domain}**: A G Suite domain (primary, instead of alias) name that represents all the users of that domain. For example, google.com or example.com.
* **projectOwner:projectid**: Owners of the given project. For example, "projectOwner:my-example-project"
* **projectEditor:projectid**: Editors of the given project. For example, "projectEditor:my-example-project"
* **projectViewer:projectid**: Viewers of the given project. For example, "projectViewer:my-example-project"
ì
projectB" ÅThe ID of the project in which the resource belongs.
If it is not provided, the project will be parsed from the identifier of the parent resource. If no project is provided in the parent identifier and no project is specified, the provider project is used.
ﬂ
role" “The role that should be applied. Only one
`gcp.workstations.WorkstationIamBinding` can be used per role. Note that custom roles must be of the format
`[projects|organizations]/{parent-name}/roles/{role-name}`.

workstationClusterId" 
workstationConfigId" 
workstationId" "è
	conditionÅB:}
{
workstationsWorkstationIamMemberConditionLgcp:workstations/WorkstationIamMemberCondition:WorkstationIamMemberCondition"3
etag" '(Computed) The etag of the IAM policy.
"ﬂ
location" ŒThe location where the workstation parent resources reside.
Used to find the parent resource to bind the IAM policy to. If not specified,
the value will be parsed from the identifier of the parent resource. If no location is provided in the parent identifier and no
location is specified, it is taken from the provider configuration.
"”	
member" ƒ	Identities that will be granted the privilege in `role`.
Each entry can have one of the following values:
* **allUsers**: A special identifier that represents anyone who is on the internet; with or without a Google account.
* **allAuthenticatedUsers**: A special identifier that represents anyone who is authenticated with a Google account or a service account.
* **user:{emailid}**: An email address that represents a specific Google account. For example, alice@gmail.com or joe@example.com.
* **serviceAccount:{emailid}**: An email address that represents a service account. For example, my-other-app@appspot.gserviceaccount.com.
* **group:{emailid}**: An email address that represents a Google group. For example, admins@example.com.
* **domain:{domain}**: A G Suite domain (primary, instead of alias) name that represents all the users of that domain. For example, google.com or example.com.
* **projectOwner:projectid**: Owners of the given project. For example, "projectOwner:my-example-project"
* **projectEditor:projectid**: Editors of the given project. For example, "projectEditor:my-example-project"
* **projectViewer:projectid**: Viewers of the given project. For example, "projectViewer:my-example-project"
"ë
project" ÅThe ID of the project in which the resource belongs.
If it is not provided, the project will be parsed from the identifier of the parent resource. If no project is provided in the parent identifier and no project is specified, the provider project is used.
"ﬂ
role" “The role that should be applied. Only one
`gcp.workstations.WorkstationIamBinding` can be used per role. Note that custom roles must be of the format
`[projects|organizations]/{parent-name}/roles/{role-name}`.
"
workstationClusterId" "
workstationConfigId" "
workstationId" *≈
`
workstationsWorkstationIamPolicy:gcp:workstations/workstationIamPolicy:WorkstationIamPolicy·

## Import

For all import syntaxes, the "resource in question" can take any of the following forms:

* projects/{{project}}/locations/{{location}}/workstationClusters/{{workstation_cluster_id}}/workstationConfigs/{{workstation_config_id}}/workstations/{{workstation_id}}

* {{project}}/{{location}}/{{workstation_cluster_id}}/{{workstation_config_id}}/{{workstation_id}}

* {{location}}/{{workstation_cluster_id}}/{{workstation_config_id}}/{{workstation_id}}

* {{workstation_id}}

Any variables not passed in the import command will be taken from the provider configuration.

Cloud Workstations workstation IAM resources can be imported using the resource identifiers, role, and member.

IAM member imports use space-delimited identifiers: the resource in question, the role, and the member identity, e.g.

```sh
$ pulumi import gcp:workstations/workstationIamPolicy:WorkstationIamPolicy editor "projects/{{project}}/locations/{{location}}/workstationClusters/{{workstation_cluster_id}}/workstationConfigs/{{workstation_config_id}}/workstations/{{workstation_id}} roles/viewer user:jane@example.com"
```

IAM binding imports use space-delimited identifiers: the resource in question and the role, e.g.

```sh
$ pulumi import gcp:workstations/workstationIamPolicy:WorkstationIamPolicy editor "projects/{{project}}/locations/{{location}}/workstationClusters/{{workstation_cluster_id}}/workstationConfigs/{{workstation_config_id}}/workstations/{{workstation_id}} roles/viewer"
```

IAM policy imports use the identifier of the resource in question, e.g.

```sh
$ pulumi import gcp:workstations/workstationIamPolicy:WorkstationIamPolicy editor projects/{{project}}/locations/{{location}}/workstationClusters/{{workstation_cluster_id}}/workstationConfigs/{{workstation_config_id}}/workstations/{{workstation_id}}
```

-> **Custom Roles** If you're importing a IAM resource with a custom role, make sure to use the

 full name of the custom role, e.g. `[projects/my-project|organizations/my-org]/roles/my-custom-role`.

·
locationB" ŒThe location where the workstation parent resources reside.
Used to find the parent resource to bind the IAM policy to. If not specified,
the value will be parsed from the identifier of the parent resource. If no location is provided in the parent identifier and no
location is specified, it is taken from the provider configuration.
_

policyData" MThe policy data generated by
a `gcp.organizations.getIAMPolicy` data source.
ì
projectB" ÅThe ID of the project in which the resource belongs.
If it is not provided, the project will be parsed from the identifier of the parent resource. If no project is provided in the parent identifier and no project is specified, the provider project is used.

workstationClusterId" 
workstationConfigId" 
workstationId" "3
etag" '(Computed) The etag of the IAM policy.
"ﬂ
location" ŒThe location where the workstation parent resources reside.
Used to find the parent resource to bind the IAM policy to. If not specified,
the value will be parsed from the identifier of the parent resource. If no location is provided in the parent identifier and no
location is specified, it is taken from the provider configuration.
"_

policyData" MThe policy data generated by
a `gcp.organizations.getIAMPolicy` data source.
"ë
project" ÅThe ID of the project in which the resource belongs.
If it is not provided, the project will be parsed from the identifier of the parent resource. If no project is provided in the parent identifier and no project is specified, the provider project is used.
"
workstationClusterId" "
workstationConfigId" "
workstationId" 2õ
B
vmwareengine
getCluster&gcp:vmwareengine/getCluster:getCluster≥Use this data source to get details about a cluster resource.

To get more information about private cloud cluster, see:
* [API documentation](https://cloud.google.com/vmware-engine/docs/reference/rest/v1/projects.locations.privateClouds.clusters)

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as gcp from "@pulumi/gcp";

const myCluster = gcp.vmwareengine.getCluster({
    name: "my-cluster",
    parent: "project/locations/us-west1-a/privateClouds/my-cloud",
});
```
```python
import pulumi
import pulumi_gcp as gcp

my_cluster = gcp.vmwareengine.get_cluster(name="my-cluster",
    parent="project/locations/us-west1-a/privateClouds/my-cloud")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Gcp = Pulumi.Gcp;

return await Deployment.RunAsync(() => 
{
    var myCluster = Gcp.VMwareEngine.GetCluster.Invoke(new()
    {
        Name = "my-cluster",
        Parent = "project/locations/us-west1-a/privateClouds/my-cloud",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-gcp/sdk/v8/go/gcp/vmwareengine"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := vmwareengine.LookupCluster(ctx, &vmwareengine.LookupClusterArgs{
			Name:   "my-cluster",
			Parent: "project/locations/us-west1-a/privateClouds/my-cloud",
		}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.gcp.vmwareengine.VmwareengineFunctions;
import com.pulumi.gcp.vmwareengine.inputs.GetClusterArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var myCluster = VmwareengineFunctions.getCluster(GetClusterArgs.builder()
            .name("my-cluster")
            .parent("project/locations/us-west1-a/privateClouds/my-cloud")
            .build());

    }
}
```
```yaml
variables:
  myCluster:
    fn::invoke:
      function: gcp:vmwareengine:getCluster
      arguments:
        name: my-cluster
        parent: project/locations/us-west1-a/privateClouds/my-cloud
```
<!--End PulumiCodeChooser -->
"
name" Name of the resource.
P
parent" BThe resource name of the private cloud that this cluster belongs.
"ï
autoscalingSettings~*|:z
x
vmwareenginegetClusterAutoscalingSettingJgcp:vmwareengine/getClusterAutoscalingSetting:getClusterAutoscalingSetting"E
id" ;The provider-assigned unique ID for this managed resource.
"

management
 "

name" "Ö
nodeTypeConfigsr*p:n
l
vmwareenginegetClusterNodeTypeConfigBgcp:vmwareengine/getClusterNodeTypeConfig:getClusterNodeTypeConfig"
parent" "
state" "	
uid" 2å
c
vmwareenginegetExternalAccessRule<gcp:vmwareengine/getExternalAccessRule:getExternalAccessRule∏Use this data source to get details about a external access rule resource.

To get more information about external address, see:
* [API documentation](https://cloud.google.com/vmware-engine/docs/reference/rest/v1/projects.locations.networkPolicies.externalAccessRules)

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as gcp from "@pulumi/gcp";

const myExternalAccessRule = gcp.vmwareengine.getExternalAccessRule({
    name: "my-external-access-rule",
    parent: "project/my-project/locations/us-west1-a/networkPolicies/my-network-policy",
});
```
```python
import pulumi
import pulumi_gcp as gcp

my_external_access_rule = gcp.vmwareengine.get_external_access_rule(name="my-external-access-rule",
    parent="project/my-project/locations/us-west1-a/networkPolicies/my-network-policy")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Gcp = Pulumi.Gcp;

return await Deployment.RunAsync(() => 
{
    var myExternalAccessRule = Gcp.VMwareEngine.GetExternalAccessRule.Invoke(new()
    {
        Name = "my-external-access-rule",
        Parent = "project/my-project/locations/us-west1-a/networkPolicies/my-network-policy",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-gcp/sdk/v8/go/gcp/vmwareengine"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := vmwareengine.LookupExternalAccessRule(ctx, &vmwareengine.LookupExternalAccessRuleArgs{
			Name:   "my-external-access-rule",
			Parent: "project/my-project/locations/us-west1-a/networkPolicies/my-network-policy",
		}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.gcp.vmwareengine.VmwareengineFunctions;
import com.pulumi.gcp.vmwareengine.inputs.GetExternalAccessRuleArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var myExternalAccessRule = VmwareengineFunctions.getExternalAccessRule(GetExternalAccessRuleArgs.builder()
            .name("my-external-access-rule")
            .parent("project/my-project/locations/us-west1-a/networkPolicies/my-network-policy")
            .build());

    }
}
```
```yaml
variables:
  myExternalAccessRule:
    fn::invoke:
      function: gcp:vmwareengine:getExternalAccessRule
      arguments:
        name: my-external-access-rule
        parent: project/my-project/locations/us-west1-a/networkPolicies/my-network-policy
```
<!--End PulumiCodeChooser -->
"
name" Name of the resource.
Q
parent" CThe resource name of the network policy that this cluster belongs.
"
action" "

createTime" "
description" "∫
destinationIpRanges¢*ü:ú
ô
vmwareengine'getExternalAccessRuleDestinationIpRange`gcp:vmwareengine/getExternalAccessRuleDestinationIpRange:getExternalAccessRuleDestinationIpRange"
destinationPorts*" "E
id" ;The provider-assigned unique ID for this managed resource.
"

ipProtocol" "

name" "
parent" "
priority "¶
sourceIpRangesì*ê:ç
ä
vmwareengine"getExternalAccessRuleSourceIpRangeVgcp:vmwareengine/getExternalAccessRuleSourceIpRange:getExternalAccessRuleSourceIpRange"
sourcePorts*" "
state" "	
uid" "

updateTime" 2”
Z
vmwareenginegetExternalAddress6gcp:vmwareengine/getExternalAddress:getExternalAddress™Use this data source to get details about a external address resource.

To get more information about external address, see:
* [API documentation](https://cloud.google.com/vmware-engine/docs/reference/rest/v1/projects.locations.privateClouds.externalAddresses)

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as gcp from "@pulumi/gcp";

const myExternalAddress = gcp.vmwareengine.getExternalAddress({
    name: "my-external-address",
    parent: "project/my-project/locations/us-west1-a/privateClouds/my-cloud",
});
```
```python
import pulumi
import pulumi_gcp as gcp

my_external_address = gcp.vmwareengine.get_external_address(name="my-external-address",
    parent="project/my-project/locations/us-west1-a/privateClouds/my-cloud")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Gcp = Pulumi.Gcp;

return await Deployment.RunAsync(() => 
{
    var myExternalAddress = Gcp.VMwareEngine.GetExternalAddress.Invoke(new()
    {
        Name = "my-external-address",
        Parent = "project/my-project/locations/us-west1-a/privateClouds/my-cloud",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-gcp/sdk/v8/go/gcp/vmwareengine"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := vmwareengine.LookupExternalAddress(ctx, &vmwareengine.LookupExternalAddressArgs{
			Name:   "my-external-address",
			Parent: "project/my-project/locations/us-west1-a/privateClouds/my-cloud",
		}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.gcp.vmwareengine.VmwareengineFunctions;
import com.pulumi.gcp.vmwareengine.inputs.GetExternalAddressArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var myExternalAddress = VmwareengineFunctions.getExternalAddress(GetExternalAddressArgs.builder()
            .name("my-external-address")
            .parent("project/my-project/locations/us-west1-a/privateClouds/my-cloud")
            .build());

    }
}
```
```yaml
variables:
  myExternalAddress:
    fn::invoke:
      function: gcp:vmwareengine:getExternalAddress
      arguments:
        name: my-external-address
        parent: project/my-project/locations/us-west1-a/privateClouds/my-cloud
```
<!--End PulumiCodeChooser -->
"
name" Name of the resource.
P
parent" BThe resource name of the private cloud that this cluster belongs.
"

createTime" "
description" "

externalIp" "E
id" ;The provider-assigned unique ID for this managed resource.
"

internalIp" "

name" "
parent" "
state" "	
uid" "

updateTime" 2µ
B
vmwareengine
getNetwork&gcp:vmwareengine/getNetwork:getNetwork¯Use this data source to get details about a VMwareEngine network resource.

To get more information about VMwareEngine Network, see:
* [API documentation](https://cloud.google.com/vmware-engine/docs/reference/rest/v1/projects.locations.vmwareEngineNetworks)

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as gcp from "@pulumi/gcp";

const myNw = gcp.vmwareengine.getNetwork({
    name: "us-central1-default",
    location: "us-central1",
});
```
```python
import pulumi
import pulumi_gcp as gcp

my_nw = gcp.vmwareengine.get_network(name="us-central1-default",
    location="us-central1")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Gcp = Pulumi.Gcp;

return await Deployment.RunAsync(() => 
{
    var myNw = Gcp.VMwareEngine.GetNetwork.Invoke(new()
    {
        Name = "us-central1-default",
        Location = "us-central1",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-gcp/sdk/v8/go/gcp/vmwareengine"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := vmwareengine.LookupNetwork(ctx, &vmwareengine.LookupNetworkArgs{
			Name:     "us-central1-default",
			Location: "us-central1",
		}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.gcp.vmwareengine.VmwareengineFunctions;
import com.pulumi.gcp.vmwareengine.inputs.GetNetworkArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var myNw = VmwareengineFunctions.getNetwork(GetNetworkArgs.builder()
            .name("us-central1-default")
            .location("us-central1")
            .build());

    }
}
```
```yaml
variables:
  myNw:
    fn::invoke:
      function: gcp:vmwareengine:getNetwork
      arguments:
        name: us-central1-default
        location: us-central1
```
<!--End PulumiCodeChooser -->
1
location" !Location of the resource.

- - -
"
name" Name of the resource.
{
projectB" jThe ID of the project in which the resource belongs. If it is not provided, the provider project is used.
"
description" "E
id" ;The provider-assigned unique ID for this managed resource.
"
location" "

name" "
projectB" "
state" "

type" "	
uid" "u
vpcNetworksf*d:b
`
vmwareenginegetNetworkVpcNetwork:gcp:vmwareengine/getNetworkVpcNetwork:getNetworkVpcNetwork2„
W
vmwareenginegetNetworkPeering4gcp:vmwareengine/getNetworkPeering:getNetworkPeering¢Use this data source to get details about a network peering resource.

To get more information about network peering, see:
* [API documentation](https://cloud.google.com/vmware-engine/docs/reference/rest/v1/projects.locations.networkPeerings)

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as gcp from "@pulumi/gcp";

const myNetworkPeering = gcp.vmwareengine.getNetworkPeering({
    name: "my-network-peering",
});
```
```python
import pulumi
import pulumi_gcp as gcp

my_network_peering = gcp.vmwareengine.get_network_peering(name="my-network-peering")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Gcp = Pulumi.Gcp;

return await Deployment.RunAsync(() => 
{
    var myNetworkPeering = Gcp.VMwareEngine.GetNetworkPeering.Invoke(new()
    {
        Name = "my-network-peering",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-gcp/sdk/v8/go/gcp/vmwareengine"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := vmwareengine.LookupNetworkPeering(ctx, &vmwareengine.LookupNetworkPeeringArgs{
			Name: "my-network-peering",
		}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.gcp.vmwareengine.VmwareengineFunctions;
import com.pulumi.gcp.vmwareengine.inputs.GetNetworkPeeringArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var myNetworkPeering = VmwareengineFunctions.getNetworkPeering(GetNetworkPeeringArgs.builder()
            .name("my-network-peering")
            .build());

    }
}
```
```yaml
variables:
  myNetworkPeering:
    fn::invoke:
      function: gcp:vmwareengine:getNetworkPeering
      arguments:
        name: my-network-peering
```
<!--End PulumiCodeChooser -->
"
name" Name of the resource.

projectB" "

createTime" "
description" "
exportCustomRoutes
 "$
exportCustomRoutesWithPublicIp
 "E
id" ;The provider-assigned unique ID for this managed resource.
"
importCustomRoutes
 "$
importCustomRoutesWithPublicIp
 "

name" "
peerNetwork" "
peerNetworkType" "
projectB" "
state" "
stateDetails" "	
uid" "

updateTime" "
vmwareEngineNetwork" ""
vmwareEngineNetworkCanonical" 2π
T
vmwareenginegetNetworkPolicy2gcp:vmwareengine/getNetworkPolicy:getNetworkPolicy Use this data source to get details about a network policy resource.

To get more information about network policy, see:
* [API documentation](https://cloud.google.com/vmware-engine/docs/reference/rest/v1/projects.locations.networkPolicies)

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as gcp from "@pulumi/gcp";

const myNetworkPolicy = gcp.vmwareengine.getNetworkPolicy({
    name: "my-network-policy",
    location: "us-central1",
});
```
```python
import pulumi
import pulumi_gcp as gcp

my_network_policy = gcp.vmwareengine.get_network_policy(name="my-network-policy",
    location="us-central1")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Gcp = Pulumi.Gcp;

return await Deployment.RunAsync(() => 
{
    var myNetworkPolicy = Gcp.VMwareEngine.GetNetworkPolicy.Invoke(new()
    {
        Name = "my-network-policy",
        Location = "us-central1",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-gcp/sdk/v8/go/gcp/vmwareengine"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := vmwareengine.LookupNetworkPolicy(ctx, &vmwareengine.LookupNetworkPolicyArgs{
			Name:     "my-network-policy",
			Location: "us-central1",
		}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.gcp.vmwareengine.VmwareengineFunctions;
import com.pulumi.gcp.vmwareengine.inputs.GetNetworkPolicyArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var myNetworkPolicy = VmwareengineFunctions.getNetworkPolicy(GetNetworkPolicyArgs.builder()
            .name("my-network-policy")
            .location("us-central1")
            .build());

    }
}
```
```yaml
variables:
  myNetworkPolicy:
    fn::invoke:
      function: gcp:vmwareengine:getNetworkPolicy
      arguments:
        name: my-network-policy
        location: us-central1
```
<!--End PulumiCodeChooser -->
*
location" Location of the resource.
"
name" Name of the resource.

projectB" "

createTime" "
description" "
edgeServicesCidr" "á
externalIpsx*v:t
r
vmwareenginegetNetworkPolicyExternalIpFgcp:vmwareengine/getNetworkPolicyExternalIp:getNetworkPolicyExternalIp"E
id" ;The provider-assigned unique ID for this managed resource.
"õ
internetAccessesÜ*É:Ä
~
vmwareenginegetNetworkPolicyInternetAccessNgcp:vmwareengine/getNetworkPolicyInternetAccess:getNetworkPolicyInternetAccess"
location" "

name" "
projectB" "	
uid" "

updateTime" "
vmwareEngineNetwork" ""
vmwareEngineNetworkCanonical" 2Ù
W
vmwareenginegetNsxCredentials4gcp:vmwareengine/getNsxCredentials:getNsxCredentialsàUse this data source to get NSX credentials for a Private Cloud.

To get more information about private cloud NSX credentials, see:
* [API documentation](https://cloud.google.com/vmware-engine/docs/reference/rest/v1/projects.locations.privateClouds/showNsxCredentials)

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as gcp from "@pulumi/gcp";

const ds = gcp.vmwareengine.getNsxCredentials({
    parent: "projects/my-project/locations/us-west1-a/privateClouds/my-cloud",
});
```
```python
import pulumi
import pulumi_gcp as gcp

ds = gcp.vmwareengine.get_nsx_credentials(parent="projects/my-project/locations/us-west1-a/privateClouds/my-cloud")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Gcp = Pulumi.Gcp;

return await Deployment.RunAsync(() => 
{
    var ds = Gcp.VMwareEngine.GetNsxCredentials.Invoke(new()
    {
        Parent = "projects/my-project/locations/us-west1-a/privateClouds/my-cloud",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-gcp/sdk/v8/go/gcp/vmwareengine"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := vmwareengine.GetNsxCredentials(ctx, &vmwareengine.GetNsxCredentialsArgs{
			Parent: "projects/my-project/locations/us-west1-a/privateClouds/my-cloud",
		}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.gcp.vmwareengine.VmwareengineFunctions;
import com.pulumi.gcp.vmwareengine.inputs.GetNsxCredentialsArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var ds = VmwareengineFunctions.getNsxCredentials(GetNsxCredentialsArgs.builder()
            .parent("projects/my-project/locations/us-west1-a/privateClouds/my-cloud")
            .build());

    }
}
```
```yaml
variables:
  ds:
    fn::invoke:
      function: gcp:vmwareengine:getNsxCredentials
      arguments:
        parent: projects/my-project/locations/us-west1-a/privateClouds/my-cloud
```
<!--End PulumiCodeChooser -->
M
parent" ?The resource name of the private cloud which contains the NSX.
"E
id" ;The provider-assigned unique ID for this managed resource.
"
parent" "4
password" $The password of the NSX Credential.
"4
username" $The username of the NSX Credential.
2Ê
Q
vmwareenginegetPrivateCloud0gcp:vmwareengine/getPrivateCloud:getPrivateCloud…Use this data source to get details about a private cloud resource.

To get more information about private cloud, see:
* [API documentation](https://cloud.google.com/vmware-engine/docs/reference/rest/v1/projects.locations.privateClouds)

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as gcp from "@pulumi/gcp";

const myPc = gcp.vmwareengine.getPrivateCloud({
    name: "my-pc",
    location: "us-central1-a",
});
```
```python
import pulumi
import pulumi_gcp as gcp

my_pc = gcp.vmwareengine.get_private_cloud(name="my-pc",
    location="us-central1-a")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Gcp = Pulumi.Gcp;

return await Deployment.RunAsync(() => 
{
    var myPc = Gcp.VMwareEngine.GetPrivateCloud.Invoke(new()
    {
        Name = "my-pc",
        Location = "us-central1-a",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-gcp/sdk/v8/go/gcp/vmwareengine"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := vmwareengine.LookupPrivateCloud(ctx, &vmwareengine.LookupPrivateCloudArgs{
			Name:     "my-pc",
			Location: "us-central1-a",
		}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.gcp.vmwareengine.VmwareengineFunctions;
import com.pulumi.gcp.vmwareengine.inputs.GetPrivateCloudArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var myPc = VmwareengineFunctions.getPrivateCloud(GetPrivateCloudArgs.builder()
            .name("my-pc")
            .location("us-central1-a")
            .build());

    }
}
```
```yaml
variables:
  myPc:
    fn::invoke:
      function: gcp:vmwareengine:getPrivateCloud
      arguments:
        name: my-pc
        location: us-central1-a
```
<!--End PulumiCodeChooser -->
1
location" !Location of the resource.

- - -
"
name" Name of the resource.
{
projectB" jThe ID of the project in which the resource belongs. If it
is not provided, the provider project is used.
"
deletionDelayHours "
description" "i
hcxes`*^:\
Z
vmwareenginegetPrivateCloudHcx6gcp:vmwareengine/getPrivateCloudHcx:getPrivateCloudHcx"E
id" ;The provider-assigned unique ID for this managed resource.
"
location" "§
managementClustersç*ä:á
Ñ
vmwareengine getPrivateCloudManagementClusterRgcp:vmwareengine/getPrivateCloudManagementCluster:getPrivateCloudManagementCluster"

name" "ê
networkConfigs~*|:z
x
vmwareenginegetPrivateCloudNetworkConfigJgcp:vmwareengine/getPrivateCloudNetworkConfig:getPrivateCloudNetworkConfig"i
nsxes`*^:\
Z
vmwareenginegetPrivateCloudNsx6gcp:vmwareengine/getPrivateCloudNsx:getPrivateCloudNsx"
projectB" ""
sendDeletionDelayHoursIfZero
 "
state" "

type" "	
uid" "x
vcentersl*j:h
f
vmwareenginegetPrivateCloudVcenter>gcp:vmwareengine/getPrivateCloudVcenter:getPrivateCloudVcenter2–
?
vmwareengine	getSubnet$gcp:vmwareengine/getSubnet:getSubnetçUse this data source to get details about a subnet. Management subnets support only read operations and should be configured through this data source. User defined subnets can be configured using the resource as well as the datasource.

To get more information about private cloud subnet, see:
* [API documentation](https://cloud.google.com/vmware-engine/docs/reference/rest/v1/projects.locations.privateClouds.subnets)

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as gcp from "@pulumi/gcp";

const mySubnet = gcp.vmwareengine.getSubnet({
    name: "service-1",
    parent: "project/my-project/locations/us-west1-a/privateClouds/my-cloud",
});
```
```python
import pulumi
import pulumi_gcp as gcp

my_subnet = gcp.vmwareengine.get_subnet(name="service-1",
    parent="project/my-project/locations/us-west1-a/privateClouds/my-cloud")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Gcp = Pulumi.Gcp;

return await Deployment.RunAsync(() => 
{
    var mySubnet = Gcp.VMwareEngine.GetSubnet.Invoke(new()
    {
        Name = "service-1",
        Parent = "project/my-project/locations/us-west1-a/privateClouds/my-cloud",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-gcp/sdk/v8/go/gcp/vmwareengine"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := vmwareengine.LookupSubnet(ctx, &vmwareengine.LookupSubnetArgs{
			Name:   "service-1",
			Parent: "project/my-project/locations/us-west1-a/privateClouds/my-cloud",
		}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.gcp.vmwareengine.VmwareengineFunctions;
import com.pulumi.gcp.vmwareengine.inputs.GetSubnetArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var mySubnet = VmwareengineFunctions.getSubnet(GetSubnetArgs.builder()
            .name("service-1")
            .parent("project/my-project/locations/us-west1-a/privateClouds/my-cloud")
            .build());

    }
}
```
```yaml
variables:
  mySubnet:
    fn::invoke:
      function: gcp:vmwareengine:getSubnet
      arguments:
        name: service-1
        parent: project/my-project/locations/us-west1-a/privateClouds/my-cloud
```
<!--End PulumiCodeChooser -->
ú
name" èName of the resource. 
UserDefined subnets are named in the format of "service-n", where n ranges from 1 to 5.
Management subnets have arbitary names including "vmotion", "vsan", "system-management" etc. More details about subnet names can be found on the cloud console.
O
parent" AThe resource name of the private cloud that this subnet belongs.
"

createTime" "ä
dhcpAddressRangesu*s:q
o
vmwareenginegetSubnetDhcpAddressRangeDgcp:vmwareengine/getSubnetDhcpAddressRange:getSubnetDhcpAddressRange"
	gatewayId" "
	gatewayIp" "E
id" ;The provider-assigned unique ID for this managed resource.
"
ipCidrRange" "

name" "
parent" "
standardConfig
 "
state" "

type" "	
uid" "

updateTime" "
vlanId 2º
c
vmwareenginegetVcenterCredentials<gcp:vmwareengine/getVcenterCredentials:getVcenterCredentials∏Use this data source to get Vcenter credentials for a Private Cloud.

To get more information about private cloud Vcenter credentials, see:
* [API documentation](https://cloud.google.com/vmware-engine/docs/reference/rest/v1/projects.locations.privateClouds/showVcenterCredentials)

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as gcp from "@pulumi/gcp";

const ds = gcp.vmwareengine.getVcenterCredentials({
    parent: "projects/my-project/locations/us-west1-a/privateClouds/my-cloud",
});
```
```python
import pulumi
import pulumi_gcp as gcp

ds = gcp.vmwareengine.get_vcenter_credentials(parent="projects/my-project/locations/us-west1-a/privateClouds/my-cloud")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Gcp = Pulumi.Gcp;

return await Deployment.RunAsync(() => 
{
    var ds = Gcp.VMwareEngine.GetVcenterCredentials.Invoke(new()
    {
        Parent = "projects/my-project/locations/us-west1-a/privateClouds/my-cloud",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-gcp/sdk/v8/go/gcp/vmwareengine"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := vmwareengine.GetVcenterCredentials(ctx, &vmwareengine.GetVcenterCredentialsArgs{
			Parent: "projects/my-project/locations/us-west1-a/privateClouds/my-cloud",
		}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.gcp.vmwareengine.VmwareengineFunctions;
import com.pulumi.gcp.vmwareengine.inputs.GetVcenterCredentialsArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var ds = VmwareengineFunctions.getVcenterCredentials(GetVcenterCredentialsArgs.builder()
            .parent("projects/my-project/locations/us-west1-a/privateClouds/my-cloud")
            .build());

    }
}
```
```yaml
variables:
  ds:
    fn::invoke:
      function: gcp:vmwareengine:getVcenterCredentials
      arguments:
        parent: projects/my-project/locations/us-west1-a/privateClouds/my-cloud
```
<!--End PulumiCodeChooser -->
Q
parent" CThe resource name of the private cloud which contains the Vcenter.
"E
id" ;The provider-assigned unique ID for this managed resource.
"
parent" "8
password" (The password of the Vcenter Credential.
"8
username" (The username of the Vcenter Credential.
2Ñ#
B
	vpcaccessgetConnector'gcp:vpcaccess/getConnector:getConnectorãGet a Serverless VPC Access connector.

To get more information about Connector, see:

* [API documentation](https://cloud.google.com/vpc/docs/reference/vpcaccess/rest/v1/projects.locations.connectors)
* How-to Guides
    * [Configuring Serverless VPC Access](https://cloud.google.com/vpc/docs/configure-serverless-vpc-access)

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as gcp from "@pulumi/gcp";

const sample = gcp.vpcaccess.getConnector({
    name: "vpc-con",
});
const connector = new gcp.vpcaccess.Connector("connector", {
    name: "vpc-con",
    ipCidrRange: "10.8.0.0/28",
    network: "default",
    region: "us-central1",
    minInstances: 2,
    maxInstances: 3,
});
```
```python
import pulumi
import pulumi_gcp as gcp

sample = gcp.vpcaccess.get_connector(name="vpc-con")
connector = gcp.vpcaccess.Connector("connector",
    name="vpc-con",
    ip_cidr_range="10.8.0.0/28",
    network="default",
    region="us-central1",
    min_instances=2,
    max_instances=3)
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Gcp = Pulumi.Gcp;

return await Deployment.RunAsync(() => 
{
    var sample = Gcp.VpcAccess.GetConnector.Invoke(new()
    {
        Name = "vpc-con",
    });

    var connector = new Gcp.VpcAccess.Connector("connector", new()
    {
        Name = "vpc-con",
        IpCidrRange = "10.8.0.0/28",
        Network = "default",
        Region = "us-central1",
        MinInstances = 2,
        MaxInstances = 3,
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-gcp/sdk/v8/go/gcp/vpcaccess"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := vpcaccess.LookupConnector(ctx, &vpcaccess.LookupConnectorArgs{
			Name: "vpc-con",
		}, nil)
		if err != nil {
			return err
		}
		_, err = vpcaccess.NewConnector(ctx, "connector", &vpcaccess.ConnectorArgs{
			Name:         pulumi.String("vpc-con"),
			IpCidrRange:  pulumi.String("10.8.0.0/28"),
			Network:      pulumi.String("default"),
			Region:       pulumi.String("us-central1"),
			MinInstances: pulumi.Int(2),
			MaxInstances: pulumi.Int(3),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.gcp.vpcaccess.VpcaccessFunctions;
import com.pulumi.gcp.vpcaccess.inputs.GetConnectorArgs;
import com.pulumi.gcp.vpcaccess.Connector;
import com.pulumi.gcp.vpcaccess.ConnectorArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var sample = VpcaccessFunctions.getConnector(GetConnectorArgs.builder()
            .name("vpc-con")
            .build());

        var connector = new Connector("connector", ConnectorArgs.builder()
            .name("vpc-con")
            .ipCidrRange("10.8.0.0/28")
            .network("default")
            .region("us-central1")
            .minInstances(2)
            .maxInstances(3)
            .build());

    }
}
```
```yaml
resources:
  connector:
    type: gcp:vpcaccess:Connector
    properties:
      name: vpc-con
      ipCidrRange: 10.8.0.0/28
      network: default
      region: us-central1
      minInstances: 2
      maxInstances: 3
variables:
  sample:
    fn::invoke:
      function: gcp:vpcaccess:getConnector
      arguments:
        name: vpc-con
```
<!--End PulumiCodeChooser -->
)
name" Name of the resource.

- - -
{
projectB" jThe ID of the project in which the resource belongs. If it
is not provided, the provider project is used.
n
regionB" ^The region in which the resource belongs. If it
is not provided, the provider region is used.
"
connectedProjects*" "E
id" ;The provider-assigned unique ID for this managed resource.
"
ipCidrRange" "
machineType" "
maxInstances "
maxThroughput "
minInstances "
minThroughput "

name" "
network" "
projectB" "
regionB" "
selfLink" "
state" "e
subnetsZ*X:V
T
	vpcaccessgetConnectorSubnet3gcp:vpcaccess/getConnectorSubnet:getConnectorSubnet2≥
Z
	workbenchgetInstanceIamPolicy7gcp:workbench/getInstanceIamPolicy:getInstanceIamPolicyŸRetrieves the current IAM policy data for instance


## example

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as gcp from "@pulumi/gcp";

const policy = gcp.workbench.getInstanceIamPolicy({
    project: instance.project,
    location: instance.location,
    name: instance.name,
});
```
```python
import pulumi
import pulumi_gcp as gcp

policy = gcp.workbench.get_instance_iam_policy(project=instance["project"],
    location=instance["location"],
    name=instance["name"])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Gcp = Pulumi.Gcp;

return await Deployment.RunAsync(() => 
{
    var policy = Gcp.Workbench.GetInstanceIamPolicy.Invoke(new()
    {
        Project = instance.Project,
        Location = instance.Location,
        Name = instance.Name,
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-gcp/sdk/v8/go/gcp/workbench"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := workbench.LookupInstanceIamPolicy(ctx, &workbench.LookupInstanceIamPolicyArgs{
			Project:  pulumi.StringRef(instance.Project),
			Location: pulumi.StringRef(instance.Location),
			Name:     instance.Name,
		}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.gcp.workbench.WorkbenchFunctions;
import com.pulumi.gcp.workbench.inputs.GetInstanceIamPolicyArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var policy = WorkbenchFunctions.getInstanceIamPolicy(GetInstanceIamPolicyArgs.builder()
            .project(instance.project())
            .location(instance.location())
            .name(instance.name())
            .build());

    }
}
```
```yaml
variables:
  policy:
    fn::invoke:
      function: gcp:workbench:getInstanceIamPolicy
      arguments:
        project: ${instance.project}
        location: ${instance.location}
        name: ${instance.name}
```
<!--End PulumiCodeChooser -->
⁄
locationB" «Part of `parent`. See documentation of `projectsId`. Used to find the parent resource to bind the IAM policy to. If not specified,
the value will be parsed from the identifier of the parent resource. If no location is provided in the parent identifier and no
location is specified, it is taken from the provider configuration.
G
name" ;Used to find the parent resource to bind the IAM policy to
ì
projectB" ÅThe ID of the project in which the resource belongs.
If it is not provided, the project will be parsed from the identifier of the parent resource. If no project is provided in the parent identifier and no project is specified, the provider project is used.
"3
etag" '(Computed) The etag of the IAM policy.
"E
id" ;The provider-assigned unique ID for this managed resource.
"
location" "

name" "ï

policyData" Ç(Required only by `gcp.workbench.InstanceIamPolicy`) The policy data generated by
a `gcp.organizations.getIAMPolicy` data source.
"
project" 2©	
{
workstationsgetWorkstationConfigIamPolicyLgcp:workstations/getWorkstationConfigIamPolicy:getWorkstationConfigIamPolicyÊ
locationB" ”The location where the workstation cluster config should reside.
Used to find the parent resource to bind the IAM policy to. If not specified,
the value will be parsed from the identifier of the parent resource. If no location is provided in the parent identifier and no
location is specified, it is taken from the provider configuration.
ì
projectB" ÅThe ID of the project in which the resource belongs.
If it is not provided, the project will be parsed from the identifier of the parent resource. If no project is provided in the parent identifier and no project is specified, the provider project is used.

workstationClusterId" 
workstationConfigId" "3
etag" '(Computed) The etag of the IAM policy.
"E
id" ;The provider-assigned unique ID for this managed resource.
"
location" "°

policyData" é(Required only by `gcp.workstations.WorkstationConfigIamPolicy`) The policy data generated by
a `gcp.organizations.getIAMPolicy` data source.
"
project" "
workstationClusterId" "
workstationConfigId" 2∂	
i
workstationsgetWorkstationIamPolicy@gcp:workstations/getWorkstationIamPolicy:getWorkstationIamPolicy·
locationB" ŒThe location where the workstation parent resources reside.
Used to find the parent resource to bind the IAM policy to. If not specified,
the value will be parsed from the identifier of the parent resource. If no location is provided in the parent identifier and no
location is specified, it is taken from the provider configuration.
ì
projectB" ÅThe ID of the project in which the resource belongs.
If it is not provided, the project will be parsed from the identifier of the parent resource. If no project is provided in the parent identifier and no project is specified, the provider project is used.

workstationClusterId" 
workstationConfigId" 
workstationId" "3
etag" '(Computed) The etag of the IAM policy.
"E
id" ;The provider-assigned unique ID for this managed resource.
"
location" "õ

policyData" à(Required only by `gcp.workstations.WorkstationIamPolicy`) The policy data generated by
a `gcp.organizations.getIAMPolicy` data source.
"
project" "
workstationClusterId" "
workstationConfigId" "
workstationId" : 
r
vmwareengineClusterAutoscalingSettingsFgcp:vmwareengine/ClusterAutoscalingSettings:ClusterAutoscalingSettings”

–
”
autoscalingPoliciesÆ*´:®
•
vmwareengine+ClusterAutoscalingSettingsAutoscalingPolicyhgcp:vmwareengine/ClusterAutoscalingSettingsAutoscalingPolicy:ClusterAutoscalingSettingsAutoscalingPolicyäThe map with autoscaling policies applied to the cluster.
The key is the identifier of the policy.
It must meet the following requirements:
* Only contains 1-63 alphanumeric characters and hyphens
* Begins with an alphabetical character
* Ends with a non-hyphen character
* Not formatted as a UUID
* Complies with [RFC 1034](https://datatracker.ietf.org/doc/html/rfc1034) (section 3.5)
Currently the map must contain only one element
that describes the autoscaling policy for compute nodes.
Structure is documented below.
«
coolDownPeriodB" ÆThe minimum duration between consecutive autoscale operations.
It starts once addition or removal of nodes is fully completed.
Minimum cool down period is 30m.
Cool down period must be in whole minutes (for example, 30m, 31m, 50m).
Mandatory for successful addition of autoscaling settings in cluster.
ï
maxClusterNodeCountB xMaximum number of nodes of any type in a cluster.
Mandatory for successful addition of autoscaling settings in cluster.
ï
minClusterNodeCountB xMinimum number of nodes of any type in a cluster.
Mandatory for successful addition of autoscaling settings in cluster.
:
•
vmwareengine+ClusterAutoscalingSettingsAutoscalingPolicyhgcp:vmwareengine/ClusterAutoscalingSettingsAutoscalingPolicy:ClusterAutoscalingSettingsAutoscalingPolicy≈

¬
Q
autoscalePolicyId" 8The identifier for this object. Format specified above.
ı
consumedMemoryThresholds˜BÙ:Ò
Ó
vmwareengineCClusterAutoscalingSettingsAutoscalingPolicyConsumedMemoryThresholdsògcp:vmwareengine/ClusterAutoscalingSettingsAutoscalingPolicyConsumedMemoryThresholds:ClusterAutoscalingSettingsAutoscalingPolicyConsumedMemoryThresholds_Utilization thresholds pertaining to amount of consumed memory.
Structure is documented below.
ø
cpuThresholds÷B”:–
Õ
vmwareengine8ClusterAutoscalingSettingsAutoscalingPolicyCpuThresholdsÇgcp:vmwareengine/ClusterAutoscalingSettingsAutoscalingPolicyCpuThresholds:ClusterAutoscalingSettingsAutoscalingPolicyCpuThresholdsUUtilization thresholds pertaining to CPU utilization.
Structure is documented below.
N

nodeTypeId" <The canonical identifier of the node type to add or remove.
Ö
scaleOutSize qNumber of nodes to add to a cluster during a scale-out operation.
Must be divisible by 2 for stretched clusters.
⁄
storageThresholds‚Bﬂ:‹
Ÿ
vmwareengine<ClusterAutoscalingSettingsAutoscalingPolicyStorageThresholdsägcp:vmwareengine/ClusterAutoscalingSettingsAutoscalingPolicyStorageThresholds:ClusterAutoscalingSettingsAutoscalingPolicyStorageThresholds`Utilization thresholds pertaining to amount of consumed storage.
Structure is documented below.
:ó
Ó
vmwareengineCClusterAutoscalingSettingsAutoscalingPolicyConsumedMemoryThresholdsògcp:vmwareengine/ClusterAutoscalingSettingsAutoscalingPolicyConsumedMemoryThresholds:ClusterAutoscalingSettingsAutoscalingPolicyConsumedMemoryThresholds£
†M
scaleIn >The utilization triggering the scale-in operation in percent.
O
scaleOut ?The utilization triggering the scale-out operation in percent.
:ˆ
Õ
vmwareengine8ClusterAutoscalingSettingsAutoscalingPolicyCpuThresholdsÇgcp:vmwareengine/ClusterAutoscalingSettingsAutoscalingPolicyCpuThresholds:ClusterAutoscalingSettingsAutoscalingPolicyCpuThresholds£
†M
scaleIn >The utilization triggering the scale-in operation in percent.
O
scaleOut ?The utilization triggering the scale-out operation in percent.
:Ç
Ÿ
vmwareengine<ClusterAutoscalingSettingsAutoscalingPolicyStorageThresholdsägcp:vmwareengine/ClusterAutoscalingSettingsAutoscalingPolicyStorageThresholds:ClusterAutoscalingSettingsAutoscalingPolicyStorageThresholds£
†M
scaleIn >The utilization triggering the scale-in operation in percent.
O
scaleOut ?The utilization triggering the scale-out operation in percent.
:¥
c
vmwareengineClusterNodeTypeConfig<gcp:vmwareengine/ClusterNodeTypeConfig:ClusterNodeTypeConfigÃ
…∂
customCoreCountB úCustomized number of cores available to each node of the type.
This number must always be one of `nodeType.availableCustomCoreCounts`.
If zero is provided max value from `nodeType.availableCustomCoreCounts` will be used.
Once the customer is created then corecount cannot be changed.
B
	nodeCount 1The number of nodes of this type in the cluster.
J

nodeTypeId" 8The identifier for this object. Format specified above.
:®
ê
vmwareengine$ExternalAccessRuleDestinationIpRangeZgcp:vmwareengine/ExternalAccessRuleDestinationIpRange:ExternalAccessRuleDestinationIpRangeí
èK
externalAddressB" 2The name of an `ExternalAddress` resource.

- - -
@
ipAddressRangeB" (An IP address range in the CIDR format.
:Ù
Å
vmwareengineExternalAccessRuleSourceIpRangePgcp:vmwareengine/ExternalAccessRuleSourceIpRange:ExternalAccessRuleSourceIpRangen
l(
	ipAddressB" A single IP address.
@
ipAddressRangeB" (An IP address range in the CIDR format.
:ú
i
vmwareengineNetworkPolicyExternalIp@gcp:vmwareengine/NetworkPolicyExternalIp:NetworkPolicyExternalIpÆ
´B
enabledB
 1True if the service is enabled; false otherwise.
e
stateB" V(Output)
State of the service. New values may be added to this enum when appropriate.
:®
u
vmwareengineNetworkPolicyInternetAccessHgcp:vmwareengine/NetworkPolicyInternetAccess:NetworkPolicyInternetAccessÆ
´B
enabledB
 1True if the service is enabled; false otherwise.
e
stateB" V(Output)
State of the service. New values may be added to this enum when appropriate.
:Ì
W
vmwareengineNetworkVpcNetwork4gcp:vmwareengine/NetworkVpcNetwork:NetworkVpcNetworkë
é¥
networkB" ¢(Output)
The relative resource name of the service VPC network this VMware Engine network is attached to.
For example: projects/123123/global/networks/my-network
U
typeB" GVMware Engine network type.
Possible values are: `LEGACY`, `STANDARD`.
:‘
Q
vmwareenginePrivateCloudHcx0gcp:vmwareengine/PrivateCloudHcx:PrivateCloudHcx˛
˚<
fqdnB" .Fully qualified domain name of the appliance.
:

internalIpB" &Internal IP address of the appliance.
R
stateB" CState of the appliance.
Possible values are: `ACTIVE`, `CREATING`.
+
versionB" Version of the appliance.
:ú
{
vmwareenginePrivateCloudManagementClusterLgcp:vmwareengine/PrivateCloudManagementCluster:PrivateCloudManagementClusterú
ô˜
autoscalingSettingsΩB∫:∑
¥
vmwareengine0PrivateCloudManagementClusterAutoscalingSettingsrgcp:vmwareengine/PrivateCloudManagementClusterAutoscalingSettings:PrivateCloudManagementClusterAutoscalingSettingsüConfiguration of the autoscaling applied to this cluster
Private cloud must have a minimum of 3 nodes to add autoscale settings
Structure is documented below.
Ì
	clusterId" €The user-provided identifier of the new Cluster. The identifier must meet the following requirements:
* Only contains 1-63 alphanumeric characters and hyphens
* Begins with an alphabetical character
* Ends with a non-hyphen character
* Not formatted as a UUID
* Complies with RFC 1034 (https://datatracker.ietf.org/doc/html/rfc1034) (section 3.5)
Ï
nodeTypeConfigs±BÆ*´:®
•
vmwareengine+PrivateCloudManagementClusterNodeTypeConfighgcp:vmwareengine/PrivateCloudManagementClusterNodeTypeConfig:PrivateCloudManagementClusterNodeTypeConfig§The map of cluster node types in this cluster,
where the key is canonical identifier of the node type (corresponds to the NodeType).
Structure is documented below.
Ω
stretchedClusterConfig∆B√:¿
Ω
vmwareengine3PrivateCloudManagementClusterStretchedClusterConfigxgcp:vmwareengine/PrivateCloudManagementClusterStretchedClusterConfig:PrivateCloudManagementClusterStretchedClusterConfigZThe stretched cluster configuration for the private cloud.
Structure is documented below.
:–
¥
vmwareengine0PrivateCloudManagementClusterAutoscalingSettingsrgcp:vmwareengine/PrivateCloudManagementClusterAutoscalingSettings:PrivateCloudManagementClusterAutoscalingSettingsñ
ìñ
autoscalingPoliciesÒ*Ó:Î
Ë
vmwareengineAPrivateCloudManagementClusterAutoscalingSettingsAutoscalingPolicyîgcp:vmwareengine/PrivateCloudManagementClusterAutoscalingSettingsAutoscalingPolicy:PrivateCloudManagementClusterAutoscalingSettingsAutoscalingPolicyäThe map with autoscaling policies applied to the cluster.
The key is the identifier of the policy.
It must meet the following requirements:
* Only contains 1-63 alphanumeric characters and hyphens
* Begins with an alphabetical character
* Ends with a non-hyphen character
* Not formatted as a UUID
* Complies with [RFC 1034](https://datatracker.ietf.org/doc/html/rfc1034) (section 3.5)
Currently the map must contain only one element
that describes the autoscaling policy for compute nodes.
Structure is documented below.
«
coolDownPeriodB" ÆThe minimum duration between consecutive autoscale operations.
It starts once addition or removal of nodes is fully completed.
Minimum cool down period is 30m.
Cool down period must be in whole minutes (for example, 30m, 31m, 50m).
Mandatory for successful addition of autoscaling settings in cluster.
ï
maxClusterNodeCountB xMaximum number of nodes of any type in a cluster.
Mandatory for successful addition of autoscaling settings in cluster.
ï
minClusterNodeCountB xMinimum number of nodes of any type in a cluster.
Mandatory for successful addition of autoscaling settings in cluster.
:˘
Ë
vmwareengineAPrivateCloudManagementClusterAutoscalingSettingsAutoscalingPolicyîgcp:vmwareengine/PrivateCloudManagementClusterAutoscalingSettingsAutoscalingPolicy:PrivateCloudManagementClusterAutoscalingSettingsAutoscalingPolicyã
àQ
autoscalePolicyId" 8The identifier for this object. Format specified above.
∑
consumedMemoryThresholdsπB∂:≥
∞
vmwareengineYPrivateCloudManagementClusterAutoscalingSettingsAutoscalingPolicyConsumedMemoryThresholdsƒgcp:vmwareengine/PrivateCloudManagementClusterAutoscalingSettingsAutoscalingPolicyConsumedMemoryThresholds:PrivateCloudManagementClusterAutoscalingSettingsAutoscalingPolicyConsumedMemoryThresholds_Utilization thresholds pertaining to amount of consumed memory.
Structure is documented below.
Å
cpuThresholdsòBï:í
è
vmwareengineNPrivateCloudManagementClusterAutoscalingSettingsAutoscalingPolicyCpuThresholdsÆgcp:vmwareengine/PrivateCloudManagementClusterAutoscalingSettingsAutoscalingPolicyCpuThresholds:PrivateCloudManagementClusterAutoscalingSettingsAutoscalingPolicyCpuThresholdsUUtilization thresholds pertaining to CPU utilization.
Structure is documented below.
N

nodeTypeId" <The canonical identifier of the node type to add or remove.
Ö
scaleOutSize qNumber of nodes to add to a cluster during a scale-out operation.
Must be divisible by 2 for stretched clusters.
ú
storageThresholds§B°:û
õ
vmwareengineRPrivateCloudManagementClusterAutoscalingSettingsAutoscalingPolicyStorageThresholds∂gcp:vmwareengine/PrivateCloudManagementClusterAutoscalingSettingsAutoscalingPolicyStorageThresholds:PrivateCloudManagementClusterAutoscalingSettingsAutoscalingPolicyStorageThresholds`Utilization thresholds pertaining to amount of consumed storage.
Structure is documented below.
:Ÿ
∞
vmwareengineYPrivateCloudManagementClusterAutoscalingSettingsAutoscalingPolicyConsumedMemoryThresholdsƒgcp:vmwareengine/PrivateCloudManagementClusterAutoscalingSettingsAutoscalingPolicyConsumedMemoryThresholds:PrivateCloudManagementClusterAutoscalingSettingsAutoscalingPolicyConsumedMemoryThresholds£
†M
scaleIn >The utilization triggering the scale-in operation in percent.
O
scaleOut ?The utilization triggering the scale-out operation in percent.
:∏
è
vmwareengineNPrivateCloudManagementClusterAutoscalingSettingsAutoscalingPolicyCpuThresholdsÆgcp:vmwareengine/PrivateCloudManagementClusterAutoscalingSettingsAutoscalingPolicyCpuThresholds:PrivateCloudManagementClusterAutoscalingSettingsAutoscalingPolicyCpuThresholds£
†M
scaleIn >The utilization triggering the scale-in operation in percent.
O
scaleOut ?The utilization triggering the scale-out operation in percent.
:À
õ
vmwareengineRPrivateCloudManagementClusterAutoscalingSettingsAutoscalingPolicyStorageThresholds∂gcp:vmwareengine/PrivateCloudManagementClusterAutoscalingSettingsAutoscalingPolicyStorageThresholds:PrivateCloudManagementClusterAutoscalingSettingsAutoscalingPolicyStorageThresholds™
ßT
scaleIn EThe utilization triggering the scale-in operation in percent.

- - -
O
scaleOut ?The utilization triggering the scale-out operation in percent.
:Ò
•
vmwareengine+PrivateCloudManagementClusterNodeTypeConfighgcp:vmwareengine/PrivateCloudManagementClusterNodeTypeConfig:PrivateCloudManagementClusterNodeTypeConfig∆
√∞
customCoreCountB ñCustomized number of cores available to each node of the type.
This number must always be one of `nodeType.availableCustomCoreCounts`.
If zero is provided max value from `nodeType.availableCustomCoreCounts` will be used.
This cannot be changed once the PrivateCloud is created.
B
	nodeCount 1The number of nodes of this type in the cluster.
J

nodeTypeId" 8The identifier for this object. Format specified above.
:ò
Ω
vmwareengine3PrivateCloudManagementClusterStretchedClusterConfigxgcp:vmwareengine/PrivateCloudManagementClusterStretchedClusterConfig:PrivateCloudManagementClusterStretchedClusterConfig’
“l
preferredLocationB" QZone that will remain operational when connection between the two zones is lost.
b
secondaryLocationB" GAdditional zone for a higher level of availability and load balancing.
:å

o
vmwareenginePrivateCloudNetworkConfigDgcp:vmwareengine/PrivateCloudNetworkConfig:PrivateCloudNetworkConfigò	
ï	B
dnsServerIpB" -(Output)
DNS Server IP of the Private Cloud.
L
managementCidr" 6Management CIDR used by VMware management appliances.
ˇ
 managementIpAddressLayoutVersionB ‘(Output)
The IP address layout version of the management IP address range.
Possible versions include:
* managementIpAddressLayoutVersion=1: Indicates the legacy IP address layout used by some existing private clouds. This is no longer supported for new private clouds
as it does not support all features.
* managementIpAddressLayoutVersion=2: Indicates the latest IP address layout
used by all newly created private clouds. This version supports all current features.
≤
vmwareEngineNetworkB" îThe relative resource name of the VMware Engine network attached to the private cloud.
Specify the name in the following form: projects/{project}/locations/{location}/vmwareEngineNetworks/{vmwareEngineNetworkId}
where {project} can either be a project number or a project ID.
…
vmwareEngineNetworkCanonicalB" ¢(Output)
The canonical name of the VMware Engine network in
the form: projects/{project_number}/locations/{location}/vmwareEngineNetworks/{vmwareEngineNetworkId}
:‘
Q
vmwareenginePrivateCloudNsx0gcp:vmwareengine/PrivateCloudNsx:PrivateCloudNsx˛
˚<
fqdnB" .Fully qualified domain name of the appliance.
:

internalIpB" &Internal IP address of the appliance.
R
stateB" CState of the appliance.
Possible values are: `ACTIVE`, `CREATING`.
+
versionB" Version of the appliance.
:‡
]
vmwareenginePrivateCloudVcenter8gcp:vmwareengine/PrivateCloudVcenter:PrivateCloudVcenter˛
˚<
fqdnB" .Fully qualified domain name of the appliance.
:

internalIpB" &Internal IP address of the appliance.
R
stateB" CState of the appliance.
Possible values are: `ACTIVE`, `CREATING`.
+
versionB" Version of the appliance.
:Ù
f
vmwareengineSubnetDhcpAddressRange>gcp:vmwareengine/SubnetDhcpAddressRange:SubnetDhcpAddressRangeâ
ÜB
firstAddressB" ,(Output)
The first IP address of the range.
@
lastAddressB" +(Output)
The last IP address of the range.
:º
x
vmwareenginegetClusterAutoscalingSettingJgcp:vmwareengine/getClusterAutoscalingSetting:getClusterAutoscalingSettingø

º
≈
autoscalingPolicies¥*±:Æ
´
vmwareengine-getClusterAutoscalingSettingAutoscalingPolicylgcp:vmwareengine/getClusterAutoscalingSettingAutoscalingPolicy:getClusterAutoscalingSettingAutoscalingPolicyˆThe map with autoscaling policies applied to the cluster.
The key is the identifier of the policy.
It must meet the following requirements:
  * Only contains 1-63 alphanumeric characters and hyphens
  * Begins with an alphabetical character
  * Ends with a non-hyphen character
  * Not formatted as a UUID
  * Complies with [RFC 1034](https://datatracker.ietf.org/doc/html/rfc1034) (section 3.5)

Currently the map must contain only one element
that describes the autoscaling policy for compute nodes.
≈
coolDownPeriod" ÆThe minimum duration between consecutive autoscale operations.
It starts once addition or removal of nodes is fully completed.
Minimum cool down period is 30m.
Cool down period must be in whole minutes (for example, 30m, 31m, 50m).
Mandatory for successful addition of autoscaling settings in cluster.
ì
maxClusterNodeCount xMaximum number of nodes of any type in a cluster.
Mandatory for successful addition of autoscaling settings in cluster.
ì
minClusterNodeCount xMinimum number of nodes of any type in a cluster.
Mandatory for successful addition of autoscaling settings in cluster.
:Ë

´
vmwareengine-getClusterAutoscalingSettingAutoscalingPolicylgcp:vmwareengine/getClusterAutoscalingSettingAutoscalingPolicy:getClusterAutoscalingSettingAutoscalingPolicy∑	
¥	
autoscalePolicyId" Ÿ
consumedMemoryThresholds˙*˜:Ù
Ò
vmwareengineDgetClusterAutoscalingSettingAutoscalingPolicyConsumedMemoryThresholdögcp:vmwareengine/getClusterAutoscalingSettingAutoscalingPolicyConsumedMemoryThreshold:getClusterAutoscalingSettingAutoscalingPolicyConsumedMemoryThreshold@Utilization thresholds pertaining to amount of consumed memory.
£
cpuThresholdsŸ*÷:”
–
vmwareengine9getClusterAutoscalingSettingAutoscalingPolicyCpuThresholdÑgcp:vmwareengine/getClusterAutoscalingSettingAutoscalingPolicyCpuThreshold:getClusterAutoscalingSettingAutoscalingPolicyCpuThreshold6Utilization thresholds pertaining to CPU utilization.
N

nodeTypeId" <The canonical identifier of the node type to add or remove.
Ö
scaleOutSize qNumber of nodes to add to a cluster during a scale-out operation.
Must be divisible by 2 for stretched clusters.
æ
storageThresholdsÂ*‚:ﬂ
‹
vmwareengine=getClusterAutoscalingSettingAutoscalingPolicyStorageThresholdågcp:vmwareengine/getClusterAutoscalingSettingAutoscalingPolicyStorageThreshold:getClusterAutoscalingSettingAutoscalingPolicyStorageThresholdAUtilization thresholds pertaining to amount of consumed storage.
:ö
Ò
vmwareengineDgetClusterAutoscalingSettingAutoscalingPolicyConsumedMemoryThresholdögcp:vmwareengine/getClusterAutoscalingSettingAutoscalingPolicyConsumedMemoryThreshold:getClusterAutoscalingSettingAutoscalingPolicyConsumedMemoryThreshold£
†M
scaleIn >The utilization triggering the scale-in operation in percent.
O
scaleOut ?The utilization triggering the scale-out operation in percent.
:˘
–
vmwareengine9getClusterAutoscalingSettingAutoscalingPolicyCpuThresholdÑgcp:vmwareengine/getClusterAutoscalingSettingAutoscalingPolicyCpuThreshold:getClusterAutoscalingSettingAutoscalingPolicyCpuThreshold£
†M
scaleIn >The utilization triggering the scale-in operation in percent.
O
scaleOut ?The utilization triggering the scale-out operation in percent.
:Ö
‹
vmwareengine=getClusterAutoscalingSettingAutoscalingPolicyStorageThresholdågcp:vmwareengine/getClusterAutoscalingSettingAutoscalingPolicyStorageThreshold:getClusterAutoscalingSettingAutoscalingPolicyStorageThreshold£
†M
scaleIn >The utilization triggering the scale-in operation in percent.
O
scaleOut ?The utilization triggering the scale-out operation in percent.
:Å
l
vmwareenginegetClusterNodeTypeConfigBgcp:vmwareengine/getClusterNodeTypeConfig:getClusterNodeTypeConfigê
ç¥
customCoreCount úCustomized number of cores available to each node of the type.
This number must always be one of 'nodeType.availableCustomCoreCounts'.
If zero is provided max value from 'nodeType.availableCustomCoreCounts' will be used.
Once the customer is created then corecount cannot be changed.
B
	nodeCount 1The number of nodes of this type in the cluster.


nodeTypeId" :¶
ô
vmwareengine'getExternalAccessRuleDestinationIpRange`gcp:vmwareengine/getExternalAccessRuleDestinationIpRange:getExternalAccessRuleDestinationIpRangeá
ÑB
externalAddress" +The name of an 'ExternalAddress' resource.
>
ipAddressRange" (An IP address range in the CIDR format.
:˘
ä
vmwareengine"getExternalAccessRuleSourceIpRangeVgcp:vmwareengine/getExternalAccessRuleSourceIpRange:getExternalAccessRuleSourceIpRangej
h&
	ipAddress" A single IP address.
>
ipAddressRange" (An IP address range in the CIDR format.
:ò
r
vmwareenginegetNetworkPolicyExternalIpFgcp:vmwareengine/getNetworkPolicyExternalIp:getNetworkPolicyExternalIp°
û@
enabled
 1True if the service is enabled; false otherwise.
Z
state" MState of the service. New values may be added to this enum when appropriate.
:§
~
vmwareenginegetNetworkPolicyInternetAccessNgcp:vmwareengine/getNetworkPolicyInternetAccess:getNetworkPolicyInternetAccess°
û@
enabled
 1True if the service is enabled; false otherwise.
Z
state" MState of the service. New values may be added to this enum when appropriate.
:‹
`
vmwareenginegetNetworkVpcNetwork:gcp:vmwareengine/getNetworkVpcNetwork:getNetworkVpcNetwork˜
Ù©
network" ôThe relative resource name of the service VPC network this VMware Engine network is attached to.
For example: projects/123123/global/networks/my-network
F
type" :Type of VPC network (INTRANET, INTERNET, or GOOGLE_CLOUD)
:“
Z
vmwareenginegetPrivateCloudHcx6gcp:vmwareengine/getPrivateCloudHcx:getPrivateCloudHcxÛ
:
fqdn" .Fully qualified domain name of the appliance.
8

internalIp" &Internal IP address of the appliance.
M
state" @State of the appliance. Possible values: ["ACTIVE", "CREATING"]
)
version" Version of the appliance.
:È
Ñ
vmwareengine getPrivateCloudManagementClusterRgcp:vmwareengine/getPrivateCloudManagementCluster:getPrivateCloudManagementClusterﬂ

‹
ﬁ
autoscalingSettings√*¿:Ω
∫
vmwareengine2getPrivateCloudManagementClusterAutoscalingSettingvgcp:vmwareengine/getPrivateCloudManagementClusterAutoscalingSetting:getPrivateCloudManagementClusterAutoscalingSettingÄConfiguration of the autoscaling applied to this cluster
Private cloud must have a minimum of 3 nodes to add autoscale settings
˜
	clusterId" ÂThe user-provided identifier of the new Cluster. The identifier must meet the following requirements:
  * Only contains 1-63 alphanumeric characters and hyphens
  * Begins with an alphabetical character
  * Ends with a non-hyphen character
  * Not formatted as a UUID
  * Complies with RFC 1034 (https://datatracker.ietf.org/doc/html/rfc1034) (section 3.5)
”
nodeTypeConfigs∑*¥:±
Æ
vmwareengine.getPrivateCloudManagementClusterNodeTypeConfigngcp:vmwareengine/getPrivateCloudManagementClusterNodeTypeConfig:getPrivateCloudManagementClusterNodeTypeConfigÖThe map of cluster node types in this cluster,
where the key is canonical identifier of the node type (corresponds to the NodeType).
®
stretchedClusterConfigsœ*Ã:…
∆
vmwareengine6getPrivateCloudManagementClusterStretchedClusterConfig~gcp:vmwareengine/getPrivateCloudManagementClusterStretchedClusterConfig:getPrivateCloudManagementClusterStretchedClusterConfig;The stretched cluster configuration for the private cloud.
:Ω
∫
vmwareengine2getPrivateCloudManagementClusterAutoscalingSettingvgcp:vmwareengine/getPrivateCloudManagementClusterAutoscalingSetting:getPrivateCloudManagementClusterAutoscalingSetting˝

˙
É
autoscalingPolicies˜*Ù:Ò
Ó
vmwareengineCgetPrivateCloudManagementClusterAutoscalingSettingAutoscalingPolicyògcp:vmwareengine/getPrivateCloudManagementClusterAutoscalingSettingAutoscalingPolicy:getPrivateCloudManagementClusterAutoscalingSettingAutoscalingPolicyÒThe map with autoscaling policies applied to the cluster.
The key is the identifier of the policy.
It must meet the following requirements:
 * Only contains 1-63 alphanumeric characters and hyphens
 * Begins with an alphabetical character
 * Ends with a non-hyphen character
 * Not formatted as a UUID
 * Complies with [RFC 1034](https://datatracker.ietf.org/doc/html/rfc1034) (section 3.5)

Currently the map must contain only one element
that describes the autoscaling policy for compute nodes.
≈
coolDownPeriod" ÆThe minimum duration between consecutive autoscale operations.
It starts once addition or removal of nodes is fully completed.
Minimum cool down period is 30m.
Cool down period must be in whole minutes (for example, 30m, 31m, 50m).
Mandatory for successful addition of autoscaling settings in cluster.
ì
maxClusterNodeCount xMaximum number of nodes of any type in a cluster.
Mandatory for successful addition of autoscaling settings in cluster.
ì
minClusterNodeCount xMinimum number of nodes of any type in a cluster.
Mandatory for successful addition of autoscaling settings in cluster.
:Ò
Ó
vmwareengineCgetPrivateCloudManagementClusterAutoscalingSettingAutoscalingPolicyògcp:vmwareengine/getPrivateCloudManagementClusterAutoscalingSettingAutoscalingPolicy:getPrivateCloudManagementClusterAutoscalingSettingAutoscalingPolicy˝

˙

autoscalePolicyId" õ
consumedMemoryThresholdsº*π:∂
≥
vmwareengineZgetPrivateCloudManagementClusterAutoscalingSettingAutoscalingPolicyConsumedMemoryThreshold∆gcp:vmwareengine/getPrivateCloudManagementClusterAutoscalingSettingAutoscalingPolicyConsumedMemoryThreshold:getPrivateCloudManagementClusterAutoscalingSettingAutoscalingPolicyConsumedMemoryThreshold@Utilization thresholds pertaining to amount of consumed memory.
Â
cpuThresholdsõ*ò:ï
í
vmwareengineOgetPrivateCloudManagementClusterAutoscalingSettingAutoscalingPolicyCpuThreshold∞gcp:vmwareengine/getPrivateCloudManagementClusterAutoscalingSettingAutoscalingPolicyCpuThreshold:getPrivateCloudManagementClusterAutoscalingSettingAutoscalingPolicyCpuThreshold6Utilization thresholds pertaining to CPU utilization.
N

nodeTypeId" <The canonical identifier of the node type to add or remove.
Ö
scaleOutSize qNumber of nodes to add to a cluster during a scale-out operation.
Must be divisible by 2 for stretched clusters.
Ä
storageThresholdsß*§:°
û
vmwareengineSgetPrivateCloudManagementClusterAutoscalingSettingAutoscalingPolicyStorageThreshold∏gcp:vmwareengine/getPrivateCloudManagementClusterAutoscalingSettingAutoscalingPolicyStorageThreshold:getPrivateCloudManagementClusterAutoscalingSettingAutoscalingPolicyStorageThresholdAUtilization thresholds pertaining to amount of consumed storage.
:‹
≥
vmwareengineZgetPrivateCloudManagementClusterAutoscalingSettingAutoscalingPolicyConsumedMemoryThreshold∆gcp:vmwareengine/getPrivateCloudManagementClusterAutoscalingSettingAutoscalingPolicyConsumedMemoryThreshold:getPrivateCloudManagementClusterAutoscalingSettingAutoscalingPolicyConsumedMemoryThreshold£
†M
scaleIn >The utilization triggering the scale-in operation in percent.
O
scaleOut ?The utilization triggering the scale-out operation in percent.
:ª
í
vmwareengineOgetPrivateCloudManagementClusterAutoscalingSettingAutoscalingPolicyCpuThreshold∞gcp:vmwareengine/getPrivateCloudManagementClusterAutoscalingSettingAutoscalingPolicyCpuThreshold:getPrivateCloudManagementClusterAutoscalingSettingAutoscalingPolicyCpuThreshold£
†M
scaleIn >The utilization triggering the scale-in operation in percent.
O
scaleOut ?The utilization triggering the scale-out operation in percent.
:«
û
vmwareengineSgetPrivateCloudManagementClusterAutoscalingSettingAutoscalingPolicyStorageThreshold∏gcp:vmwareengine/getPrivateCloudManagementClusterAutoscalingSettingAutoscalingPolicyStorageThreshold:getPrivateCloudManagementClusterAutoscalingSettingAutoscalingPolicyStorageThreshold£
†M
scaleIn >The utilization triggering the scale-in operation in percent.
O
scaleOut ?The utilization triggering the scale-out operation in percent.
:æ
Æ
vmwareengine.getPrivateCloudManagementClusterNodeTypeConfigngcp:vmwareengine/getPrivateCloudManagementClusterNodeTypeConfig:getPrivateCloudManagementClusterNodeTypeConfigä
áÆ
customCoreCount ñCustomized number of cores available to each node of the type.
This number must always be one of 'nodeType.availableCustomCoreCounts'.
If zero is provided max value from 'nodeType.availableCustomCoreCounts' will be used.
This cannot be changed once the PrivateCloud is created.
B
	nodeCount 1The number of nodes of this type in the cluster.


nodeTypeId" :ù
∆
vmwareengine6getPrivateCloudManagementClusterStretchedClusterConfig~gcp:vmwareengine/getPrivateCloudManagementClusterStretchedClusterConfig:getPrivateCloudManagementClusterStretchedClusterConfig—
Œj
preferredLocation" QZone that will remain operational when connection between the two zones is lost.
`
secondaryLocation" GAdditional zone for a higher level of availability and load balancing.
:Ú	
x
vmwareenginegetPrivateCloudNetworkConfigJgcp:vmwareengine/getPrivateCloudNetworkConfig:getPrivateCloudNetworkConfigı
Ú7
dnsServerIp" $DNS Server IP of the Private Cloud.
L
managementCidr" 6Management CIDR used by VMware management appliances.
Ù
 managementIpAddressLayoutVersion ÀThe IP address layout version of the management IP address range.
Possible versions include:
* managementIpAddressLayoutVersion=1: Indicates the legacy IP address layout used by some existing private clouds. This is no longer supported for new private clouds
as it does not support all features.
* managementIpAddressLayoutVersion=2: Indicates the latest IP address layout
used by all newly created private clouds. This version supports all current features.
∞
vmwareEngineNetwork" îThe relative resource name of the VMware Engine network attached to the private cloud.
Specify the name in the following form: projects/{project}/locations/{location}/vmwareEngineNetworks/{vmwareEngineNetworkId}
where {project} can either be a project number or a project ID.
æ
vmwareEngineNetworkCanonical" ôThe canonical name of the VMware Engine network in
the form: projects/{project_number}/locations/{location}/vmwareEngineNetworks/{vmwareEngineNetworkId}
:“
Z
vmwareenginegetPrivateCloudNsx6gcp:vmwareengine/getPrivateCloudNsx:getPrivateCloudNsxÛ
:
fqdn" .Fully qualified domain name of the appliance.
8

internalIp" &Internal IP address of the appliance.
M
state" @State of the appliance. Possible values: ["ACTIVE", "CREATING"]
)
version" Version of the appliance.
:ﬁ
f
vmwareenginegetPrivateCloudVcenter>gcp:vmwareengine/getPrivateCloudVcenter:getPrivateCloudVcenterÛ
:
fqdn" .Fully qualified domain name of the appliance.
8

internalIp" &Internal IP address of the appliance.
M
state" @State of the appliance. Possible values: ["ACTIVE", "CREATING"]
)
version" Version of the appliance.
:Â
o
vmwareenginegetSubnetDhcpAddressRangeDgcp:vmwareengine/getSubnetDhcpAddressRange:getSubnetDhcpAddressRanger
p7
firstAddress" #The first IP address of the range.
5
lastAddress" "The last IP address of the range.
:˘
K
	vpcaccessConnectorSubnet-gcp:vpcaccess/ConnectorSubnet:ConnectorSubnet©
¶ˇ
nameB" Subnet name (relative, not fully qualified). E.g. if the full subnet selfLink is
https://compute.googleapis.com/compute/v1/projects/{project}/regions/{region}/subnetworks/{subnetName} the correct input for this field would be {subnetName}"
°
	projectIdB" çProject in which the subnet exists. If not set, this project is assumed to be the project for which the connector create request was issued.
:©
T
	vpcaccessgetConnectorSubnet3gcp:vpcaccess/getConnectorSubnet:getConnectorSubnet–
Õ)
name" Name of the resource.

- - -
ü
	projectId" çProject in which the subnet exists. If not set, this project is assumed to be the project for which the connector create request was issued.
:∫
N
	workbenchInstanceGceSetup/gcp:workbench/InstanceGceSetup:InstanceGceSetupÁ
‰Ô
acceleratorConfigsçBä*á:Ñ
Å
	workbench!InstanceGceSetupAcceleratorConfigQgcp:workbench/InstanceGceSetupAcceleratorConfig:InstanceGceSetupAcceleratorConfig»The hardware accelerators used on this instance. If you use accelerators, make sure that your configuration has
[enough vCPUs and memory to support the `machine_type` you have selected](https://cloud.google.com/compute/docs/gpus/#gpus-list).
Currently supports only one accelerator configuration.
Structure is documented below.
∏
bootDisklBj:h
f
	workbenchInstanceGceSetupBootDisk?gcp:workbench/InstanceGceSetupBootDisk:InstanceGceSetupBootDisk>The definition of a boot disk.
Structure is documented below.
Ë
containerImage~B|:z
x
	workbenchInstanceGceSetupContainerImageKgcp:workbench/InstanceGceSetupContainerImage:InstanceGceSetupContainerImageVUse a container image to start the workbench instance.
Structure is documented below.
Ï
	dataDisksoBm:k
i
	workbenchInstanceGceSetupDataDisksAgcp:workbench/InstanceGceSetupDataDisks:InstanceGceSetupDataDisksnData disks attached to the VM instance. Currently supports only one data disk.
Structure is documented below.
a
disablePublicIpB
 HOptional. If true, no external IP will be assigned to this VM instance.
ö
enableIpForwardingB
 ~Optional. Flag to enable ip forwarding or not, default false/off.
https://cloud.google.com/vpc/docs/using-routes#canipforward
{
machineTypeB" fOptional. The machine type of the VM instance. https://cloud.google.com/compute/docs/machine-resource
I
metadataB2" 5Optional. Custom metadata to apply to this instance.
Ä
networkInterfacesâBÜ*É:Ä
~
	workbench InstanceGceSetupNetworkInterfaceOgcp:workbench/InstanceGceSetupNetworkInterface:InstanceGceSetupNetworkInterface_The network interfaces for the VM. Supports only one interface.
Structure is documented below.
ß
serviceAccountsÄB~*|:z
x
	workbenchInstanceGceSetupServiceAccountKgcp:workbench/InstanceGceSetupServiceAccount:InstanceGceSetupServiceAccountêThe service account that serves as an identity for the VM instance. Currently supports only one service account.
Structure is documented below.
î
shieldedInstanceConfigôBñ:ì
ê
	workbench&InstanceGceSetupShieldedInstanceConfig[gcp:workbench/InstanceGceSetupShieldedInstanceConfig:InstanceGceSetupShieldedInstanceConfig›A set of Shielded Instance options. See [Images using supported Shielded
VM features](https://cloud.google.com/compute/docs/instances/modifying-shielded-vm).
Not all combinations are valid.
Structure is documented below.
§
tagsB*" ìOptional. The Compute Engine tags to add to instance (see [Tagging
instances](https://cloud.google.com/compute/docs/label-or-tag-resources#tags)).
ß
vmImageiBg:e
c
	workbenchInstanceGceSetupVmImage=gcp:workbench/InstanceGceSetupVmImage:InstanceGceSetupVmImage∞Definition of a custom Compute Engine virtual machine image for starting
a workbench instance with the environment installed directly on the VM.
Structure is documented below.
:Â
Å
	workbench!InstanceGceSetupAcceleratorConfigQgcp:workbench/InstanceGceSetupAcceleratorConfig:InstanceGceSetupAcceleratorConfigﬁ
€A
	coreCountB" .Optional. Count of cores of this accelerator.
ï
typeB" ÜOptional. Type of this accelerator.
Possible values are: `NVIDIA_TESLA_P100`, `NVIDIA_TESLA_V100`, `NVIDIA_TESLA_P4`, `NVIDIA_TESLA_T4`, `NVIDIA_TESLA_A100`, `NVIDIA_A100_80GB`, `NVIDIA_L4`, `NVIDIA_TESLA_T4_VWS`, `NVIDIA_TESLA_P100_VWS`, `NVIDIA_TESLA_P4_VWS`.
:‚
f
	workbenchInstanceGceSetupBootDisk?gcp:workbench/InstanceGceSetupBootDisk:InstanceGceSetupBootDisk˜
Ùû
diskEncryptionB" ÖOptional. Input only. Disk encryption method used on the boot and
data disks, defaults to GMEK.
Possible values are: `GMEK`, `CMEK`.
¬

diskSizeGbB" ≠Optional. The size of the boot disk in GB attached to this instance,
up to a maximum of 64000 GB (64 TB). If not specified, this defaults to the
recommended value of 150GB.
á
diskTypeB" uOptional. Indicates the type of the disk.
Possible values are: `PD_STANDARD`, `PD_SSD`, `PD_BALANCED`, `PD_EXTREME`.
Å
kmsKeyB" 'Optional. The KMS key used to encrypt the disks, only
applicable if disk_encryption is CMEK. Format: `projects/{project_id}/locations/{location}/keyRings/{key_ring_id}/cryptoKeys/{key_id}`
Learn more about using your own encryption keys.'
:œ
x
	workbenchInstanceGceSetupContainerImageKgcp:workbench/InstanceGceSetupContainerImage:InstanceGceSetupContainerImage“
œk

repository" YThe path to the container image repository.
For example: gcr.io/{project_id}/{imageName}
`
tagB" SThe tag of the container image. If not specified, this defaults to the latest tag.
:’
i
	workbenchInstanceGceSetupDataDisksAgcp:workbench/InstanceGceSetupDataDisks:InstanceGceSetupDataDisksÁ
‰û
diskEncryptionB" ÖOptional. Input only. Disk encryption method used on the boot
and data disks, defaults to GMEK.
Possible values are: `GMEK`, `CMEK`.
•

diskSizeGbB" êOptional. The size of the disk in GB attached to this VM instance,
up to a maximum of 64000 GB (64 TB). If not specified, this defaults to
100.
î
diskTypeB" ÅOptional. Input only. Indicates the type of the disk.
Possible values are: `PD_STANDARD`, `PD_SSD`, `PD_BALANCED`, `PD_EXTREME`.
Å
kmsKeyB" 'Optional. The KMS key used to encrypt the disks,
only applicable if disk_encryption is CMEK. Format: `projects/{project_id}/locations/{location}/keyRings/{key_ring_id}/cryptoKeys/{key_id}`
Learn more about using your own encryption keys.'
:ô
~
	workbench InstanceGceSetupNetworkInterfaceOgcp:workbench/InstanceGceSetupNetworkInterface:InstanceGceSetupNetworkInterfaceñ
ìÿ
accessConfigsÆB´*®:•
¢
	workbench,InstanceGceSetupNetworkInterfaceAccessConfigggcp:workbench/InstanceGceSetupNetworkInterfaceAccessConfig:InstanceGceSetupNetworkInterfaceAccessConfigïOptional. An array of configurations for this interface. Currently, only one access
config, ONE_TO_ONE_NAT, is supported. If no accessConfigs specified, the
instance will have an external internet access through an ephemeral
external IP address.
Structure is documented below.
L
networkB" ;Optional. The name of the VPC that this VM instance is in.
ó
nicTypeB" ÖOptional. The type of vNIC to be used on this interface. This
may be gVNIC or VirtioNet.
Possible values are: `VIRTIO_NET`, `GVNIC`.
N
subnetB" >Optional. The name of the subnet that this VM instance is in.
:˚
¢
	workbench,InstanceGceSetupNetworkInterfaceAccessConfigggcp:workbench/InstanceGceSetupNetworkInterfaceAccessConfig:InstanceGceSetupNetworkInterfaceAccessConfig”
–Õ

externalIp" ∫An external IP address associated with this instance. Specify an unused
static external IP address available to the project or leave this field
undefined to use an IP from a shared ephemeral IP address pool. If you
specify a static external IP address, it must live in the same region as
the zone of the instance.
:
x
	workbenchInstanceGceSetupServiceAccountKgcp:workbench/InstanceGceSetupServiceAccount:InstanceGceSetupServiceAccountÛ
?
emailB" 0Optional. Email address of the service account.
¨
scopesB*" ô(Output)
Output only. The list of scopes to be made available for this
service account. Set by the CLH to https://www.googleapis.com/auth/cloud-platform
:ª
ê
	workbench&InstanceGceSetupShieldedInstanceConfig[gcp:workbench/InstanceGceSetupShieldedInstanceConfig:InstanceGceSetupShieldedInstanceConfig•
¢Ä
enableIntegrityMonitoringB
 ‹Optional. Defines whether the VM instance has integrity monitoring
enabled. Enables monitoring and attestation of the boot integrity of the VM
instance. The attestation is performed against the integrity policy baseline.
This baseline is initially derived from the implicitly trusted boot image
when the VM instance is created. Enabled by default.
≤
enableSecureBootB
 óOptional. Defines whether the VM instance has Secure Boot enabled.
Secure Boot helps ensure that the system only runs authentic software by verifying
the digital signature of all boot components, and halting the boot process
if signature verification fails. Disabled by default.
h

enableVtpmB
 TOptional. Defines whether the VM instance has the vTPM enabled.
Enabled by default.
:å
c
	workbenchInstanceGceSetupVmImage=gcp:workbench/InstanceGceSetupVmImage:InstanceGceSetupVmImage§
°t
familyB" dOptional. Use this VM image family to find the image; the newest
image in this family will be used.
=
nameB" /Optional. Use VM image name to find the image.
j
projectB" YThe name of the Google Cloud project that this VM image belongs to.
Format: {project_id}
:Z
T
	workbenchInstanceHealthInfo3gcp:workbench/InstanceHealthInfo:InstanceHealthInfo
 :©
o
	workbenchInstanceIamBindingConditionEgcp:workbench/InstanceIamBindingCondition:InstanceIamBindingCondition6
4
descriptionB" 

expression" 
title" :¶
l
	workbenchInstanceIamMemberConditionCgcp:workbench/InstanceIamMemberCondition:InstanceIamMemberCondition6
4
descriptionB" 

expression" 
title" :˙
`
	workbenchInstanceUpgradeHistory;gcp:workbench/InstanceUpgradeHistory:InstanceUpgradeHistoryï
í8
actionB" (Optional. Action. Rolloback or Upgrade.
T
containerImageB" <Optional. The container image before this instance upgrade.
ñ

createTimeB" ÅAn RFC3339 timestamp in UTC time. This in the format of yyyy-MM-ddTHH:mm:ss.SSSZ.
The milliseconds portion (".SSS") is optional.
G
	frameworkB" 4Optional. The framework of this workbench instance.
e
snapshotB" SOptional. The snapshot of the boot disk of this workbench instance before upgrade.
W
stateB" H(Output)
Output only. The state of this instance upgrade history entry.
>
targetVersionB" 'Optional. Target VM Version, like m63.
V
versionB" EOptional. The version of the workbench instance before this upgrade.
F
vmImageB" 5Optional. The VM image before this instance upgrade.
:ä
u
workstationsWorkstationClusterConditionHgcp:workstations/WorkstationClusterCondition:WorkstationClusterConditionê
çZ
codeB L(Output)
The status code, which should be an enum value of google.rpc.Code.
O
detailsB*2" :(Output)
A list of messages that carry the error details.
^
messageB" M(Output)
Human readable message indicating details about the current status.
:¬
~
workstationsWorkstationClusterDomainConfigNgcp:workstations/WorkstationClusterDomainConfig:WorkstationClusterDomainConfig@
><
domain" .Domain used by Workstations for HTTP ingress.
:Ù
ñ
workstations&WorkstationClusterPrivateClusterConfig^gcp:workstations/WorkstationClusterPrivateClusterConfig:WorkstationClusterPrivateClusterConfigÿ
’‚
allowedProjectsB*" ∆Additional project IDs that are allowed to attach to the workstation cluster's service attachment.
By default, the workstation cluster's project and the VPC host project (if different) are allowed.
√
clusterHostnameB" ©(Output)
Hostname for the workstation cluster.
This field will be populated only when private endpoint is enabled.
To access workstations in the cluster, create a new DNS zone mapping this domain name to an internal IP address and a forwarding rule mapping that address to the service attachment.
G
enablePrivateEndpoint
 *Whether Workstations endpoint is private.
ﬁ
serviceAttachmentUriB" ø(Output)
Service attachment URI for the workstation cluster.
The service attachment is created when private endpoint is enabled.
To access workstations in the cluster, configure access to the managed service using (Private Service Connect)[https://cloud.google.com/vpc/docs/configure-private-service-connect-services].
:â
x
workstationsWorkstationConfigAllowedPortJgcp:workstations/WorkstationConfigAllowedPort:WorkstationConfigAllowedPortå
âÉ
firstB tStarting port number for the current range of ports. Valid ports are 22, 80, and ports within the range 1024-65535.
Ä
lastB rEnding port number for the current range of ports. Valid ports are 22, 80, and ports within the range 1024-65535.
:á
r
workstationsWorkstationConfigConditionFgcp:workstations/WorkstationConfigCondition:WorkstationConfigConditionê
çZ
codeB L(Output)
The status code, which should be an enum value of google.rpc.Code.
O
detailsB*2" :(Output)
A list of messages that carry the error details.
^
messageB" M(Output)
Human readable message indicating details about the current status.
:‘
r
workstationsWorkstationConfigContainerFgcp:workstations/WorkstationConfigContainer:WorkstationConfigContainer›
⁄4
argsB*" $Arguments passed to the entrypoint.
U
commandsB*" AIf set, overrides the default ENTRYPOINT specified by the image.
™
envB2" öEnvironment variables passed to the container.
The elements are of the form "KEY=VALUE" for the environment variable "KEY" being given the value "VALUE".
s
imageB" dDocker image defining the container. This image must be accessible by the config's service account.
Y
	runAsUserB FIf set, overrides the USER specified in the image with the given uid.
N

workingDirB" :If set, overrides the default DIR specified by the image.
:ú
~
workstationsWorkstationConfigEncryptionKeyNgcp:workstations/WorkstationConfigEncryptionKey:WorkstationConfigEncryptionKeyô
ñ?
kmsKey" 1The name of the Google Cloud KMS encryption key.
S
kmsKeyServiceAccount" 7The service account to use with the specified KMS key.
:˜
ç
workstations#WorkstationConfigEphemeralDirectoryXgcp:workstations/WorkstationConfigEphemeralDirectory:WorkstationConfigEphemeralDirectory‰
·í
gcePd•B¢:ü
ú
workstations(WorkstationConfigEphemeralDirectoryGcePdbgcp:workstations/WorkstationConfigEphemeralDirectoryGcePd:WorkstationConfigEphemeralDirectoryGcePdaAn EphemeralDirectory backed by a Compute Engine persistent disk.
Structure is documented below.
J
	mountPathB" 7Location of this directory in the running workstation.
:È
ú
workstations(WorkstationConfigEphemeralDirectoryGcePdbgcp:workstations/WorkstationConfigEphemeralDirectoryGcePd:WorkstationConfigEphemeralDirectoryGcePd«
ƒH
diskTypeB" 6Type of the disk to use. Defaults to `"pd-standard"`.
É
readOnlyB
 qWhether the disk is read only. If true, the disk may be shared by multiple VMs and `sourceSnapshot` must be set.
·
sourceImageB" ÀName of the disk image to use as the source for the disk.
Must be empty `sourceSnapshot` is set.
Updating `sourceImage` will update content in the ephemeral directory after the workstation is restarted.
ç
sourceSnapshotB" ÙName of the snapshot to use as the source for the disk.
Must be empty if `sourceImage` is set.
Must be empty if `read_only` is false.
Updating `source_snapshot` will update content in the ephemeral directory after the workstation is restarted.
:◊
c
workstationsWorkstationConfigHost<gcp:workstations/WorkstationConfigHost:WorkstationConfigHostÔ
ÏÈ
gceInstanceçBä:á
Ñ
workstations WorkstationConfigHostGceInstanceRgcp:workstations/WorkstationConfigHostGceInstance:WorkstationConfigHostGceInstanceJA runtime using a Compute Engine instance.
Structure is documented below.
:˛
Ñ
workstations WorkstationConfigHostGceInstanceRgcp:workstations/WorkstationConfigHostGceInstance:WorkstationConfigHostGceInstanceÙ
Òë
accelerators±BÆ*´:®
•
workstations+WorkstationConfigHostGceInstanceAcceleratorhgcp:workstations/WorkstationConfigHostGceInstanceAccelerator:WorkstationConfigHostGceInstanceAcceleratorMAn accelerator card attached to the instance.
Structure is documented below.
⁄
boostConfigs±BÆ*´:®
•
workstations+WorkstationConfigHostGceInstanceBoostConfighgcp:workstations/WorkstationConfigHostGceInstanceBoostConfig:WorkstationConfigHostGceInstanceBoostConfigïA list of the boost configurations that workstations created using this workstation configuration are allowed to use.
Structure is documented below.
5
bootDiskSizeGbB Size of the boot disk in GB.
÷
confidentialInstanceConfig‹BŸ:÷
”
workstations:WorkstationConfigHostGceInstanceConfidentialInstanceConfigÜgcp:workstations/WorkstationConfigHostGceInstanceConfidentialInstanceConfig:WorkstationConfigHostGceInstanceConfidentialInstanceConfigYA set of Compute Engine Confidential VM instance options.
Structure is documented below.
O
disablePublicIpAddressesB
 -Whether instances have no public IP address.
=

disableSshB
 )Whether to disable SSH access to the VM.
∞
enableNestedVirtualizationB
 ãWhether to enable nested virtualization on the Compute Engine VMs backing the Workstations.
See https://cloud.google.com/workstations/docs/reference/rest/v1beta/projects.locations.workstationClusters.workstationConfigs#GceInstance.FIELDS.enable_nested_virtualization
@
machineTypeB" +The name of a Compute Engine machine type.
N
poolSizeB <Number of instances to pool for faster workstation startup.
∑
serviceAccountB" ûEmail address of the service account that will be used on VM instances used to support this config. This service account must have permission to pull the specified container image. If not set, VMs will run without a service account, in which case the image must be publicly accessible.
Ö
serviceAccountScopesB*" ‰Scopes to grant to the service_account. Various scopes are automatically added based on feature usage. When specified, users of workstations under this configuration must have `iam.serviceAccounts.actAs` on the service account.
æ
shieldedInstanceConfigœBÃ:…
∆
workstations6WorkstationConfigHostGceInstanceShieldedInstanceConfig~gcp:workstations/WorkstationConfigHostGceInstanceShieldedInstanceConfig:WorkstationConfigHostGceInstanceShieldedInstanceConfigRA set of Compute Engine Shielded instance options.
Structure is documented below.
]
tagsB*" MNetwork tags to add to the Compute Engine machines backing the Workstations.
∂
vmTagsB2" £Resource manager tags to be bound to the VM instances backing the Workstations.
Tag keys and values have the same definition as
https://cloud.google.com/resource-manager/docs/tags/tags-overview
Keys must be in the format `tagKeys/{tag_key_id}`, and
values are in the format `tagValues/456`.
:⁄
•
workstations+WorkstationConfigHostGceInstanceAcceleratorhgcp:workstations/WorkstationConfigHostGceInstanceAccelerator:WorkstationConfigHostGceInstanceAcceleratorØ
¨B
count 5Number of accelerator cards exposed to the instance.
f
type" ZType of accelerator resource to attach to the instance, for example, "nvidia-tesla-p100".
:ÿ

•
workstations+WorkstationConfigHostGceInstanceBoostConfighgcp:workstations/WorkstationConfigHostGceInstanceBoostConfig:WorkstationConfigHostGceInstanceBoostConfig≠	
™	∏
accelerators“Bœ*Ã:…
∆
workstations6WorkstationConfigHostGceInstanceBoostConfigAccelerator~gcp:workstations/WorkstationConfigHostGceInstanceBoostConfigAccelerator:WorkstationConfigHostGceInstanceBoostConfigAcceleratorSAn accelerator card attached to the boost instance.
Structure is documented below.
q
bootDiskSizeGbB YSize of the boot disk in GB. The minimum boot disk size is `30` GB. Defaults to `50` GB.
¥
enableNestedVirtualizationB
 èWhether to enable nested virtualization on the Compute Engine VMs backing boosted Workstations.
See https://cloud.google.com/workstations/docs/reference/rest/v1beta/projects.locations.workstationClusters.workstationConfigs#GceInstance.FIELDS.enable_nested_virtualization
2
id" (The id to be used for the boost config.
Ω
machineTypeB" ßThe type of machine that boosted VM instances will use‚Äîfor example, e2-standard-4. For more information about machine types that Cloud Workstations supports, see the list of available machine types https://cloud.google.com/workstations/docs/available-machine-types. Defaults to e2-standard-4.
O
poolSizeB =Number of instances to pool for faster workstation boosting.
:˚
∆
workstations6WorkstationConfigHostGceInstanceBoostConfigAccelerator~gcp:workstations/WorkstationConfigHostGceInstanceBoostConfigAccelerator:WorkstationConfigHostGceInstanceBoostConfigAcceleratorØ
¨B
count 5Number of accelerator cards exposed to the instance.
f
type" ZType of accelerator resource to attach to the instance, for example, "nvidia-tesla-p100".
:∂
”
workstations:WorkstationConfigHostGceInstanceConfidentialInstanceConfigÜgcp:workstations/WorkstationConfigHostGceInstanceConfidentialInstanceConfig:WorkstationConfigHostGceInstanceConfidentialInstanceConfig^
\Z
enableConfidentialComputeB
 7Whether the instance has confidential compute enabled.
:∂
∆
workstations6WorkstationConfigHostGceInstanceShieldedInstanceConfig~gcp:workstations/WorkstationConfigHostGceInstanceShieldedInstanceConfig:WorkstationConfigHostGceInstanceShieldedInstanceConfigÍ
ÁZ
enableIntegrityMonitoringB
 7Whether the instance has integrity monitoring enabled.
H
enableSecureBootB
 .Whether the instance has Secure Boot enabled.
?

enableVtpmB
 +Whether the instance has the vTPM enabled.
:À
ê
workstations$WorkstationConfigIamBindingConditionZgcp:workstations/WorkstationConfigIamBindingCondition:WorkstationConfigIamBindingCondition6
4
descriptionB" 

expression" 
title" :»
ç
workstations#WorkstationConfigIamMemberConditionXgcp:workstations/WorkstationConfigIamMemberCondition:WorkstationConfigIamMemberCondition6
4
descriptionB" 

expression" 
title" :◊
ê
workstations$WorkstationConfigPersistentDirectoryZgcp:workstations/WorkstationConfigPersistentDirectory:WorkstationConfigPersistentDirectory¡
æÔ
gcePd®B•:¢
ü
workstations)WorkstationConfigPersistentDirectoryGcePddgcp:workstations/WorkstationConfigPersistentDirectoryGcePd:WorkstationConfigPersistentDirectoryGcePd∫A directory to persist across workstation sessions, backed by a Compute Engine regional persistent disk. Can only be updated if not empty during creation.
Structure is documented below.
J
	mountPathB" 7Location of this directory in the running workstation.
:◊	
ü
workstations)WorkstationConfigPersistentDirectoryGcePddgcp:workstations/WorkstationConfigPersistentDirectoryGcePd:WorkstationConfigPersistentDirectoryGcePd≤
ØH
diskTypeB" 6Type of the disk to use. Defaults to `"pd-standard"`.
≈
fsTypeB" ¥Type of file system that the disk should be formatted with. The workstation image must support this file system type. Must be empty if `sourceSnapshot` is set. Defaults to `ext4`.
Œ
reclaimPolicyB" ∂Whether the persistent disk should be deleted when the workstation is deleted. Valid values are `DELETE` and `RETAIN`. Defaults to `DELETE`.
Possible values are: `DELETE`, `RETAIN`.
π
sizeGbB ®The GB capacity of a persistent home directory for each workstation created with this configuration. Must be empty if `sourceSnapshot` is set.
Valid values are `10`, `50`, `100`, `200`, `500`, or `1000`. Defaults to `200`. If less than `200` GB, the `diskType` must be `pd-balanced` or `pd-ssd`.
ç
sourceSnapshotB" ÙName of the snapshot to use as the source for the disk.
Must be empty if `sourceImage` is set.
Must be empty if `read_only` is false.
Updating `source_snapshot` will update content in the ephemeral directory after the workstation is restarted.
:¯
Å
workstationsWorkstationConfigReadinessCheckPgcp:workstations/WorkstationConfigReadinessCheck:WorkstationConfigReadinessCheckr
p6
path" *Path to which the request should be sent.
6
port *Port to which the request should be sent.
:∏
~
workstationsWorkstationIamBindingConditionNgcp:workstations/WorkstationIamBindingCondition:WorkstationIamBindingCondition6
4
descriptionB" 

expression" 
title" :µ
{
workstationsWorkstationIamMemberConditionLgcp:workstations/WorkstationIamMemberCondition:WorkstationIamMemberCondition6
4
descriptionB" 

expression" 
title" 