use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// 链接状态
#[derive(Debug, Clone, Deserialize,Serialize, PartialEq)]
pub enum LinkStatus{
    Unknown,
    Active,
    Timeout,
    Dead(u16),
    Redirect(String),
    Error(String),
}

/// 书签实体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bookmark {
    /// 书签ID
    pub id: String,

    /// 原始链接
    pub url: String,
    
    /// 标准化URL(去除跟踪参数等)
    pub url_normalized: String,

    /// 标题
    pub title: String,
    
    /// 描述/备注
    pub description: Option<String>,
    
    /// 文件夹路径
    pub path: Vec<String>,

    /// 标签
    pub tags: Vec<String>,

    /// 元数据
    pub meta: HashMap<String, String>,

    /// 创建时间(时间戳 Unix Timestamp)
    pub create_at: Option<String>,

    /// 检测状态
    pub status: LinkStatus,
}

impl Bookmark {
    pub fn new(url: String, title: String, add_date: Option<String>) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            url_normalized: url.clone(),
            url,
            title,
            description: None,
            path: Vec::new(),
            tags: Vec::new(),
            meta: HashMap::new(),
            create_at: add_date,
            status: LinkStatus::Unknown,
        }
    }
}