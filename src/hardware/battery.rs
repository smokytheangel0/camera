/// This is where we poll the battery
/// and ram usage numbers, for adding
/// to the log queue, and for adding
/// to the ui queue

/// This enum describes which state
/// your main battery is in, this battery
/// is attached to the laptop internally
pub enum MainBattery {
    /// this indicates the battery has enough energy remaining
    HasEnergy(
        /// number of seconds the battery is predicted to last from average draw
        u64,
    ),
    /// this indicates the battery is in need of replacement
    NearEmpty(
        /// number of seconds the battery is predicted to last from average draw
        u64,
    ),
    /// this indicates the system will power down within five minutes
    Empty,
}

/// This enum describes which state your
/// secondary battery is in, this battery
/// is attached to the laptop with a 12v
/// DC adapter
pub enum SecondaryBattery {
    /// this indicates the battery has enough energy remaining
    HasEnergy(
        /// the number of seconds the battery is predicted to last from average draw
        u64,
    ),
    /// this indicates the battery is in need of replacement
    NearEmpty(
        /// number of secondsthe battery is predicted to last from average draw
        u64,
    ),
    /// this indicates the system will power down within five minutes
    Empty,
}
