use legion::{system, Resources, Schedule, World};
use spin_sleep::SpinSleeper;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};


struct TickCounter(AtomicU32);
fn main() {
    let mut world = World::default();
    let mut resources = Resources::default();
    resources.insert(Arc::new(TickCounter(AtomicU32::new(0))));


    // Startup schedule, runs once on startup. add run once systems here.
    let mut startup = Schedule::builder()
        .add_system(load_master_data_system())
        .flush()
        .add_system(generate_entities_system())
        .build();

    // main sim
    let mut sim_schedule = Schedule::builder() // Main game loop, add systems that runs per frame here.
        .add_system(increase_sim_tick_system())
        .build();


    let sleeper = SpinSleeper::new(0).with_spin_strategy(spin_sleep::SpinStrategy::YieldThread); // prevents CPU burn

    //Tick the startup schedule
    startup.execute(&mut world, &mut resources);
    loop {
        let tick_start = Instant::now();
        let tick_duration= Duration::from_millis(100);
        sim_schedule.execute(&mut world, &mut resources);
        let elapsed = tick_start.elapsed();
        if elapsed < tick_duration {
            sleeper.sleep(tick_duration - elapsed);
        } else {
            eprintln!(
                "Tick lag: {:?}",
                elapsed - tick_duration
            );
            // Don’t sleep again — just loop immediately to catch up
        }
    }
}

#[system]
fn increase_sim_tick(#[resource] tick_counter: &mut Arc<TickCounter>) {
    println!("{}", tick_counter.0.fetch_add(1, Ordering::Relaxed));
}

#[system]
fn generate_entities(){
    todo!()
}
#[system]
fn load_master_data() {
    todo!()
}

