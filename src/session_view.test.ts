import { renderSessionList } from './session_view';
import { describe, it, expect, beforeEach } from 'vitest';

describe('renderSessionList', () => {
  beforeEach(() => {
    document.body.innerHTML = '<div id="sessions-list"></div>';
  });

  it('should render a table with session data', () => {
    const sessions = [
      { title: 'Session 1', name: 'session1', state: 'Completed' },
      { title: 'Session 2', name: 'session2', state: 'In Progress' },
    ];
    renderSessionList(sessions);

    const table = document.querySelector('.session-table');
    expect(table).not.toBeNull();

    const rows = table!.querySelectorAll('tbody tr');
    expect(rows.length).toBe(2);

    const firstRowCells = rows[0].querySelectorAll('td');
    expect(firstRowCells[0].textContent).toBe('Session 1');
    expect(firstRowCells[1].textContent).toBe('session1');
    expect(firstRowCells[2].textContent).toBe('Completed');
  });

  it('should display a message when no sessions are provided', () => {
    renderSessionList([]);
    const sessionsList = document.querySelector('#sessions-list');
    expect(sessionsList!.innerHTML).toContain('No sessions found');
  });
});
