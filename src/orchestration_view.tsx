import React from 'react';
import { createRoot } from 'react-dom/client';
import OrchestrationCanvas from './OrchestrationCanvas';

let orchestrationView: HTMLElement | null;
let reactFlowRoot: HTMLElement | null;

export function initOrchestrationView(repoPath: string | null) {
    orchestrationView = document.getElementById("orchestration-view");
    reactFlowRoot = document.getElementById('react-flow-root');

    if (reactFlowRoot) {
        const root = createRoot(reactFlowRoot);
        if (repoPath) {
            root.render(<OrchestrationCanvas repoPath={repoPath} />);
        } else {
            root.render(<div>Please select a repository in the settings tab to use the orchestration canvas.</div>);
        }
    }
}

export function showOrchestrationView() {
    if (orchestrationView) {
        orchestrationView.style.display = "block";
    }
}

export function hideOrchestrationView() {
    if (orchestrationView) {
        orchestrationView.style.display = "none";
    }
}