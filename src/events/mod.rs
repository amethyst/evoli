// event producers and consumers will both depend on the same event enumeration
// there may be multiple producers; there may be multiple consumers
// that is, there is no clear owner
// thus, events are defined in this directory independent of their producers and consumers

pub mod day_night_cycle;
