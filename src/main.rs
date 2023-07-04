
#[derive(Debug)]
struct Theme<'a> {
    title: String,
    objectives: Vec<Objective<'a>>,
}

impl Theme<'static> {
    fn new(title: String) -> Theme<'static> {
        Theme {
            title: String::from(title),
            objectives: Vec::new(),
        }
    }

    fn add_objective(&mut self, title: String) {
        let obj = Objective::new(title);
        self.objectives.push(obj)
    }
}

#[derive(Debug)]
struct Objective<'a> {
    title: String,
    key_results: Vec<KeyResult>,
    initiatives: Vec<Box<Initiative>>,
    projects: Vec<Project<'a>>,
}

impl Objective<'static> {
    fn new(title: String) -> Objective<'static> {
        Objective {
            title: String::from(title),
            key_results: Vec::new(),
            initiatives: Vec::new(),
            projects: Vec::new(),
        }
    }

    fn add_key_result(&mut self, title: String) {
        self.key_results.push(KeyResult::new(title))
    }

    fn add_initiative(&mut self, initiative: Initiative) {
        self.initiatives.push(Box::new(initiative))
    }

    fn add_project(&mut self, title: String) {
        self.projects.push(Project::new(title))
    }
}

#[derive(Debug)]
struct KeyResult {
    title: String,
    measurements: Vec<String>,
}

impl KeyResult {
    fn new(title: String) -> KeyResult {
        KeyResult {
            title: String::from(title),
            measurements: Vec::new(),
        }
    }

    fn add_measurement(&mut self, measurement: String) {
        self.measurements.push(measurement);
    }
}

#[derive(Debug)]
struct Initiative {
    title: String,
    // explanation: String,
    status: Status,
}

impl Initiative {
    fn new(title: String) -> Initiative {
    // fn new(title: String, explanation: String) -> Initiative {
        Initiative {
            title: String::from(title),
            // explanation: String::from(explanation),
            status: Status::NotStarted
        }
    }

    fn unstart(&mut self) {
        self.status = Status::NotStarted
    }

    fn start(&mut self) {
        self.status = Status::InProgress
    }

    fn complete(&mut self) {
        self.status = Status::Completed
    }
}

#[derive(Debug)]
struct Project<'a> {
    title: String,
    tasks: Vec<Task<'a>>,
    status: Status
}

impl Project<'static> {
    fn new(title: String) -> Project<'static> {
        Project {
            title: String::from(title),
            tasks: Vec::new(),
            status: Status::NotStarted
        }
    }

    fn add_task(self, title: String) {
        let task = Task::new(title, &self);
        self.tasks.push(task);
    }

    fn unstart(&mut self) {
        self.status = Status::NotStarted
    }

    fn start(&mut self) {
        self.status = Status::InProgress
    }

    fn complete(&mut self) {
        self.status = Status::Completed
    }

    fn check_if_subtasks_complete(mut self) {
        let completed_count: usize = self.tasks.iter()
            .filter(|t| t.status == Status::Completed)
            .count();
        let not_started_count: usize = self.tasks.iter()
            .filter(|t| t.status == Status::NotStarted)
            .count();
        if completed_count == self.tasks.len() {
            self.status = Status::Completed
        } else if not_started_count == self.tasks.len() {
            self.status = Status::NotStarted
        } else {
            self.status = Status::InProgress
        }
    }
}

#[derive(Debug)]
struct Task<'a> {
    title: String,
    status: Status,
    parent: &'a Project<'a>
}

impl <'a>Task<'a> {
    fn new(title: String, parent: &'a Project<'a>) -> Task<'a> {
        Task {
            title: String::from(title),
            status: Status::NotStarted,
            parent: parent
        }
    }

    fn unstart(&mut self) {
        self.status = Status::NotStarted;
        // self.parent.check_if_subtasks_complete();
    }

    fn start(&mut self) {
        self.status = Status::InProgress;
        // self.parent.check_if_subtasks_complete();
    }

    fn complete(&mut self) {
        self.status = Status::Completed;
        // self.parent.check_if_subtasks_complete();
    }
}


#[derive(Debug, PartialEq, Eq, Hash)]
enum Status {
    NotStarted,
    InProgress,
    Completed,
}

fn main() {
    println!("Hello, world!");
    let mut theme = Theme::new(String::from("Physique"));
    theme.add_objective(String::from("Build a killer physique"));
    
    let i_1 = Initiative::new(String::from("Go to the gym once per week"));
    let i_2 = Initiative::new(String::from("Meal prep twice per week"));
    theme.objectives[0].add_initiative(i_1);
    theme.objectives[0].add_initiative(i_2);
    theme.objectives[0].add_key_result(String::from("Body fat percentage of 12%"));
    theme.objectives[0].add_key_result(String::from("Muscle mass of 40 kilograms"));
    theme.objectives[0].add_project(String::from("Determine gym routine"));
    theme.objectives[0].projects[0].add_task(String::from("Read Body By Science"));
    theme.objectives[0].projects[0].add_task(String::from("Schedule in gym time"));
    theme.objectives[0].add_project(String::from("Create diet plan"));
    theme.objectives[0].projects[1].add_task(String::from("Read the PE diet"));
    theme.objectives[0].projects[1].add_task(String::from("Calculate protein/energy ratio by cost for groceries"));
    theme.objectives[0].projects[1].tasks[0].start();
    theme.objectives[0].projects[1].start();
    theme.objectives[0].projects[1].tasks[1].complete();
    theme.objectives[0].initiatives[0].start();
    theme.objectives[0].key_results[0].add_measurement(String::from("20% body fat"));
    theme.objectives[0].key_results[0].add_measurement(String::from("18% body fat"));
    println!("{:#?}", theme);
}

// TODO: convert into Rust web-server
// TODO: add cascading statuses
// -- if a task goes 

// TOKIP Hierarchy
// Theme
// stretch Objective (multiple to a Theme, one at a time (90 days))
// Key Results (2—4 to a Objective)
// Initiative (multiple recurring to an Objective)
// Project (multiple one-off to an Objective)
