use chrono::{NaiveDate, NaiveTime};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

use crate::models::paginate::{default_limit, default_page};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Workday {
    pub date: NaiveDate,
    pub fk_driver_id: Uuid,
    pub start_time: NaiveTime,
    pub end_time: Option<NaiveTime>,
    pub rest_time: NaiveTime,
    pub overnight_rest: bool,
}

#[derive(Debug, Deserialize)]
pub struct GetAllWorkdaysQuery {
    #[serde(default = "default_page")]
    pub page: u32,
    #[serde(default = "default_limit")]
    pub limit: u32,
    pub month: Option<i32>,
    pub year: Option<i32>
}

impl Default for GetAllWorkdaysQuery {
    fn default() -> Self {
        Self {
            page: default_page(),
            limit: default_limit(),
            month: None,
            year: None
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct GetAllWorkdaysByPeriodQuery {
    #[serde(default = "default_page")]
    pub page: u32,
    #[serde(default = "default_limit")]
    pub limit: u32,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
}

impl Default for GetAllWorkdaysByPeriodQuery {
    fn default() -> Self {
        Self {
            page: default_page(),
            limit: default_limit(),
            start_date: NaiveDate::from_ymd_opt(1970, 1, 1).unwrap(),
            end_date: NaiveDate::from_ymd_opt(1970, 1, 1).unwrap(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateWorkdayRequest {
    pub date: NaiveDate,
    pub fk_driver_id: Uuid,
    pub start_time: NaiveTime,
    pub end_time: Option<NaiveTime>,
    pub rest_time: NaiveTime,
    pub overnight_rest: bool,
}