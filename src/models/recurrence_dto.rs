use chrono::{DateTime, NaiveDate, Utc};

#[derive(Default)]
pub struct RecurrenceDto {
    pub name: String,
    pub description: String,
    pub amount: f64,
    pub circulating_date: NaiveDate,
}

impl RecurrenceDto {
    pub fn clear(&mut self) {
        self.name.clear();
        self.description.clear();
        self.amount = 0.00;
        self.circulating_date = Utc::now().date_naive();
    }

    pub fn get_recurrence_date_time(&self) -> DateTime<Utc> {
        DateTime::<Utc>::from_naive_utc_and_offset(
            self.circulating_date.and_hms_opt(0, 0, 0).unwrap(),
            Utc
        )
    }
}