import React from 'react';
import { render, screen, waitFor } from '@testing-library/react';
import { describe, it, expect, vi } from 'vitest';
import DiffViewer from './DiffViewer';
import { invoke } from '@tauri-apps/api/core';

describe('DiffViewer', () => {
    it('should call invoke with get_diff', async () => {
        vi.mocked(invoke).mockResolvedValue('test diff');
        render(<DiffViewer repoPath="/app" />);
        await waitFor(() => {
            expect(invoke).toHaveBeenCalledWith('get_diff', { repoPath: '/app' });
        });
    });

    it('should display the diff', async () => {
        vi.mocked(invoke).mockResolvedValue('test diff');
        render(<DiffViewer repoPath="/app" />);
        expect(await screen.findByText('test diff')).toBeInTheDocument();
    });

    it('should display an error', async () => {
        vi.mocked(invoke).mockRejectedValue('test error');
        render(<DiffViewer repoPath="/app" />);
        expect(await screen.findByText('Error: test error')).toBeInTheDocument();
    });

    it('should display a loading message', async () => {
        vi.mocked(invoke).mockReturnValue(new Promise(() => {}));
        render(<DiffViewer repoPath="/app" />);
        expect(screen.getByText('Loading diff...')).toBeInTheDocument();
    });
});