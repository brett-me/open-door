---------------------------------------------------------------------------------------------------
Cycle 1: Rust Fundamentals & First Application (Weeks 1-12 Focus - Iterative)
---------------------------------------------------------------------------------------------------

Goal1: Get the absolute essentials solid enough to not hinder early Rust/backend work.

Goal2: Internalize Rust's core unique features (ownership!) and build something runnable. Overlap
Rust syntax with basic application structure.

Track 1: Absolute essentials (Git, PostgreSQL, HTTP)
---------------------------------------------------------------------------------------------------
1. Git: Focus on workflow – branching (git checkout -b feature), committing (git commit -m
"Meaningful message"), pushing (git push), merging (git merge / git pull). Understand .gitignore.
Leave complex rebasing/stashing for later when you need it. Resource: Pro Git Ch 1-3, Learn Git
Branching. Action: Use Git for everything from day one, even simple code snippets.

2. SQL (PostgreSQL): Focus on SELECT (with WHERE, basic JOIN), INSERT, UPDATE, DELETE. Understand
primary/foreign keys conceptually. Resource: SQLZoo, PostgreSQL Tutorial (basic CRUD).
Action: Install PostgreSQL, use psql or DBeaver to run basic commands. Design a simple schema.


3. HTTP Basics: Understand Request/Response cycle, Methods (GET/POST), Status Codes, Headers
Resource: MDN HTTP Basics. Action: Use browser dev tools (Network tab) actively.
Use curl to make simple GET requests.

Track 2: Core Rust Language (Continuous Focus):
---------------------------------------------------------------------------------------------------
1. Syntax, variables, types, functions, control flow.

2. OWNERSHIP, BORROWING, LIFETIMES: This is non-negotiable and requires dedicated, repeated effort.
Don't expect to "get it" and move on; you'll revisit this constantly.

3. Structs, Enums, match.

4. Error Handling (Result, Option, ?).

5. Collections (Vec, HashMap, String).

Resources: The Book (Ch 1-10 are crucial), Rustlings (mandatory practice).

Track 3: Simple Application & Tooling (Starts around Week 6-7):
---------------------------------------------------------------------------------------------------
1. Build Command-Line Tools: Apply Rust concepts immediately. Ideas: a file word counter, a simple
calculator parsing arguments, a tool to interact with a public JSON API (like weather).

2. Basic Testing: Write #[test] functions for your logic.
Tooling: Get comfortable with cargo clippy (address warnings!) and rustfmt. Use rust-analyzer in
your IDE.

3. Introduce serde early for handling JSON if your CLI tool interacts with APIs.

Resource: The Book (Ch 11-12 for testing/CLI project), serde docs.

This phase is long because mastering ownership takes time. Building CLIs forces you to handle I/O,
errors, and structure code early. Starting serde here prepares for web APIs. Expect frustration
with the borrow checker; persistence is key. Re-read chapters, work through Rustlings exercises
repeatedly.

---------------------------------------------------------------------------------------------------
Cycle 2: Web Backend Basics & Async Introduction (Weeks 10-20 Focus - Iterative)
---------------------------------------------------------------------------------------------------

Goal: Build a basic web API and start grappling with asynchronous Rust. Overlap web framework
learning with async concepts.

Track 1: Web Framework (Axum Recommended):
---------------------------------------------------------------------------------------------------
1. Routing, handlers, extracting data (path, query, JSON body via serde).

2. Returning responses (JSON, status codes).

3. Basic error handling in web context (mapping internal errors to HTTP responses).

4. Framework-specific state sharing (e.g., axum::extract::State).

Resource: Axum documentation & examples.

Track 2: Asynchronous Rust (Tokio):
---------------------------------------------------------------------------------------------------
1. async/await syntax.

2. Understanding the concept of non-blocking I/O.

3. Tokio basics: runtime, tasks (tokio::spawn).

Resource: Tokio Tutorial (Mini-Redis is excellent), Async Book (online).

Track 3: First Web Project:
---------------------------------------------------------------------------------------------------
1. Build the In-Memory TODO API. Start simple: create/list todos.

2. Apply Axum concepts. Handlers will need to be async fn.
Use tokio::sync::Mutex to protect shared in-memory state. This forces engagement with async-aware
synchronization.

Action: Get this simple API running locally. Use curl or tools like Postman/Insomnia to interact
with it. Introduce async because web servers require it. Using tokio::sync::Mutex early provides a
practical reason to learn async synchronization. Keep the API extremely simple initially to focus
on framework mechanics and async basics.

---------------------------------------------------------------------------------------------------
Cycle 3: Database Integration & API Enhancement (Weeks 18-28 Focus - Iterative)
---------------------------------------------------------------------------------------------------

Goal: Persist data using a database and handle more realistic API scenarios.
Overlap database driver usage with refining async understanding.

Track 1: Database Driver (SQLx Recommended):
---------------------------------------------------------------------------------------------------
1. Connecting to PostgreSQL.

2. Connection Pooling (sqlx::PgPool).

3. Executing queries (CRUD operations). Use parameterized queries!

4. Mapping results to Rust structs (using sqlx::FromRow).

5. Basic transactions.

6. Migrations (sqlx-cli). Do this early.

7. Resource: SQLx documentation & examples.

Track 2: Enhance API Project:
---------------------------------------------------------------------------------------------------
1. Replace in-memory store with PostgreSQL using SQLx.

2. Refactor handlers to be async and interact with the DB pool.

3. Improve error handling: Define custom error types (thiserror crate is useful) and map them
appropriately to HTTP responses in Axum.

4. Implement basic input validation (e.g., ensuring task descriptions aren't empty).

5. Write integration tests that hit your API endpoints (using crates like reqwest or hyper). These
might need a test database setup.

Action: Your TODO API should now persist data across restarts.

Track 3: Deepen Async/Error Handling:
---------------------------------------------------------------------------------------------------
1. Revisit Tokio concepts. Understand Send and Sync traits in async context.

2. Explore error handling libraries (anyhow, thiserror) for cleaner code.

Resource: Revisit Tokio/Async docs, library docs.

---------------------------------------------------------------------------------------------------
Cycle 4: Authentication, Containerization & Production Readiness (Weeks 26-36+ Focus - Iterative)
---------------------------------------------------------------------------------------------------

Goal: Secure the API, package it for deployment, and learn essential operational concerns.

Track 1: Authentication & Authorization:
---------------------------------------------------------------------------------------------------
1. Password Hashing (argon2 or bcrypt crates). Never store plain text passwords.

2. JWT generation and validation (jsonwebtoken crate). Understand claims.

3. Implement user registration and login endpoints.

4. Create Axum middleware or extractors to protect routes based on JWT.

Resource: Crate documentations, relevant blog posts/tutorials on JWT auth in Axum.

Track 2: Containerization (Docker):
---------------------------------------------------------------------------------------------------
1. Write a Dockerfile for your Rust app (use multi-stage builds).

2. Write docker-compose.yml to run your app and PostgreSQL together.

Resource: Docker documentation, tutorials on Dockerizing Rust apps.
Action: Run your entire application stack locally using docker-compose up.

Track 3: Observability & Configuration:
---------------------------------------------------------------------------------------------------
1. Structured Logging (tracing crate). Set up basic logging middleware.

2. Configuration (config crate or similar) to manage settings (DB URL, JWT secret) via
files/env vars.

3. Basic Health Check endpoint.

Track 4: Refine API & Testing:
---------------------------------------------------------------------------------------------------
1. Add more features to your API (or start a new, slightly more complex one).

2. Refine integration tests to cover auth flows.

3. Ensure robust error handling for all edge cases.

4. Focus on getting one auth method (JWT) working well. Docker is essential for modern deployment.
tracing is the de facto standard for logging. This cycle builds towards something deployable and
maintainable.

--------------------------------------------------------------------------------------------------------------------------------
Cycle 5: Portfolio Projects & Job Prep (Ongoing from Week ~30+)
--------------------------------------------------------------------------------------------------------------------------------

Goal: Build independent, showcase-worthy projects and prepare for the job market.
Activities:

1. Project Building: Build 1-2 distinct backend projects from scratch applying all learned concepts.
Choose something slightly more complex that interests you. Document them well (README, API docs).
Deploy them (Fly.io, Render).

2. Code Quality & Idiomatic Rust: Focus on clean code, SOLID principles (where applicable),
idiomatic Rust patterns. Seek code reviews (online communities, forums).

3. Revisit Fundamentals: Go back and solidify understanding of DS&A in Rust. Practice LeetCode-style
problems in Rust. Understand performance implications of choices (e.g., String vs &str, Vec cloning).

4. System Design Lite: Understand core concepts (load balancing, caching basics, message queues
purpose, monolith vs microservice tradeoffs). Don't aim for expert level, aim for conceptual
understanding.

5. CI/CD Basics: Implement a simple GitHub Actions workflow to build/test/lint your portfolio
projects automatically.

6. Portfolio projects are non-negotiable. They prove you can integrate skills. Continuous learning
is vital. Job prep is a skill in itself.

The "Why": Constantly ask why things are done a certain way in Rust (borrowing, async models,
error handling). Understanding the rationale prevents superficial learning.

Feedback Loop: Actively seek feedback. Post small code snippets on the Rust Discord or forums asking
for reviews ("How could this be more idiomatic?"). Explain your thought process when asking questions.
Depth First: Prioritize deep understanding of Core Rust, Async, your chosen Web Framework, and
Database interaction over trying to learn every adjacent technology initially. Get the foundation
rock solid.
