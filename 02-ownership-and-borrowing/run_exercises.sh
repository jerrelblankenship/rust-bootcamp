#!/bin/bash
# Module 02 Exercise Runner
# Runs all exercises and project demonstrations for Module 02: Ownership and Borrowing

echo "🦀 Module 02: Ownership and Borrowing - Exercise Runner"
echo "======================================================"

# Set colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Function to run exercise
run_exercise() {
    local exercise=$1
    local name=$2
    
    echo -e "\n${BLUE}📝 Running $name${NC}"
    echo "----------------------------------------"
    
    if rustc "exercises/$exercise" -o "target/$exercise" 2>/dev/null; then
        echo -e "${GREEN}✅ Compilation successful${NC}"
        echo -e "${YELLOW}Output:${NC}"
        ./target/$exercise
        echo -e "${GREEN}✅ $name completed successfully${NC}"
    else
        echo -e "${RED}❌ Compilation failed for $exercise${NC}"
        rustc "exercises/$exercise" 2>&1
    fi
}

# Function to run project binary
run_project_binary() {
    local binary=$1
    local name=$2
    
    echo -e "\n${BLUE}🚀 Running $name${NC}"
    echo "----------------------------------------"
    
    cd project-memory-visualizer
    if cargo run --bin "$binary" --quiet; then
        echo -e "${GREEN}✅ $name completed successfully${NC}"
    else
        echo -e "${RED}❌ $name failed${NC}"
    fi
    cd ..
}

# Create target directory
mkdir -p target

echo -e "${YELLOW}Starting Module 02 exercises...${NC}\n"

# Run all exercises
run_exercise "ex01-ownership.rs" "Exercise 1: Ownership Basics"
run_exercise "ex02-borrowing.rs" "Exercise 2: Borrowing and References"
run_exercise "ex03-lifetimes.rs" "Exercise 3: Lifetimes"
run_exercise "ex04-smart-pointers.rs" "Exercise 4: Smart Pointers"
run_exercise "ex05-advanced-patterns.rs" "Exercise 5: Advanced Patterns"

echo -e "\n${YELLOW}Running project demonstrations...${NC}\n"

# Run project binaries
run_project_binary "ownership-demo" "Ownership Demo"
run_project_binary "borrowing-demo" "Borrowing Demo"
run_project_binary "memory-visualizer" "Memory Visualizer"

echo -e "\n${BLUE}🧪 Running tests...${NC}"
echo "----------------------------------------"

# Run exercise tests
echo -e "${YELLOW}Testing exercises:${NC}"
for exercise in exercises/ex*.rs; do
    if [[ -f "$exercise" ]]; then
        filename=$(basename "$exercise")
        exercise_name="${filename%.rs}"
        
        if rustc --test "$exercise" -o "target/test_$exercise_name" 2>/dev/null; then
            echo -e "${GREEN}✅ $filename tests passed${NC}"
            ./target/test_$exercise_name --quiet
        else
            echo -e "${RED}❌ $filename tests failed to compile${NC}"
        fi
    fi
done

# Run project tests
echo -e "\n${YELLOW}Testing project:${NC}"
cd project-memory-visualizer
if cargo test --quiet; then
    echo -e "${GREEN}✅ Project tests passed${NC}"
else
    echo -e "${RED}❌ Project tests failed${NC}"
fi
cd ..

# Cleanup
echo -e "\n${BLUE}🧹 Cleaning up...${NC}"
rm -rf target

echo -e "\n${GREEN}🎉 Module 02 exercise run completed!${NC}"
echo -e "${YELLOW}Key concepts covered:${NC}"
echo "• Ownership rules and move semantics"
echo "• Borrowing and reference types"
echo "• Lifetime annotations and management"
echo "• Smart pointers (Box, Rc, Arc, RefCell)"
echo "• Advanced ownership patterns"
echo "• Memory visualization and debugging"

echo -e "\n${BLUE}📚 Next steps:${NC}"
echo "• Review any failed exercises"
echo "• Experiment with the memory visualizer"
echo "• Try modifying the project code"
echo "• Move on to Module 03: Error Handling"
