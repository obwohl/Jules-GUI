import { Session } from "./models";

/**
 * Renders the list of sessions in a table format.
 * @param sessions - The list of sessions to render.
 */
export function renderSessionList(sessions: Session[]) {
  const sessionsList = document.querySelector<HTMLDivElement>("#sessions-list")!;

  if (sessions.length === 0) {
    sessionsList.innerHTML = "<p>No sessions found.</p>";
    return;
  }

  const table = document.createElement("table");
  table.className = "session-table";

  const thead = table.createTHead();
  const headerRow = thead.insertRow();
  const headers = ["Title", "Name", "State"];
  headers.forEach(headerText => {
    const th = document.createElement("th");
    th.textContent = headerText;
    headerRow.appendChild(th);
  });

  const tbody = table.createTBody();
  sessions.forEach(session => {
    const row = tbody.insertRow();
    const titleCell = row.insertCell();
    titleCell.textContent = session.title;

    const nameCell = row.insertCell();
    nameCell.textContent = session.name;

    const stateCell = row.insertCell();
    stateCell.textContent = session.state;
  });

  sessionsList.innerHTML = "";
  sessionsList.appendChild(table);
}
