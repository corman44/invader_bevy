use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PeoplePlugin)
        .run();
}

pub struct PeoplePlugin;

impl Plugin for PeoplePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup)
            .add_systems(Update, (print_names, person_does_job));
    }
}

pub fn setup(mut commands: Commands) {
    commands.spawn((
        Person {
        name: "Cory".to_string(),
        },
        Employed {
            job: Job::Doctor,
        }
    ));
    commands.spawn((
        Person {
        name: "Deshawn".to_string(),
        },
        Employed {
            job: Job::Engineer,
        }
    ));
    commands.spawn(Person {
        name: "David".to_string(),
    });
}

#[derive(Component)]
pub struct Person {
    pub name: String
}

#[derive(Component)]
pub struct Name(String);

#[derive(Component)]
pub struct Employed {
    pub job: Job
}

pub enum Job {
    Doctor,
    FireFighter,
    Lawyer,
    Engineer,
    Nurse
}

pub fn print_names(person_query: Query<&Person>) {
    for person in person_query.iter() {
        println!("Name: {}", person.name);
    }
}

pub fn people_with_jobs(person_query: Query<&Person, With<Employed>>) {
    for person in person_query.iter() {
        println!("{} has a job",person.name);
    }
}

pub fn person_does_job( person_query: Query<(&Person, &Employed)> ) {
    for (person, employed) in person_query.iter() {
        let job_name = match employed.job {
            Job::Doctor => "Doctor",
            Job::FireFighter => "FireFighter",
            Job::Lawyer => "Lawyer",
            Job::Engineer => "Engineer",
            Job::Nurse => "Nurse",
        };
        println!("{} is a {}.", person.name, job_name);
    }
}
