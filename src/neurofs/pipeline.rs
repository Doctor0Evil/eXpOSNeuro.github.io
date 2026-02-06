#[derive(Debug, Clone)]
pub struct Step {
    pub name: String,
    pub run: String,
}

#[derive(Debug, Clone)]
pub struct Job {
    pub id: String,
    pub runs_on: String,
    pub working_directory: Option<String>,
    pub steps: Vec<Step>,
}

#[derive(Debug, Clone)]
pub struct Pipeline {
    pub name: String,
    pub triggers: Vec<String>,
    pub jobs: Vec<Job>,
}

pub fn bootstrap_nexsm_pipeline() -> Pipeline {
    Pipeline {
        name: "Build & Deploy Bootstrap v5 NEXSM Dashboard".into(),
        triggers: vec![
            "push:main".into(),
            "pull_request:main".into(),
            "workflow_dispatch".into(),
        ],
        jobs: vec![
            Job {
                id: "build-bootstrap-ui".into(),
                runs_on: "ubuntu-latest".into(),
                working_directory: Some("frontend".into()),
                steps: vec![
                    Step { name: "Checkout source code".into(), run: "actions/checkout@v4".into() },
                    Step { name: "Set up Node.js".into(), run: "actions/setup-node@v4 node=20.x".into() },
                    Step { name: "Install dependencies".into(), run: "npm ci".into() },
                    Step { name: "Build Bootstrap v5 assets".into(), run: "npm run build".into() },
                    Step { name: "Verify built assets".into(), run: "ls -lh dist || ls -lh build || ls -lh ../public".into() },
                    Step { name: "Upload artifact".into(), run: "actions/upload-artifact@v4".into() },
                ],
            },
            Job {
                id: "deploy-gh-pages".into(),
                runs_on: "ubuntu-latest".into(),
                working_directory: None,
                steps: vec![
                    Step { name: "Download built artifact".into(), run: "actions/download-artifact@v4".into() },
                    Step { name: "Deploy to GitHub Pages".into(), run: "peaceiris/actions-gh-pages@v4".into() },
                ],
            },
        ],
    }
}
