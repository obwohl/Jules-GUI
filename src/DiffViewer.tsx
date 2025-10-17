import React, { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';

interface DiffViewerProps {
    repoPath: string;
}

const DiffViewer: React.FC<DiffViewerProps> = ({ repoPath }) => {
    const [diff, setDiff] = useState<string | null>(null);
    const [error, setError] = useState<string | null>(null);

    useEffect(() => {
        invoke<string>('get_diff', { repoPath })
            .then(setDiff)
            .catch(err => setError(err.toString()));
    }, [repoPath]);

    if (error) {
        return <pre style={{ color: 'red' }}>Error: {error}</pre>;
    }

    if (diff === null) {
        return <div>Loading diff...</div>;
    }

    return (
        <pre style={{ whiteSpace: 'pre-wrap', fontFamily: 'monospace' }}>
            {diff}
        </pre>
    );
};

export default DiffViewer;