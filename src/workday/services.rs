use sqlx::PgPool;
use uuid::Uuid;
use crate::{driver::models::Driver, errors::app_error::AppError, models::paginate::PaginateQuery, workday::models::{CreateWorkdayRequest, GetAllWorkdaysByPeriodQuery, GetAllWorkdaysQuery, Workday}};

pub struct WorkdayService {
    pool: PgPool,
}

impl WorkdayService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn get_all_workdays(&self, filters: &GetAllWorkdaysQuery) -> Result<(Vec<Workday>, u64), AppError> {
        let mut where_list_request = vec![];
        if filters.month.is_some() {
            where_list_request.push(format!("EXTRACT(MONTH FROM date) = {}", filters.month.unwrap()));
        }
        if filters.year.is_some() {
            where_list_request.push(format!("EXTRACT(YEAR FROM date) = {}", filters.year.unwrap()));
        }

        let mut where_clause = "".to_string();
        if filters.month.is_some() || filters.year.is_some() {
            where_clause = format!("WHERE {}", where_list_request.join(" AND "));
        }

        let total_count = sqlx::query_scalar::<_, i64>(&format!("SELECT COUNT(*) as count FROM workdays {}", where_clause))
                .fetch_one(&self.pool)
                .await? as u64;

        let query = format!(
            "SELECT fk_driver_id, date, start_time, end_time, rest_time, overnight_rest FROM workdays {} ORDER BY date DESC LIMIT $1 OFFSET $2",
            where_clause
        );

        let workdays = sqlx::query_as::<_, Workday>(&query)
            .bind(filters.limit as i64)
            .bind(((filters.page - 1) * filters.limit) as i64)
            .fetch_all(&self.pool)
            .await?;

        Ok((workdays, total_count))
    }

    pub async fn get_all_workdays_by_period(&self, filters: &GetAllWorkdaysByPeriodQuery) -> Result<(Vec<Workday>, u64), AppError> {
        let start_date = &filters.start_date;
        let end_date = &filters.end_date;

        let total_count = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) as count FROM workdays WHERE date >= $1 AND date <= $2")
            .bind(start_date)
            .bind(end_date)
            .fetch_one(&self.pool)
            .await? as u64;

        let workdays = sqlx::query_as!(
            Workday,
            r#"
            SELECT fk_driver_id, date, start_time, end_time, rest_time, overnight_rest
            FROM workdays
            WHERE date >= $1 AND date <= $2
            ORDER BY date ASC
            LIMIT $3
            OFFSET $4
            "#,
            start_date,
            end_date,
            filters.limit as i64,
            ((filters.page - 1) * filters.limit) as i64
        )
        .fetch_all(&self.pool)
        .await?;

        Ok((workdays, total_count))
    }

    pub async fn get_all_workdays_by_driver_id(&self, filters: &PaginateQuery, driver_id: Uuid) -> Result<(Vec<Workday>, u64), AppError> {
        let total_count = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) as count FROM workdays WHERE fk_driver_id = $1")
                .bind(driver_id)
                .fetch_one(&self.pool)
                .await? as u64;
        
        let workdays = sqlx::query_as!(
            Workday,
            r#"
            SELECT fk_driver_id, date, start_time, end_time, rest_time, overnight_rest
            FROM workdays
            WHERE fk_driver_id = $1
            ORDER BY date DESC
            LIMIT $2
            OFFSET $3
            "#,
            driver_id,
            filters.limit as i64,
            ((filters.page - 1) * filters.limit) as i64
        )
        .fetch_all(&self.pool)
        .await?;

        Ok((workdays, total_count))
    }

    pub async fn get_all_monthly_wordays_by_driver_id(&self, request: &GetAllWorkdaysQuery, driver_id: Uuid) -> Result<Vec<Workday>, AppError> {
        let mut where_list_request = vec![];

        if !request.month.is_none() {
            where_list_request.push(format!("EXTRACT(MONTH FROM date) = {}", request.month.unwrap()));
        }

        if !request.year.is_none() {
            where_list_request.push(format!("EXTRACT(YEAR FROM date) = {}", request.year.unwrap()));
        }

        let query_date = if where_list_request.is_empty() {
            "".to_string()
        } else {
            format!("AND {}", where_list_request.join(" AND "))
        };

        let mut query = String::from(
            "SELECT fk_driver_id, date, start_time, end_time, rest_time, overnight_rest FROM workdays WHERE fk_driver_id = $1"
        );
        if !query_date.is_empty() {
            query.push(' ');
            query.push_str(&query_date);
        }
        query.push_str(" ORDER BY date DESC");

        let workdays = sqlx::query_as::<_, Workday>(&query)
            .bind(driver_id)
            .fetch_all(&self.pool)
            .await?;

        Ok(workdays)
    }

    pub async fn create_workday(&self, create_req: &CreateWorkdayRequest) -> Result<Workday, AppError> {
        let driver_exists = sqlx::query_as!(
            Driver,
            "SELECT pk_driver_id, firstname, lastname, gender, email, phone_number, is_searchable, allow_request_professional_agreement, language, rest_json, mail_preferences, created_at, verified_at, last_login_at, deactivated_at FROM \"drivers\" WHERE pk_driver_id = $1",
            create_req.fk_driver_id
        )
        .fetch_optional(&self.pool)
        .await?
        .is_some();

        if !driver_exists {
            return Err(AppError::NotFound("Driver not found".to_string()));
        }

        let workday = sqlx::query_as!(
            Workday,
            r#"
            INSERT INTO workdays (fk_driver_id, date, start_time, end_time, rest_time, overnight_rest)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING fk_driver_id, date, start_time, end_time, rest_time, overnight_rest
            "#,
            create_req.fk_driver_id,
            create_req.date,
            create_req.start_time,
            create_req.end_time,
            create_req.rest_time,
            create_req.overnight_rest
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|_| AppError::Conflict("A workday for this driver already exists".to_string(), "WORKDAY_ALREADY_EXISTS".to_string()))?;

        Ok(workday)
    }
}