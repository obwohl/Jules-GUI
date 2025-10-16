import { Session } from "./models";

/**
 * Renders the list of sessions in a table format.
 * @param sessions - The list of sessions to render.
 */
export function renderSessionList(sessions: Session[]) {
  const sessionsList = document.querySelector<HTMLDivElement>("#sessions-list");

  if (!sessionsList) {
    return;
  }

  // Clear any existing content
  sessionsList.innerHTML = "";

  if (sessions.length === 0) {
    const p = document.createElement("p");
    p.textContent = "No sessions found.";
    sessionsList.appendChild(p);
    return;
  }

  const table = document.createElement("table");
  table.className = "session-table";

  const columns = [
    { header: "Title", key: "title" },
    { header: "Name", key: "name" },
    { header: "State", key: "state" },
  ];

  const thead = table.createTHead();
  const headerRow = thead.insertRow();
  columns.forEach(({ header }) => {
    const th = document.createElement("th");
    th.textContent = header;
    headerRow.appendChild(th);
  });

  const tbody = table.createTBody();
  sessions.forEach(session => {
    const row = tbody.insertRow();
    columns.forEach(({ key }) => {
      const cell = row.insertCell();
      cell.textContent = session[key as keyof Session];
    });
  });

  sessionsList.appendChild(table);
}
