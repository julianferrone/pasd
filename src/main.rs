use std::{borrow::BorrowMut, collections::HashMap};
use uuid::Uuid;

#[derive(Debug)]
struct Pool {
    parent_map: HashMap<Uuid, Uuid>,
    children_map: HashMap<Uuid, Vec<Uuid>>,
    themes: Vec<Theme>,
    objectives: Vec<Objective>,
    key_results: Vec<KeyResult>,
    initiatives: Vec<Initiative>,
    projects: Vec<Project>,
    tasks: Vec<Task>,
    measurements: Vec<Measurement>,
}

impl Pool {
    fn new() -> Self {
        let pool = Pool {
            parent_map: HashMap::new(),
            children_map: HashMap::new(),
            themes: vec![],
            objectives: vec![],
            key_results: vec![],
            initiatives: vec![],
            projects: vec![],
            tasks: vec![],
            measurements: vec![],
        };
        pool
    }

    fn add_theme(&mut self, title: impl Into<String>) -> Uuid {
        let theme = Theme::new(title);
        let theme_id = theme.id.clone();
        self.themes.push(theme);
        theme_id
    }

    fn add_objective(&mut self, title: impl Into<String>, theme_id: Uuid) -> Uuid {
        let objective = Objective::new(title);
        let objective_id = objective.id.clone();
        self.objectives.push(objective);
        self.parent_map.insert(objective_id, theme_id);
        self.children_map
            .entry(theme_id)
            .or_insert(Vec::new())
            .push(objective_id);
        objective_id
    }

    fn add_key_result(&mut self, title: impl Into<String>, objective_id: Uuid) -> Uuid {
        let key_result = KeyResult::new(title);
        let key_result_id = key_result.id.clone();
        self.key_results.push(key_result);
        self.parent_map.insert(key_result_id, objective_id);
        self.children_map
            .entry(objective_id)
            .or_insert(Vec::new())
            .push(key_result_id);
        key_result_id
    }

    fn add_initiative(&mut self, title: impl Into<String>, objective_id: Uuid) -> Uuid {
        let initiative = Initiative::new(title);
        let initiative_id = initiative.id.clone();
        self.initiatives.push(initiative);
        self.parent_map.insert(initiative_id, objective_id);
        self.children_map
            .entry(objective_id)
            .or_insert(Vec::new())
            .push(initiative_id);
        initiative_id
    }

    fn add_project(&mut self, title: impl Into<String>, objective_id: Uuid) -> Uuid {
        let project = Project::new(title);
        let project_id = project.id.clone();
        self.projects.push(project);
        self.parent_map.insert(project_id, objective_id);
        self.children_map
            .entry(objective_id)
            .or_insert(Vec::new())
            .push(project_id);
        project_id
    }

    fn add_measurement(&mut self, measurement: impl Into<String>, key_result_id: Uuid) -> Uuid {
        let measurement = Measurement::new(measurement);
        let measurement_id = measurement.id.clone();
        self.measurements.push(measurement);
        self.parent_map.insert(measurement_id, key_result_id);
        self.children_map
            .entry(key_result_id)
            .or_insert(Vec::new())
            .push(measurement_id);
        measurement_id
    }

    fn add_task(&mut self, title: impl Into<String>, project_id: Uuid) -> Uuid {
        let task = Task::new(title);
        let task_id = task.id.clone();
        self.tasks.push(task);
        self.parent_map.insert(task_id, project_id);
        self.children_map
            .entry(project_id)
            .or_insert(Vec::new())
            .push(task_id);
        self.auto_set_project(project_id);
        task_id
    }

    fn find_task_with_id(&mut self, task_id: Uuid) -> &mut Task {
        let index = self
            .tasks
            .iter()
            .position(|task| task.id == task_id)
            .unwrap();
        self.tasks.get_mut(index).unwrap()
    }

    fn unstart_task(&mut self, task_id: Uuid) {
        let mut task = self.find_task_with_id(task_id);
        task.status = Status::NotStarted;
        let project_id = self.parent_map.get(&task_id).unwrap().clone();
        self.auto_set_project(project_id);
    }

    fn start_task(&mut self, task_id: Uuid) {
        let mut task = self.find_task_with_id(task_id);
        task.status = Status::InProgress;
        let project_id = self.parent_map.get(&task_id).unwrap().clone();
        self.auto_set_project(project_id);
    }

    fn complete_task(&mut self, task_id: Uuid) {
        let mut task = self.find_task_with_id(task_id);
        task.status = Status::Completed;
        let project_id = self.parent_map.get(&task_id).unwrap().clone();
        self.auto_set_project(project_id);
    }

    fn find_project_with_id(&mut self, project_id: &Uuid) -> &mut Project {
        let index = self
            .projects
            .iter()
            .position(|project| project.id == *project_id)
            .unwrap();
        self.projects.get_mut(index).unwrap()
    }

    fn auto_set_project(&mut self, project_id: Uuid) {
        let task_ids = self.children_map.get(&project_id).unwrap().clone();
        let mut tasks = self.tasks.clone().into_iter().filter(|t| task_ids.contains(&t.id));
        let mut project = self.find_project_with_id(&project_id);

        if tasks.all(|t| t.status == Status::Completed) {
            project.status = Status::Completed
        } else if tasks.all(|t| t.status == Status::NotStarted) {
            project.status = Status::NotStarted
        } else {
            project.status = Status::InProgress
        }
    }
}

// TOKIP Hierarchy
// Theme
// stretch Objective (multiple to a Theme, one at a time (90 days))
// Key Results (2—4 to a Objective)
// Initiative (multiple recurring to an Objective)
// Project (multiple one-off to an Objective)
// Measurements (multiple to a key result)

#[derive(Debug)]
struct Theme {
    id: Uuid,
    title: String,
    objectives: Vec<Uuid>,
    status: Status,
}

impl Theme {
    fn new(title: impl Into<String>) -> Theme {
        Theme {
            id: Uuid::new_v4(),
            title: title.into(),
            objectives: vec![],
            status: Status::default(),
        }
    }
}

#[derive(Debug)]
struct Objective {
    id: Uuid,
    title: String,
    key_results: Vec<Uuid>,
    initiatives: Vec<Uuid>,
    projects: Vec<Uuid>,
}

impl Objective {
    fn new(title: impl Into<String>) -> Objective {
        Objective {
            id: Uuid::new_v4(),
            title: title.into(),
            key_results: vec![],
            initiatives: vec![],
            projects: vec![],
        }
    }
}

#[derive(Debug)]
struct KeyResult {
    id: Uuid,
    title: String,
    measurements: Vec<String>,
}

impl KeyResult {
    fn new(title: impl Into<String>) -> KeyResult {
        KeyResult {
            id: Uuid::new_v4(),
            title: title.into(),
            measurements: vec![],
        }
    }
}

#[derive(Debug)]
struct Initiative {
    id: Uuid,
    title: String,
    // explanation: String,
    status: Status,
}

impl Initiative {
    fn new(title: impl Into<String>) -> Initiative {
        Initiative {
            id: Uuid::new_v4(),
            title: title.into(),
            status: Status::default(),
        }
    }
}

#[derive(Debug)]
struct Project {
    id: Uuid,
    title: String,
    status: Status,
}

impl Project {
    fn new(title: impl Into<String>) -> Project {
        Project {
            id: Uuid::new_v4(),
            title: title.into(),
            status: Status::default(),
        }
    }
}

#[derive(Debug, Clone)]
struct Task {
    id: Uuid,
    title: String,
    status: Status,
}

impl Task {
    fn new(title: impl Into<String>) -> Task {
        Task {
            id: Uuid::new_v4(),
            title: title.into(),
            status: Status::default(),
        }
    }
}

#[derive(Debug)]
struct Measurement {
    id: Uuid,
    measurement: String,
}

impl Measurement {
    fn new(measurement: impl Into<String>) -> Measurement {
        Measurement {
            id: Uuid::new_v4(),
            measurement: measurement.into(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Status {
    NotStarted,
    InProgress,
    Completed,
}

impl Status {
    fn default() -> Status {
        Status::NotStarted
    }
}

fn main() {
    // println!("Hello, world!");
    let mut pool = Pool::new();
    let theme_1 = pool.add_theme("Physique");

    let obj_1 = pool.add_objective("Build a killer physique", theme_1);
    let ini_1 = pool.add_initiative("Go to the gym once per week", obj_1);
    let ini_2 = pool.add_initiative("Meal prep twice per week", obj_1);

    let kr_1 = pool.add_key_result("Body fat percentage of 12%", obj_1);
    let kr_2 = pool.add_key_result("Muscle mass of 40 kilograms", obj_1);

    let proj_1 = pool.add_project("Build gym routine", obj_1);
    let proj_2 = pool.add_project("Create diet plan", obj_1);

    let task_1 = pool.add_task("Read Body By Science", proj_1);
    let task_2 = pool.add_task("Schedule in gym time", proj_1);

    let task_3 = pool.add_task("Read the PE diet", proj_2);
    let task_4 = pool.add_task(
        "Calculate protein/energy ratio by cost for groceries",
        proj_2,
    );

    pool.add_measurement("20% body fat", kr_1);
    pool.add_measurement("18% body fat", kr_1);

    pool.start_task(task_1);
    pool.complete_task(task_3);
    pool.complete_task(task_4);



    println!("{:#?}", pool);
}

// TODO: convert into Rust web-server
// TODO: add cascading statuses
// -- if a task goes
