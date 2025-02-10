#[derive(Default, Debug)]
pub struct IndecisivePrevention {
    pub for_candidate_status_ids: Vec<i32>,
    pub if_shift_end_reason_ids: Vec<i32>,
    pub for_the_next_x_days: Option<i32>,
    pub enforce: Option<bool>,
}

#[derive(Default, Debug)]
pub struct SideJobPrevention {
    pub for_candidate_status_ids: Vec<i32>,
    pub role_type_id: Option<i32>,
    pub enforce: Option<bool>,
}

#[derive(Default, Debug)]
pub struct LastMinuteActionPreventionForBooking {
    pub for_candidate_status_ids: Vec<i32>,
    pub minutes: Option<i32>,
    pub enforce: Option<bool>,
}

#[derive(Default, Debug)]
pub struct LastMinuteActionPreventionForCanceling {
    pub for_candidate_status_ids: Vec<i32>,
    pub minutes: Option<i32>,
    pub enforce: Option<bool>,
}

#[derive(Default, Debug)]
pub struct ExhaustionPrevention {
    pub for_candidate_status_ids: Vec<i32>,
    pub for_shift_status_ids: Vec<i32>,
    pub hours_allowed: Option<i32>,
    pub within_x_hours: Option<i32>,
    pub enforce: Option<bool>,
}
