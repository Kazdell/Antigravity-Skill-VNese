---
trigger: always_on
---

# SECURITY.MD - Security Guardrails

> **Goal**: Protect the system from common vulnerabilities and human errors.

---

## 1. FORBIDDEN ACTIONS

1. **Hardcode Secrets**:
   - Never write API Keys, Passwords, or Tokens directly in code.
   - Always use `process.env` or environment variables.
2. **Commit Tokens**:
   - Inspect `.gitignore` before committing.
   - Ensure `.env` is listed in `.gitignore`.
3. **Delete Database & Destructive Commands**:
   - STRICTLY FORBIDDEN to execute database dropping or destructive commands (e.g., `dotnet ef database drop`, `DROP DATABASE`, `DROP TABLE`, or deleting `.sqlite` files) without EXPLICIT user instruction and THREE confirmation steps.
   - Using the `--force` flag on production DB/migrations is strictly prohibited unless approved in writing by the Team Lead.

---

## 2. CODING STANDARDS

1. **SQL Injection**:
   - Always use Parameterized Queries or ORMs (Prisma, TypeORM, EF Core).
   - Never concatenate raw strings into SQL queries.
2. **XSS (Cross-Site Scripting)**:
   - Sanitize all user/API inputs.
   - Use libraries like `dompurify` when rendering HTML.
3. **Authentication**:
   - Always hash passwords (Bcrypt/Argon2).
4. **Null Pointer Exceptions (NPE) & Safe Checking**:
   - Always validate objects (`null`/`undefined`) before accessing properties or methods.
   - Prefer optional chaining (`?.`), nullish coalescing (`??`), or explicit `if` checks.
5. **Thread-Safety & Concurrency**:
   - Avoid shared mutable states across threads/async processes without proper locks or synchronization.
   - Ensure concurrent functions do not cause database/cache race conditions.
6. **Resource Cleanup & Leak Prevention**:
   - Always close/release resources (file streams, DB connections, event subscriptions) immediately after use using safe patterns like `try-finally`, `using` (C#), or cleanup functions in `useEffect` (React).

---

## 3. INCIDENT PROTOCOL

If a vulnerability is detected or a secret leak is suspected:
1. **STOP**: Halt all current tasks immediately.
2. **REPORT**: Alert the user immediately with a prominent warning (RED ALERT).
3. **MITIGATE**: Propose key rotation or patching solutions.
