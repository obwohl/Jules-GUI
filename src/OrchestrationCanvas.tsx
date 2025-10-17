import React, { useState, useCallback, useMemo } from 'react';
import ReactFlow, {
  Node,
  Edge,
  addEdge,
  useNodesState,
  useEdgesState,
  Controls,
  Background,
  applyNodeChanges,
} from 'reactflow';
import 'reactflow/dist/style.css';
import { invoke } from '@tauri-apps/api/core';
import DiffViewer from './DiffViewer';
import CustomNode from './CustomNode';

const initialNodes: Node[] = [
  { id: '1', type: 'input', data: { label: 'Start' }, position: { x: 250, y: 5 } },
];

let id = 2;
const getNextId = () => `${id++}`;

interface OrchestrationCanvasProps {
  repoPath: string;
}

const OrchestrationCanvas: React.FC<OrchestrationCanvasProps> = ({ repoPath }) => {
  const [nodes, setNodes, onNodesChange] = useNodesState(initialNodes);
  const [edges, setEdges, onEdgesChange] = useEdgesState([]);
  const [showDiff, setShowDiff] = useState(false);

  const onConnect = useCallback(
    (params: Edge | any) => setEdges((eds) => addEdge(params, eds)),
    [setEdges],
  );

  const onNodesChange = (changes) => {
    setNodes((nds) => applyNodeChanges(changes, nds));
  };

  const onAddNode = (type: string) => {
    const newNode = {
      id: getNextId(),
      type: 'custom',
      data: { label: type, onChange: onNodesChange },
      position: {
        x: Math.random() * window.innerWidth - 100,
        y: Math.random() * window.innerHeight,
      },
    };
    setNodes((nds) => nds.concat(newNode));
  };

  const onSaveWorkflow = () => {
    const workflow = {
        nodes: nodes,
        edges: edges,
    };
    invoke('save_workflow', { workflow })
        .then(() => console.log('Workflow saved successfully'))
        .catch(console.error);
  };

  const onExecuteWorkflow = () => {
    const workflow = {
        nodes: nodes,
        edges: edges,
    };
    invoke('execute_workflow', { workflow, repoPath })
        .then(() => console.log('Workflow executed successfully'))
        .catch(console.error);
  };

  const onToggleDiff = () => {
    setShowDiff(prev => !prev);
  }

  const nodeTypes = useMemo(() => ({ custom: CustomNode }), []);

  return (
    <div style={{ height: '100%' }}>
      {showDiff && (
        <div style={{ position: 'absolute', top: 40, left: 10, zIndex: 5, backgroundColor: 'white', padding: 10, border: '1px solid black', maxHeight: '80vh', overflowY: 'auto' }}>
          <DiffViewer repoPath={repoPath} />
        </div>
      )}
      <ReactFlow
        nodes={nodes}
        edges={edges}
        onNodesChange={onNodesChange}
        onEdgesChange={onEdgesChange}
        onConnect={onConnect}
        nodeTypes={nodeTypes}
        fitView
      >
        <Controls />
        <Background />
      </ReactFlow>
      <div style={{ position: 'absolute', top: 10, left: 10, zIndex: 4 }}>
        <button onClick={() => onAddNode('Get Diff')}>Add Get Diff Node</button>
        <button onClick={() => onAddNode('Apply Patch')}>Add Apply Patch Node</button>
        <button onClick={() => onAddNode('Create Branch and Commit')}>Add Create Branch and Commit Node</button>
        <button onClick={onSaveWorkflow} style={{ marginLeft: 5 }}>Save Workflow</button>
        <button onClick={onExecuteWorkflow} style={{ marginLeft: 5 }}>Execute Workflow</button>
        <button onClick={onToggleDiff} style={{ marginLeft: 5 }}>Toggle Diff</button>
      </div>
    </div>
  );
};

export default OrchestrationCanvas;