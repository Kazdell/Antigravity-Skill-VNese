---
trigger: always_on
---

# ON-DEMAND & SCOPED TESTING RULE (ZERO-WASTE)

1. **On-Demand Testing Only**:
   - **FORBIDDEN**: Do NOT automatically write unit tests or run full test suites (`cargo test`, `dotnet test`, `go test ./...`, `pytest`, `gradlew test`, or `npx.cmd tsc`) unless explicitly requested by the user.
   - Only the Lead agent navigates to the modified Backend service directory and runs the fast compile/typecheck command (`cargo check`, `dotnet build --no-restore`, `go build ./...`, `gradlew compileJava`, etc.) **once** at the very end when Backend files are modified. Subagents are strictly forbidden from running build/test commands.
2. **Strictly Scoped Execution (No Full-Project Test Runs)**:
   - When asked to test, navigate to the target service directory and run **ONLY** the target test module, class, or function using the test runner's filter flag (e.g., `cargo test <test_name>`, `dotnet test --filter <TestName>`, `go test -run <TestName>`, `pytest -k <test_name>`, `gradlew test --tests *TargetTestClass`).
   - **FORBIDDEN**: Running the entire project's test suite unless explicitly requested by the user.
3. **Realistic Mocks & Test-First Diagnosis**:
   - Mock realistic domain contracts (return proper domain errors/exceptions instead of unrealistic `null` where the contract forbids it). If a test fails after refactoring, fix the flawed test mock/input first—never add defensive clutter to production code just to appease an outdated test.
