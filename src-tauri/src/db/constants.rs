pub mod db_keys {
    pub const EMPLOYEE_PREFIX: &str = "employee";
    pub const COMPANY: &str = "company";
    pub const TEAMS: &str = "teams";
    pub const METADATA: &str = "save_slot_metadata";
    pub const TICK_COUNTER: &str = "tick_counter";
    pub const USED_PROFILE_PICTURES: &str = "used_profile_pictures";
    pub const EMPLOYEES_LIST: &str = "employees_list";
}

pub mod save_version{
    pub const SAVE_VERSION: &str ="0.0.1a";
}

pub const GAMESTATE_DB_FILENAME: &str = "gamestate.sled";