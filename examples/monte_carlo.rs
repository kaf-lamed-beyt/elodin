use nalgebra::{vector, Vector3};
use paracosm::{
    forces::gravity,
    monte_carlo::{DistributionSpec, MonteCarlo, Normal},
    runtime::JobSpec,
    xpbd::{
        builder::{EntityBuilder, XpbdBuilder},
        runner::{IntoSimRunner, RunMode},
    },
    Force, Pos, Time,
};

fn main() {
    MonteCarlo::default()
        .var::<f64>(DistributionSpec::Normal(Normal::new(1.0, 0.2)))
        .job(job)
        .run()
}

fn job(thrust: f64) -> JobSpec {
    JobSpec::default().sim(
        (|b: XpbdBuilder<'_>| {
            sim(b, thrust);
        })
        .run_mode(RunMode::FixedTicks(100)),
    )
}

fn sim(mut builder: XpbdBuilder<'_>, thrust: f64) {
    builder.entity(
        EntityBuilder::default()
            .mass(1.0)
            .pos(vector![0.0, 0.0, 1.0])
            .vel(vector![1.0, 0.0, 0.0])
            .effector(gravity(1.0 / 6.649e-11, Vector3::zeros()))
            .effector(move |Time(t)| {
                if (9.42..10.0).contains(&t) {
                    Force(Vector3::new(0.0, -0.3, 0.5) * thrust)
                } else {
                    Force(Vector3::zeros())
                }
            })
            .trace(Vector3::zeros()),
    );
}
