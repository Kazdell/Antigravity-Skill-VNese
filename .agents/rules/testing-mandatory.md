---
trigger: always_on
---
# MANDATORY TESTING RULE (ENFORCED)

> **Goal**: Ensure the highest quality of the system (Quality Assurance) via Test-Driven Development and Regression Avoidance.

1. **Test-Driven / Coverage Mandatory**: From now on, WHENEVER a new feature, API, or logic file is added, it is MANDATORY that the code is accompanied by or updated with corresponding Test Cases (Playwright E2E for Frontend UI or xUnit Integration/Unit Tests for Backend C#).
2. **No Deploy / Complete without Passing Tests**: No module can be marked as "Done" if tests fail or have not been executed.
3. **Follow Decision Tables**: All generated test cases must cover both the happy path and edge cases/alternative flows.
4. **Testcontainers Standard**: Backend tests must use Testcontainers to spin up virtual databases (based on SQL Edge/Postgres images) to isolate tests from the production/development databases. Direct testing against the application's real database is strictly prohibited.
5. **Mandatory AI Code Review**: Before finishing a task or merging a PR, it is MANDATORY to perform an internal AI Code Review (by running `/audit` or self-reviewing via the `open-code-review` skill). Critical security flaws, NPEs, or resource leaks found by the AI must be fixed and verified before executing final unit/E2E tests.
