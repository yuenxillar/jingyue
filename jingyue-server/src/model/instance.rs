use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct InstanceInfoBody {
    /// 命名空间Id，默认为 "public"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace_id: Option<String>,

    /// 分组名，默认为 "DEFAULT_GROUP"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,

    /// 服务名 (必填)
    pub service_name: String,

    /// IP地址 (必填)
    pub ip: String,

    /// 端口号 (必填)
    pub port: u32,

    /// 集群名称，默认为 "DEFAULT"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_name: Option<String>,

    /// 是否只查找健康实例，默认为 true
    #[serde(skip_serializing_if = "Option::is_none")]
    pub healthy: Option<bool>,

    /// 实例权重，默认为 1.0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<f64>,

    /// 是否可用，默认为 true
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    /// 实例元数据，JSON格式的字符串
    /// 使用 HashMap<String, String> 来表示键值对形式的元数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,

    /// 是否为临时实例 (通常由客户端SDK根据心跳机制设置)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ephemeral: Option<bool>,

    /// 是否为续约请求 (心跳)，默认为 false
    /// 注意：这个字段在注册实例时可能不常用，更多用于心跳接口
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heart_beat: Option<bool>,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CancelInstanceBody {
    /// 命名空间Id，默认为 "public"
    /// 标记为可选字段 (Option<T>)，并在序列化时如果为 None 则省略
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace_id: Option<String>,

    /// 分组名，默认为 "DEFAULT_GROUP"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,

    /// 服务名 (必填)
    pub service_name: String,

    /// IP地址 (必填)
    pub ip: String,

    /// 端口号 (必填)
    pub port: u32,

    /// 集群名称，默认为 "DEFAULT"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_name: Option<String>,

    /// 是否为临时实例 (通常由客户端SDK根据心跳机制设置)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ephemeral: Option<bool>,
}

#[derive(Debug, Clone)]
pub struct GetServiceInstanceListParams {
    /// 否	命名空间Id，默认为public
    pub namespace_id: Option<String>,

    /// 否	分组名，默认为DEFAULT_GROUP
    pub group_name: Option<String>,

    /// 是	服务名 必填
    pub service_name: String,

    /// 否	集群名称，默认为DEFAULT
    /// 非必填
    pub cluster_name: Option<String>,

    /// 否	是否只获取健康实例，默认为false
    /// 非必填
    pub healthy_only: Option<bool>,
}

impl From<HashMap<String, String>> for GetServiceInstanceListParams {
    fn from(value: HashMap<String, String>) -> Self {
        Self {
            namespace_id: value
                .get("namespaceId")
                .map(|s| Some(s.to_string()))
                .unwrap_or_default(),
            group_name: value
                .get("groupName")
                .map(|s| Some(s.to_string()))
                .unwrap_or_default(),
            service_name: value.get("serviceName").unwrap().to_string(),
            cluster_name: value
                .get("clusterName")
                .map(|s| Some(s.to_string()))
                .unwrap_or_default(),
            healthy_only: value
                .get("healthyOnly")
                .map(|s| Some(s.parse().unwrap()))
                .unwrap_or_default(),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InstanceItem {
    /// IP地址
    pub ip: String,
    /// 端口号
    pub port: u32,
    /// 实例权重
    pub weight: f32,
    /// 实例是否健康
    pub healthy: bool,
    /// 实例是否启用
    pub enabled: bool,
    /// 是否为临时实例
    pub ephemeral: bool,
    /// 集群名称
    pub cluster_name: String,
    /// 服务名 (格式通常为 groupName@@serviceName)
    pub service_name: String,
    /// 实例元数据
    #[serde(default)] // 如果 JSON 中没有 metadata 字段或为 null，则使用默认值 HashMap::new()
    pub metadata: HashMap<String, String>,
    /// IP删除超时时间 (毫秒)
    pub ip_delete_timeout: u32,
    /// 实例ID生成器类型
    pub instance_id_generator: String,
    /// 实例心跳间隔 (毫秒)
    pub instance_heart_beat_interval: u32,
    /// 实例心跳超时时间 (毫秒)
    pub instance_heart_beat_time_out: u32,
}
