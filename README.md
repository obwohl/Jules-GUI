# Jules-GUI
A non shitty GUI for Jules 
Google for Developers
Search all articles...
Search

Google for Developers
Products
More
Solutions
Events
Learn
Community
More
Developer Program
Blog
Develop
Android
Chrome
ChromeOS
Cloud
Firebase
Flutter
Google Assistant
Google Maps Platform
Google Workspace
TensorFlow
YouTube
Grow
Firebase
Google Ads
Google Analytics
Google Play
Search
Web Push and Notification APIs
Earn
AdMob
Google Ads API
Google Pay
Google Play Billing
Interactive Media Ads

English
Level Up Your Dev Game: The Jules API is Here!
OCT. 3, 2025
Jane Fine
Senior Product Manager
Labs
Jenny Cong
Software Engineer
Labs

Share
Ready to help turbocharge your workflow? We’re excited to introduce the Jules API—a new way to automate, integrate, and innovate across the entire software development lifecycle.

Jules API Building Blocks
The Jules API is built around a few simple, powerful concepts:

Source: This is your input, like a GitHub repository. Just make sure you’ve installed the Jules GitHub app first.
Session: Think of this as kicking off a project with your asynchronous coding agent. It’s a continuous block of work, like a chat session, where all the magic happens.
Activity: These are the individual steps and events inside a session. From the agent generating a plan to you sending a message, every action is an activity.
Build the Future: What Will You Create?
This isn't just about simple automation; it's about creating your own "agents" to handle complex tasks. Imagine building a custom bot that can…

Fix Bugs from Slack: A user reports a bug in a Slack channel. Your bot picks it up, invokes the Jules API, and Jules gets to work. It analyzes the code, creates a fix, and runs tests to make sure nothing breaks. The whole time, it keeps you updated right in the original Slack thread. When it's done, it creates a PR for you to review and merge.
Automate Backlog Triage: Programmatically pull minor bugs or feature requests from your backlog and assign them directly to Jules.
Ready to Dive In? A 30-Second Quickstart
Let's get this party started. Fire up your terminal and give this a whirl.

Get your API key by going to https://jules.google.com/settings#api.
Screenshot 2025-10-02 at 8.35.12 PM
2. Find the source repo you want to work with. First, see what GitHub repos you have connected.

curl 'https://jules.googleapis.com/v1alpha/sources' \
    -H 'X-Goog-Api-Key: YOUR_API_KEY'
Shell

3. Kick off a session. Now, let's give Jules a task. How about creating a boba app?

curl 'https://jules.googleapis.com/v1alpha/sessions' \
    -X POST \
    -H "Content-Type: application/json" \
    -H 'X-Goog-Api-Key: YOUR_API_KEY' \
    -d '{
      "prompt": "Create a boba app!",
      "sourceContext": {
        "source": "sources/github/bobalover/boba",
        "githubRepoContext": {
          "startingBranch": "main"
        }
      },
      "title": "Boba App"
    }'
Shell

And there you have it! This is how easy it can be to work with Jules programmatically. You can find full documentation here.

Try the Jules API today
Again, this is an early version of the Jules API and while we have so many plans in store for where to take it next, we’re most excited to see what you will build with it!

So, stay tuned for more and join our Discord channel to tell us what you think and where we should take the API next.

posted in:
AI
Announcements
Learn
Related Posts
Gemini for Home: Expanding the Platform for a New Era of Smart Home AI
AI
Announcements
Gemini for Home: Expanding the Platform for a New Era of Smart Home AI

OCT. 1, 2025
Meet Jules Tools: A Command Line Companion for Google’s Async Coding Agent
AI
Announcements
Meet Jules Tools: A Command Line Companion for Google’s Async Coding Agent

OCT. 2, 2025
Unlocking Multi-Spectral Data with Gemini
AI
Case Studies
How-To Guides
Unlocking Multi-Spectral Data with Gemini

OCT. 1, 2025
Connect
Blog
Bluesky
Instagram
LinkedIn
X (Twitter)
YouTube
Programs
Google Developer Program
Google Developer Groups
Google Developer Experts
Accelerators
Women Techmakers
Google Cloud & NVIDIA
Developer consoles
Google API Console
Google Cloud Platform Console
Google Play Console
Firebase Console
Actions on Google Console
Cast SDK Developer Console
Chrome Web Store Dashboard
Google Home Developer Console
Google for Developers
Android
Chrome
Firebase
Google Cloud Platform
All products
Terms
Privacy

English
