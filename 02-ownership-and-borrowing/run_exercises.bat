@echo off
REM Module 02 Exercise Runner for Windows
REM Runs all exercises and project demonstrations for Module 02: Ownership and Borrowing

echo 🦀 Module 02: Ownership and Borrowing - Exercise Runner
echo ======================================================

REM Create target directory
if not exist target mkdir target

echo.
echo Starting Module 02 exercises...
echo.

REM Function to run exercise
call :run_exercise "ex01-ownership.rs" "Exercise 1: Ownership Basics"
call :run_exercise "ex02-borrowing.rs" "Exercise 2: Borrowing and References"
call :run_exercise "ex03-lifetimes.rs" "Exercise 3: Lifetimes"
call :run_exercise "ex04-smart-pointers.rs" "Exercise 4: Smart Pointers"
call :run_exercise "ex05-advanced-patterns.rs" "Exercise 5: Advanced Patterns"

echo.
echo Running project demonstrations...
echo.

REM Run project binaries
call :run_project_binary "ownership-demo" "Ownership Demo"
call :run_project_binary "borrowing-demo" "Borrowing Demo"
call :run_project_binary "memory-visualizer" "Memory Visualizer"

echo.
echo 🧪 Running tests...
echo ----------------------------------------

REM Run exercise tests
echo Testing exercises:
for %%f in (exercises\ex*.rs) do (
    echo Testing %%f...
    rustc --test "%%f" -o "target\test_%%~nf.exe" >nul 2>&1
    if errorlevel 1 (
        echo ❌ %%f tests failed to compile
    ) else (
        target\test_%%~nf.exe --quiet
        if errorlevel 1 (
            echo ❌ %%f tests failed
        ) else (
            echo ✅ %%f tests passed
        )
    )
)

REM Run project tests
echo.
echo Testing project:
cd project-memory-visualizer
cargo test --quiet >nul 2>&1
if errorlevel 1 (
    echo ❌ Project tests failed
) else (
    echo ✅ Project tests passed
)
cd ..

REM Cleanup
echo.
echo 🧹 Cleaning up...
if exist target rmdir /s /q target

echo.
echo 🎉 Module 02 exercise run completed!
echo Key concepts covered:
echo • Ownership rules and move semantics
echo • Borrowing and reference types
echo • Lifetime annotations and management
echo • Smart pointers (Box, Rc, Arc, RefCell)
echo • Advanced ownership patterns
echo • Memory visualization and debugging
echo.
echo 📚 Next steps:
echo • Review any failed exercises
echo • Experiment with the memory visualizer
echo • Try modifying the project code
echo • Move on to Module 03: Error Handling

goto :eof

:run_exercise
set exercise=%~1
set name=%~2

echo.
echo 📝 Running %name%
echo ----------------------------------------

rustc "exercises\%exercise%" -o "target\%exercise%.exe" >nul 2>&1
if errorlevel 1 (
    echo ❌ Compilation failed for %exercise%
    rustc "exercises\%exercise%"
) else (
    echo ✅ Compilation successful
    echo Output:
    target\%exercise%.exe
    echo ✅ %name% completed successfully
)
goto :eof

:run_project_binary
set binary=%~1
set name=%~2

echo.
echo 🚀 Running %name%
echo ----------------------------------------

cd project-memory-visualizer
cargo run --bin %binary% --quiet >nul 2>&1
if errorlevel 1 (
    echo ❌ %name% failed
) else (
    echo ✅ %name% completed successfully
)
cd ..
goto :eof
