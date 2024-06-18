use std::path::Path;

pub(crate) const MOON: &str = "moon";
pub(crate) const MOON_REPO: &str = "MOON";

#[derive(Debug, Clone, Default)]
pub(crate) struct Moon {
    pub(crate) projects: Vec<Project>,
}

impl Moon {
    pub(crate) fn new(projects: Vec<Project>) -> Self {
        Self { projects }
    }

    pub(crate) fn generate() -> Self {
        let moon = std::process::Command::new(MOON)
            .current_dir(Path::new(&std::env::var(MOON_REPO).unwrap()))
            .args(["query", "tasks"])
            .output()
            .expect("Failed to execute `moon query tasks`");

        let stdout = String::from_utf8(moon.stdout).expect("Failed to convert stdout to string");

        let mut projects: Vec<Project> = Vec::new();
        let mut project: Option<Box<Project>> = None;

        for line in stdout.lines() {
            if line.trim().is_empty() {
                continue;
            }

            if !line.starts_with('\t') {
                match project {
                    Some(ref mut p) => {
                        projects.push(*p.clone());
                        *p = Box::new(Project::new(line.trim().to_string(), vec![]));
                    }
                    _ => {
                        project = Some(Box::new(Project::new(line.trim().to_string(), vec![])));
                    }
                }
            } else if let Some(ref mut p) = project {
                p.tasks
                    .push(Task::new(p.project.clone(), line.trim().to_string()));
            }
        }

        Self::new(projects)
    }
}

#[derive(Debug, Clone, Default)]
pub(crate) struct Project {
    pub(crate) project: String,
    pub(crate) tasks: Vec<Task>,
}

impl Project {
    pub(crate) fn new(project: String, tasks: Vec<Task>) -> Self {
        Self { project, tasks }
    }
}

#[derive(Debug, Clone, Default)]
pub(crate) struct Task {
    pub(crate) task: String,
    pub(crate) command: String,
}

impl Task {
    pub(crate) fn new(project: String, task: String) -> Self {
        let task_name = task.split_once('|').unwrap().0.trim();
        let command = format!("{project}{task_name}");
        Self { task, command }
    }
}
