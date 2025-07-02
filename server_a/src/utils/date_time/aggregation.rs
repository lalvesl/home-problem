use chrono::{Datelike, NaiveDateTime, Timelike};
use strum::{EnumCount, EnumIter};

#[allow(unused)]
#[derive(
    Copy, Clone, EnumCount, EnumIter, Eq, PartialEq, Debug, PartialOrd, Ord,
)]
pub enum TimeAggregation {
    Years,
    Months,
    WeeksOfYear,
    WeeksOfMonth,
    DaysOfYear,
    DaysOfMonth,
    DaysOfWeek,
    Hours,
}

#[allow(unused)]
impl TimeAggregation {
    // pub fn index(&self) -> usize {
    //     *self as usize
    // }

    #[inline(always)]
    pub fn size_of_aggregation(&self) -> u64 {
        match self {
            TimeAggregation::Years => 365 * 24 * 60 * 60, // Approximate, no leap year
            TimeAggregation::Months => 30 * 24 * 60 * 60, // Approximate
            TimeAggregation::WeeksOfYear | TimeAggregation::WeeksOfMonth => {
                7 * 24 * 60 * 60
            }
            TimeAggregation::DaysOfYear
            | TimeAggregation::DaysOfMonth
            | TimeAggregation::DaysOfWeek => 24 * 60 * 60,
            TimeAggregation::Hours => 60 * 60,
        }
    }

    #[inline(always)]
    pub fn get(&self, datetime: NaiveDateTime) -> u16 {
        match self {
            TimeAggregation::Years => datetime.year() as u16,
            TimeAggregation::Months => datetime.month() as u16,
            TimeAggregation::WeeksOfYear => datetime.iso_week().week() as u16,
            TimeAggregation::WeeksOfMonth => {
                let first_day = datetime.with_day(1).unwrap();
                let first_week = first_day.iso_week().week();
                let current_week = datetime.iso_week().week();
                (current_week - first_week + 1) as u16
            }
            TimeAggregation::DaysOfYear => datetime.ordinal() as u16,
            TimeAggregation::DaysOfMonth => datetime.day() as u16,
            TimeAggregation::DaysOfWeek => {
                datetime.weekday().num_days_from_sunday() as u16
            } // 0 = Sunday
            TimeAggregation::Hours => datetime.hour() as u16,
        }
    }
}

#[allow(unused)]
pub enum LabelFormat {
    Short,
    Long,
}
// Translator someday......... this implementation will use a decent translation module and be replaced by tags, someday
#[allow(unused)]
impl TimeAggregation {
    #[inline(always)]
    pub fn label(&self, value: u16, format: LabelFormat) -> String {
        match self {
            TimeAggregation::Years => format!("{}", value),
            TimeAggregation::Months => match (value, format) {
                (1, LabelFormat::Short) => "Jan".into(),
                (1, LabelFormat::Long) => "Janeiro".into(),
                (2, LabelFormat::Short) => "Fev".into(),
                (2, LabelFormat::Long) => "Fevereiro".into(),
                (3, LabelFormat::Short) => "Mar".into(),
                (3, LabelFormat::Long) => "Março".into(),
                (4, LabelFormat::Short) => "Abr".into(),
                (4, LabelFormat::Long) => "Abril".into(),
                (5, LabelFormat::Short) => "Mai".into(),
                (5, LabelFormat::Long) => "Maio".into(),
                (6, LabelFormat::Short) => "Jun".into(),
                (6, LabelFormat::Long) => "Junho".into(),
                (7, LabelFormat::Short) => "Jul".into(),
                (7, LabelFormat::Long) => "Julho".into(),
                (8, LabelFormat::Short) => "Ago".into(),
                (8, LabelFormat::Long) => "Agosto".into(),
                (9, LabelFormat::Short) => "Set".into(),
                (9, LabelFormat::Long) => "Setembro".into(),
                (10, LabelFormat::Short) => "Out".into(),
                (10, LabelFormat::Long) => "Outubro".into(),
                (11, LabelFormat::Short) => "Nov".into(),
                (11, LabelFormat::Long) => "Novembro".into(),
                (12, LabelFormat::Short) => "Dez".into(),
                (12, LabelFormat::Long) => "Dezembro".into(),
                _ => format!("Mês {}", value),
            },
            TimeAggregation::DaysOfWeek => match (value, format) {
                (0, LabelFormat::Short) => "Dom".into(),
                (0, LabelFormat::Long) => "Domingo".into(),
                (1, LabelFormat::Short) => "Seg".into(),
                (1, LabelFormat::Long) => "Segunda-feira".into(),
                (2, LabelFormat::Short) => "Ter".into(),
                (2, LabelFormat::Long) => "Terça-feira".into(),
                (3, LabelFormat::Short) => "Qua".into(),
                (3, LabelFormat::Long) => "Quarta-feira".into(),
                (4, LabelFormat::Short) => "Qui".into(),
                (4, LabelFormat::Long) => "Quinta-feira".into(),
                (5, LabelFormat::Short) => "Sex".into(),
                (5, LabelFormat::Long) => "Sexta-feira".into(),
                (6, LabelFormat::Short) => "Sáb".into(),
                (6, LabelFormat::Long) => "Sábado".into(),
                _ => format!("Dia {}", value),
            },
            TimeAggregation::WeeksOfYear | TimeAggregation::WeeksOfMonth => {
                match format {
                    LabelFormat::Short => format!("Sem {}", value),
                    LabelFormat::Long => format!("Semana {}", value),
                }
            }
            TimeAggregation::DaysOfYear => format!("Dia {}", value),
            TimeAggregation::DaysOfMonth => format!("{}", value),
            TimeAggregation::Hours => match format {
                LabelFormat::Short => format!("{value} h"),
                LabelFormat::Long => format!("{value} hr"),
            },
        }
    }
}
