import React from 'react';
import { createRoot } from 'react-dom/client';
import OrchestrationCanvas from './OrchestrationCanvas';

let orchestrationView: HTMLElement | null;

export function initOrchestrationView() {
    orchestrationView = document.getElementById("orchestration-view");
    const reactFlowRoot = document.getElementById('react-flow-root');

    if (reactFlowRoot) {
        const root = createRoot(reactFlowRoot);
        root.render(<OrchestrationCanvas />);
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