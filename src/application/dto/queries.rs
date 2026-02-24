/// Query to get a single project
#[derive(Debug, Clone)]
pub struct ProjectQuery {
    pub identifier: String, // Can be ID or name
}

/// Query to list projects
#[derive(Debug, Clone, Default)]
pub struct ListProjectsQuery {
    pub language: Option<String>,
    pub architecture: Option<String>,
    pub status: Option<String>,
}
