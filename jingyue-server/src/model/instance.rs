use std::{sync::Arc, time::Duration};

use jingyue_core::model::instance::{InstanceItemInfo, InstanceRequestBody};
use tokio::{sync::Mutex, time::Instant};

#[derive(Clone)]
pub struct Instance {
    pub info: InstanceItemInfo,
    pub time_window: Arc<Mutex<TimeWindow>>,
    /// 实例创建时间
    pub created_at: Instant,
}

#[derive(Clone)]
pub struct TimeWindow {
    /// 时间窗口，如 3s
    window: Duration,
    /// 最少心跳数，如 10     
    threshold: usize,
    /// 保留窗口内的心跳时间         
    history: Vec<Instant>,
}

impl Default for TimeWindow {
    fn default() -> Self {
        Self {
            window: std::time::Duration::from_secs(5),
            threshold: 1,
            history: Default::default(),
        }
    }
}

impl Instance {
    /// 记录一次心跳
    pub async fn record(&self) {
        let mut time_window = self.time_window.lock().await;
        let now = Instant::now();
        time_window.history.push(now);

        // 清理过期的心跳（只保留 window 时间内的）
        let cutoff = now - time_window.window;
        time_window.history.retain(|t| *t >= cutoff);
    }

    /// 检查是否健康：当前窗口内心跳数 >= threshold
    pub async fn is_healthy(&self) -> bool {
        let time_window = self.time_window.lock().await;
        let now = Instant::now();
        let cutoff = now - time_window.window;

        // 再次清理（防止 record 没被频繁调用）
        let valid_count = time_window.history.iter().filter(|t| **t >= cutoff).count();
        valid_count >= time_window.threshold
    }

    /// 获取当前统计信息（用于监控）
    // pub async fn stats(&self) -> HeartbeatStats {
    //     let guard = self.inner.lock().await;
    //     let now = Instant::now();
    //     let cutoff = now - guard.window;
    //     let count = guard.history.iter().filter(|t| **t >= cutoff).count();

    //     HeartbeatStats {
    //         window_ms: guard.window.as_millis(),
    //         required: guard.threshold,
    //         current: count,
    //         healthy: count >= guard.threshold,
    //     }
    // }

    /// 动态更新规则（运行时修改）
    pub async fn update(&self, window: Duration, threshold: usize) -> Result<(), &'static str> {
        if window <= Duration::from_millis(10) {
            return Err("Window too short");
        }
        if threshold == 0 || threshold > 1000 {
            return Err("Invalid threshold");
        }

        let mut time_window = self.time_window.lock().await;
        time_window.window = window;
        time_window.threshold = threshold;

        // 清理历史记录以适应新窗口
        let cutoff = Instant::now() - window;
        time_window.history.retain(|t| *t >= cutoff);

        Ok(())
    }
}

impl Into<Instance> for InstanceRequestBody {
    fn into(self) -> Instance {
        Instance {
            info: InstanceItemInfo {
                ip: self.ip,
                port: self.port,
                weight: self.weight,
                healthy: self.healthy,
                enabled: self.enabled,
                ephemeral: self.ephemeral.unwrap_or(false),
                cluster_name: self.cluster_name,
                service_name: self.service_name,
                metadata: self.metadata,
                ip_delete_timeout: Default::default(),
                instance_id_generator: String::from("default"),
                instance_heart_beat_interval: Default::default(),
                instance_heart_beat_time_out: Default::default(),
            },
            time_window: Arc::new(Mutex::new(Default::default())),
            created_at: Instant::now(),
        }
    }
}
