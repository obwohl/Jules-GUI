import React, { useCallback } from 'react';
import { Handle, Position } from 'reactflow';

const CustomNode = ({ data }) => {
  const onDataChange = useCallback((evt) => {
    const { name, value } = evt.target;
    data[name] = value;
  }, [data]);


  const renderInputs = () => {
    switch (data.label) {
      case 'Apply Patch':
        return (
          <div>
            <label>Patch:</label>
            <textarea name="patch" onChange={onDataChange} />
          </div>
        );
      case 'Create Branch and Commit':
        return (
          <div>
            <div>
              <label>Branch Name:</label>
              <input type="text" name="branchName" onChange={onDataChange} />
            </div>
            <div>
              <label>Commit Message:</label>
              <textarea name="commitMessage" onChange={onDataChange} />
            </div>
          </div>
        );
      default:
        return null;
    }
  };

  return (
    <div style={{ border: '1px solid #777', padding: 10 }}>
      <Handle type="target" position={Position.Top} />
      <div>{data.label}</div>
      {renderInputs()}
      <Handle type="source" position={Position.Bottom} />
    </div>
  );
};

export default CustomNode;