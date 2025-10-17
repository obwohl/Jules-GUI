import React, { useState } from 'react';
import ReactFlow, { Node, Edge } from 'reactflow';
import 'reactflow/dist/style.css';

const initialNodes: Node[] = [
  { id: '1', position: { x: 0, y: 0 }, data: { label: 'Hello' } },
  { id: '2', position: { x: 0, y: 100 }, data: { label: 'World' } },
];
const initialEdges: Edge[] = [{ id: 'e1-2', source: '1', target: '2' }];

function OrchestrationCanvas() {
  const [nodes, setNodes] = useState<Node[]>(initialNodes);
  const [edges, setEdges] = useState<Edge[]>(initialEdges);

  return (
    <ReactFlow
      nodes={nodes}
      edges={edges}
    />
  );
}

export default OrchestrationCanvas;