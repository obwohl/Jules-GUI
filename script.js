document.addEventListener('DOMContentLoaded', () => {
    const apiKeyInput = document.getElementById('api-key-input');
    const saveApiKeyButton = document.getElementById('save-api-key');
    const listSourcesButton = document.getElementById('list-sources');
    const sourcesList = document.getElementById('sources-list');
    const createSessionForm = document.getElementById('create-session-form');
    const sessionSourceSelect = document.getElementById('session-source');
    const listSessionsButton = document.getElementById('list-sessions');
    const sessionsList = document.getElementById('sessions-list');
    const sessionInfo = document.getElementById('session-info');
    const activityFeed = document.getElementById('activity-feed');
    const userMessageInput = document.getElementById('user-message');
    const sendMessageButton = document.getElementById('send-message');
    const approvePlanButton = document.getElementById('approve-plan');

    const JULES_API_BASE_URL = 'https://jules.googleapis.com/v1alpha';
    let apiKey = localStorage.getItem('jules-api-key');
    let currentSessionId = null;

    if (apiKey) {
        apiKeyInput.value = apiKey;
    }

    saveApiKeyButton.addEventListener('click', () => {
        apiKey = apiKeyInput.value;
        localStorage.setItem('jules-api-key', apiKey);
        alert('API Key saved!');
    });

    async function makeApiRequest(endpoint, method = 'GET', body = null) {
        if (!apiKey) {
            alert('Please enter your API key.');
            return;
        }

        const headers = {
            'Content-Type': 'application/json',
            'X-Goog-Api-Key': apiKey,
        };

        const options = {
            method,
            headers,
        };

        if (body) {
            options.body = JSON.stringify(body);
        }

        try {
            const response = await fetch(`${JULES_API_BASE_URL}/${endpoint}`, options);
            if (!response.ok) {
                const error = await response.json();
                throw new Error(error.error.message);
            }
            return response.json();
        } catch (error) {
            alert(`Error: ${error.message}`);
        }
    }

    listSourcesButton.addEventListener('click', async () => {
        const data = await makeApiRequest('sources');
        if (data && data.sources) {
            sourcesList.innerHTML = '';
            sessionSourceSelect.innerHTML = '';
            data.sources.forEach(source => {
                const listItem = document.createElement('li');
                listItem.textContent = source.name;
                sourcesList.appendChild(listItem);

                const optionItem = document.createElement('option');
                optionItem.value = source.name;
                optionItem.textContent = source.name;
                sessionSourceSelect.appendChild(optionItem);
            });
        }
    });

    createSessionForm.addEventListener('submit', async (event) => {
        event.preventDefault();
        const title = document.getElementById('session-title').value;
        const prompt = document.getElementById('session-prompt').value;
        const source = sessionSourceSelect.value;
        const branch = document.getElementById('session-branch').value;

        const body = {
            title,
            prompt,
            sourceContext: {
                source,
                githubRepoContext: {
                    startingBranch: branch,
                }
            }
        };

        const session = await makeApiRequest('sessions', 'POST', body);
        if (session) {
            alert(`Session created: ${session.name}`);
            listSessions();
        }
    });

    async function listSessions() {
        const data = await makeApiRequest('sessions');
        if (data && data.sessions) {
            sessionsList.innerHTML = '';
            data.sessions.forEach(session => {
                const listItem = document.createElement('li');
                listItem.textContent = `${session.title} (${session.name})`;
                listItem.addEventListener('click', () => {
                    loadSession(session.name);
                });
                sessionsList.appendChild(listItem);
            });
        }
    }

    listSessionsButton.addEventListener('click', listSessions);

    async function loadSession(sessionId) {
        currentSessionId = sessionId;
        const session = await makeApiRequest(sessionId);
        if(session) {
            sessionInfo.innerHTML = `
                <h3>${session.title}</h3>
                <p><strong>ID:</strong> ${session.name}</p>
                <p><strong>Prompt:</strong> ${session.prompt}</p>
            `;
            listActivities();
        }
    }

    async function listActivities() {
        if (!currentSessionId) return;
        const data = await makeApiRequest(`${currentSessionId}/activities`);
        if (data && data.activities) {
            activityFeed.innerHTML = '';
            data.activities.forEach(activity => {
                const listItem = document.createElement('li');
                let content = '';
                if(activity.message) {
                    content = `<b>${activity.message.role}:</b> ${activity.message.content}`;
                } else if (activity.plan) {
                    content = `<b>Plan:</b> ${activity.plan.reasoning}`;
                } else if (activity.progress) {
                    content = `<b>Progress:</b> ${activity.progress.message}`;
                }
                listItem.innerHTML = content;
                activityFeed.appendChild(listItem);
            });
        }
    }

    sendMessageButton.addEventListener('click', async () => {
        if (!currentSessionId) {
            alert('Please select a session first.');
            return;
        }
        const prompt = userMessageInput.value;
        await makeApiRequest(`${currentSessionId}:sendMessage`, 'POST', { prompt });
        userMessageInput.value = '';
        setTimeout(listActivities, 2000); // Give agent time to respond
    });

    approvePlanButton.addEventListener('click', async () => {
        if (!currentSessionId) {
            alert('Please select a session first.');
            return;
        }
        await makeApiRequest(`${currentSessionId}:approvePlan`, 'POST');
        alert('Plan approved!');
        setTimeout(listActivities, 2000);
    });

    setInterval(listActivities, 10000);
});