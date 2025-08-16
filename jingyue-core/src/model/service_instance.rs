use std::{collections::HashMap, net::SocketAddr};
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceInstance {
    /// 实例唯一ID
    pub id: String,
    /// 服务名               
    pub service_name: String,     
     /// 分组名
    pub group_name: String,      
    /// IP+端口
    pub address: SocketAddr,      
    /// 元数据
    pub metadata: HashMap<String, String>, 
     /// 权重
    pub weight: f32,             
    /// 健康状态
    pub healthy: bool,            
     /// 是否临时实例
    pub ephemeral: bool,         
    /// 最后心跳时间
    pub last_beat: DateTime<Utc>, 
}