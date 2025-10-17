use crate::{git_operations, models::Workflow};
use crate::models::{Node, Edge};
use std::collections::{HashMap, VecDeque};

pub fn execute_workflow(workflow: &Workflow, repo_path: &str) -> Result<(), String> {
    println!("Executing workflow with {} nodes and {} edges", workflow.nodes.len(), workflow.edges.len());

    let mut adj: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut in_degree: HashMap<&str, i32> = HashMap::new();

    for node in &workflow.nodes {
        adj.insert(&node.id, Vec::new());
        in_degree.insert(&node.id, 0);
    }

    for edge in &workflow.edges {
        adj.entry(&edge.source).or_default().push(&edge.target);
        *in_degree.entry(&edge.target).or_default() += 1;
    }

    let mut queue: VecDeque<&str> = VecDeque::new();
    for (node_id, &degree) in &in_degree {
        if degree == 0 {
            queue.push_back(node_id);
        }
    }

    while let Some(node_id) = queue.pop_front() {
        if let Some(node) = workflow.nodes.iter().find(|n| n.id == node_id) {
            execute_node(node, repo_path)?;
        }

        if let Some(neighbors) = adj.get(node_id) {
            for neighbor_id in neighbors {
                *in_degree.entry(neighbor_id).or_default() -= 1;
                if *in_degree.get(neighbor_id).unwrap_or(&0) == 0 {
                    queue.push_back(neighbor_id);
                }
            }
        }
    }

    Ok(())
}

fn execute_node(node: &Node, repo_path: &str) -> Result<(), String> {
    if let Some(label) = node.data.get("label").and_then(|v| v.as_str()) {
        match label {
            "Get Diff" => {
                let diff = git_operations::get_diff(repo_path)?;
                println!("Diff:\n{}", diff);
            }
            "Apply Patch" => {
                if let Some(patch) = node.data.get("patch").and_then(|v| v.as_str()) {
                    git_operations::apply_patch(repo_path, patch)?;
                    println!("Patch applied successfully.");
                } else {
                    return Err("Patch not found in node data.".to_string());
                }
            }
            "Create Branch and Commit" => {
                let branch_name = node.data.get("branchName").and_then(|v| v.as_str()).ok_or("Branch name not found in node data.")?;
                let commit_message = node.data.get("commitMessage").and_then(|v| v.as_str()).ok_or("Commit message not found in node data.")?;
                git_operations::create_branch_and_commit(repo_path, branch_name, commit_message)?;
                println!("Branch created and changes committed successfully.");
            }
            _ => {
                println!("Unknown or non-executable node type: {}", label);
            }
        }
    } else {
        println!("Node is missing a label");
    }
    Ok(())
}